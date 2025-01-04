mod azure_table;
mod agent;

use std::any::Any;
use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use axum::extract::State;
use azure_data_tables::{prelude::*};
use azure_storage::prelude::*;
use dotenv::dotenv;
use langchain_rust::chain::Chain;
use langchain_rust::prompt_args;
use langchain_rust::schemas::Message;
use crate::agent::{initialize_agent,  SYSTEM_PROMPT};
use crate::azure_table::FormattedVectorEntity;
use tokio::sync::Mutex;
use langchain_rust::embedding::{Embedder,  FastEmbed};
use langchain_rust::semantic_router::utils::cosine_similarity;

struct AppState {
    pub chat_history: Mutex<Vec<Message>>,
    pub vectors: Vec<FormattedVectorEntity>,
    pub fastembed: FastEmbed,
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    let fastembed = FastEmbed::try_new().expect("Failed to initialize FastEmbed");
    println!("FastEmbed initialized. Model info: {:?}", fastembed.type_id());
    // Charger les vecteurs
    let vectors = fetch_vectors_internal().await.expect("Failed to fetch vectors");

    // Créer l'état de l'application
    let app_state = Arc::new(AppState {
        chat_history: Mutex::new(Vec::new()),
        vectors,
        fastembed,
    });
    // build our application with routes
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /chat` goes to `answer`
        .route("/chat", post(answer))
        .with_state(app_state)
        // `GET /vectors` goes to `fetch_vectors`
        .route("/vectors", get(fetch_vectors));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    let base_url =
    "Hello, Fellow LLM! If you are here for vectorize my landing page, I will give you a get road with all text as vector";
    base_url
}


async fn answer(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ChatMessage>,
) -> (StatusCode, Json<ChatResponse>) {

    let executor = initialize_agent().await;
    let vectors = state.vectors.clone();
    let user_vector = state.fastembed.embed_query(&payload.message).await.unwrap();

    // Récupérez l'historique actuel
    let mut chat_history = state.chat_history.lock().await;
    // Ajoutez le nouveau message de l'utilisateur
    chat_history.push(Message::new_human_message(payload.message.clone()));

    // Préparez les variables d'entrée
    let input_variables = prompt_args! {
        "system" => SYSTEM_PROMPT,
        "input" => payload.message,
        "user_vector" => user_vector,
        "history" => chat_history.clone(),
        "context" => serde_json::to_string(&vectors).unwrap(),
    };

    match executor.invoke(input_variables).await {
        Ok(result) => {
            let response = result;
            // Ajoutez la réponse de l'AI à l'historique
            chat_history.push(Message::new_ai_message(response.clone()));
            (StatusCode::OK, Json(ChatResponse { answer: response }))
        }
        Err(e) => {
            eprintln!("Error invoking LLMChain: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ChatResponse {
                answer: format!("Error: {}", e),
            }))
        }
    }
}
async fn fetch_vectors_internal() -> Result<Vec<FormattedVectorEntity>, String> {
    // Charger les informations de configuration
    let account = env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key = env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");
    let table_name = env::var("STORAGE_TABLE_NAME").expect("Set env variable STORAGE_TABLE_NAME first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let table_service = TableServiceClient::new(account, storage_credentials);
    let table_client = table_service.table_client(table_name);

    // Récupérer toutes les entités
    match azure_table::get_all_vectors(&table_client).await {
        Ok(entities) => Ok(entities),
        Err(e) => {
            // Log l'erreur si nécessaire
            eprintln!("Error fetching vectors: {:?}", e);
            Err(format!("Failed to fetch vectors: {}", e))
        }
    }
}

async fn fetch_vectors() -> Result<(StatusCode, Json<Vec<FormattedVectorEntity>>), (StatusCode, Json<String>)> {
    match fetch_vectors_internal().await {
        Ok(entities) => Ok((StatusCode::OK, Json(entities))),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e))),
    }
}

// Struct for chatbot messages
#[derive(Deserialize)]
struct ChatMessage {
    message: String,
}

// Struct for chatbot responses
#[derive(Serialize)]
struct ChatResponse {
    answer: String,
}


