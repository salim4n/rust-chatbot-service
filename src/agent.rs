use langchain_rust::{chain::{builder::ConversationalChainBuilder, Chain}, fmt_message, fmt_template, llm::openai::{OpenAI, OpenAIModel}, memory::SimpleMemory, message_formatter, template_fstring};
use langchain_rust::prompt::HumanMessagePromptTemplate;
use langchain_rust::schemas::{ Message};
use crate::azure_table::FormattedVectorEntity;

pub const SYSTEM_PROMPT: &str = r#"Tu es l'assistant officiel d'IgnitionAI, une agence spécialisée en intelligence artificielle.

EXPERTISE:
...

CONSIGNES:
1. Base tes réponses principalement sur les documents fournis dans le contexte
2. Utilise l'historique de la conversation pour maintenir la cohérence
3. Reste factuel et technique quand nécessaire
...

INFORMATIONS DISPONIBLES:
Contexte (vecteurs d'information): {context}
Historique de la conversation: {history}
Vecteur de la question actuelle: {user_vector}

QUESTION: {question}

RÉPONSE:"#;

pub async fn initialize_chain() -> impl Chain {
    let llm = OpenAI::default().with_model(OpenAIModel::Gpt4oMini);
    let memory = SimpleMemory::new();

    let chain = ConversationalChainBuilder::new()
        .llm(llm)
        .prompt(message_formatter![
            fmt_message!(Message::new_system_message(SYSTEM_PROMPT)),
            fmt_template!(HumanMessagePromptTemplate::new(
            template_fstring!("

Vous êtes  un assistant expert en intelligence artificielle et assistant conseiller commercial sur le site ignitionai.fr
conversation:
{history}
Human: {input}
Context: {context}
User Vector: {user_vector}
AI:
",
            "input", "history", "context", "user_vector")))
        ])
        .memory(memory.into())
        .build()
        .expect("Error building ConversationalChain");
    chain
}


// Calculer la similarité entre deux vecteurs
fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let norm_v1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_v2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (norm_v1 * norm_v2)
}

// Trouver la correspondance la plus proche
fn find_closest_match(vectors: Vec<FormattedVectorEntity>, query_vector: Vec<f32>, top_k: usize) -> Vec<FormattedVectorEntity> {
    let mut closest_matches = Vec::new();

    for entity in vectors {
        let similarity = cosine_similarity(&query_vector, &entity.vector);
        closest_matches.push((entity, similarity));
    }

    closest_matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    closest_matches.iter().map(|(entity, _)| entity.clone()).take(top_k).collect()
}

pub fn find_relevant_documents(vectors: &[FormattedVectorEntity], user_vector: &Vec<f64>) -> Vec<FormattedVectorEntity> {
    // Convertir user_vector en Vec<f32> si nécessaire
    let user_vector_vec: Vec<f32> = user_vector.iter().map(|x| *x as f32).collect();

    // Utiliser find_closest_match pour trouver les documents les plus pertinents
    // Ajustez le nombre (5 dans cet exemple) selon vos besoins
    find_closest_match(vectors.to_vec(), user_vector_vec, 5)
}

