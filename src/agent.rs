use azure_data_tables::{prelude::*};
use langchain_rust::embedding::{Embedder, FastEmbed};
use serde::{Deserialize, Serialize};
use langchain_rust::llm::OpenAIConfig;
use langchain_rust::{language_models::llm::LLM, llm::openai::OpenAI};
use crate::azure_table::{get_all_vectors, FormattedVectorEntity};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct VectorEntity {
    #[serde(rename = "PartitionKey")]
    pub category: String,
    #[serde(rename = "RowKey")]
    pub id: String,
    pub timestamp: Option<String>,
    pub vector: String,
    pub content: Option<String>,
}

pub struct Agent {
    embedder: FastEmbed,
    stored_vectors: Vec<FormattedVectorEntity>,
    openai_api_key: String
}

impl Agent {
    pub async fn new(table_client: &TableClient, openai_api_key: String) -> Result<Self, Box<dyn std::error::Error>> {
        let embedder = FastEmbed::try_new()?;
        let stored_vectors = get_all_vectors(table_client).await?;
        Ok(Self {
            embedder,
            stored_vectors,
            openai_api_key
        })
    }


    // Traitement d'un message
    pub async fn process_message(&self, message: &str) -> Option<Vec<String>> {
        // Encoder le message
        let query_embedding = self
            .embedder
            .embed_query(&*message.to_string())
            .await
            .ok()?;

        // Trouver les k meilleures correspondances
        let matches = self.find_top_k_matches(&query_embedding, 5);

        if matches.is_empty() {
            None
        } else {
            Some(matches)
        }
    }

    // Calculer la similarité entre deux vecteurs
    fn cosine_similarity(v1: &[f32], v2: &[f32]) -> f32 {
        let dot_product: f32 = v1.iter().zip(v2.iter()).map(|(a, b)| a * b).sum();
        let norm_v1: f32 = v1.iter().map(|x| x * x).sum::<f32>().sqrt();
        let norm_v2: f32 = v2.iter().map(|x| x * x).sum::<f32>().sqrt();
        dot_product / (norm_v1 * norm_v2)
    }

    // Trouver la correspondance la plus proche
    fn find_top_k_matches(&self, query_embedding: &Vec<f64>, k: usize) -> Vec<String> {
        let query_embedding_f32: Vec<f32> = query_embedding.iter().map(|&x| x as f32).collect();
        let mut matches: Vec<(&FormattedVectorEntity, f32)> = Vec::new();

        for entity in &self.stored_vectors {
            let similarity = Self::cosine_similarity(&entity.vector, &query_embedding_f32);
            matches.push((entity, similarity));
        }

        // Trier les correspondances par similarité décroissante
        matches.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Prendre les K meilleures correspondances et retourner leurs contenus
        matches.into_iter()
            .take(k)
            .map(|(entity, _)| entity.content.clone())
            .collect()
    }

    async fn query_openai(api_key: &str, context: &str, message: &str) -> Result<String, Box<dyn std::error::Error>> {
        let open_ai = OpenAI::default().with_config(
            OpenAIConfig::default()
                .with_api_key(api_key),

        );
        let system_prompt = "You are a helpful assistant.";

        let response = open_ai.invoke("hola").await.unwrap();
        Ok(response)
    }
}
