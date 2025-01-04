use azure_data_tables::{prelude::*, operations::QueryEntityResponse};
use futures::stream::StreamExt;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Serialize)]
pub struct FormattedVectorEntity {
    pub id: String,
    pub category: String,
    pub timestamp: String,
    pub vector: Vec<f32>,
    pub content: String,
}

pub async fn get_all_vectors(
    table_client: &TableClient,
) -> azure_core::Result<Vec<FormattedVectorEntity>> {
    let mut formatted_entities = Vec::new();
    let mut stream = table_client.query().into_stream::<VectorEntity>();

    while let Some(response) = stream.next().await {
        let QueryEntityResponse { entities, .. } = response?;

        for entity in entities {
            let vector: Vec<f32> = entity
                .vector
                .split(',')
                .filter_map(|v| v.parse::<f32>().ok())
                .collect();

            formatted_entities.push(FormattedVectorEntity {
                id: entity.id,
                category: entity.category,
                timestamp: entity.timestamp.unwrap_or_else(|| "unknown".to_string()),
                vector,
                content: entity.content.unwrap_or_else(|| "N/A".to_string()),
            });
        }
    }

    Ok(formatted_entities)
}
