use std::sync::Arc;
use langchain_rust::{chain::{builder::ConversationalChainBuilder, Chain}, fmt_message, fmt_template, llm::openai::{OpenAI, OpenAIModel}, memory::SimpleMemory, message_formatter, prompt_args, template_fstring};
use langchain_rust::agent::{AgentExecutor, ConversationalAgent, ConversationalAgentBuilder};
use langchain_rust::chain::options::ChainCallOptions;
use langchain_rust::embedding::{Embedder, FastEmbed};
use langchain_rust::prompt::{HumanMessagePromptTemplate, PromptTemplate, TemplateFormat};
use langchain_rust::schemas::{BaseMemory, Message};
use langchain_rust::tools::CommandExecutor;
use ndarray::prelude::*;
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
pub async fn initialize_agent() -> AgentExecutor<ConversationalAgent> {
    let llm = OpenAI::default().with_model(OpenAIModel::Gpt4oMini);
    let memory = SimpleMemory::new();
    let command_executor = CommandExecutor::default();
    let agent = ConversationalAgentBuilder::new()
        .tools(&[Arc::new(command_executor)])
        .options(ChainCallOptions::new().with_max_tokens(6000))
        .build(llm)
        .unwrap();

    let executor = AgentExecutor::from_agent(agent).with_memory(memory.into());

    executor
}
pub async fn vectorize_text(text: &str) -> Vec<f64> {
    let fastembed: FastEmbed = FastEmbed::try_new().unwrap();
    let embeddings = fastembed
        .embed_query(&text)
        .await
        .unwrap();

    println!("Len: {}", embeddings.len());
    println!("Embeddings: {:?}", embeddings);
    embeddings
}

// Calculer la similarité entre deux vecteurs
fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
    let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
    let norm_v1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_v2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();
    dot_product / (norm_v1 * norm_v2)
}

// Trouver la correspondance la plus proche
fn find_closest_match(vectors: Vec<FormattedVectorEntity>, query_vector: Vec<f32>, top_K: usize) -> Vec<FormattedVectorEntity> {
    let mut closest_matches = Vec::new();

    for entity in vectors {
        let similarity = cosine_similarity(&query_vector, &entity.vector);
        closest_matches.push((entity, similarity));
    }

    closest_matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    closest_matches.iter().map(|(entity, _)| entity.clone()).take(top_K).collect()
}

pub fn find_relevant_documents(vectors: &[FormattedVectorEntity], user_vector: &Vec<f64>) -> Vec<FormattedVectorEntity> {
    // Convertir user_vector en Vec<f32> si nécessaire
    let user_vector_vec: Vec<f32> = user_vector.iter().map(|x| *x as f32).collect();

    // Utiliser find_closest_match pour trouver les documents les plus pertinents
    // Ajustez le nombre (5 dans cet exemple) selon vos besoins
    find_closest_match(vectors.to_vec(), user_vector_vec, 5)
}

