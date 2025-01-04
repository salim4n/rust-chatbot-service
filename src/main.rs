mod azure_table;
mod agent;
mod agent_v2;

use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::env;
use azure_data_tables::{prelude::*};
use azure_storage::prelude::*;
use dotenv::dotenv;
use crate::azure_table::FormattedVectorEntity;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with routes
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        // `POST /users` goes to `create_user`
        .route("/users", post(create_user))
        // `POST /chat` goes to `answer`
        .route("/chat", post(answer))
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

// handler to simulate a chatbot response
async fn answer(
    Json(payload): Json<ChatMessage>,
) -> (StatusCode, Json<ChatResponse>) {
    // Logic for processing the question and returning an answer
    let answer_text = match payload.message.as_str() {
        "What is the meaning of life?" => "42".to_string(),
        _ => "I don't know!".to_string(),
    };

    let response = ChatResponse {
        answer: answer_text,
    };

    (StatusCode::OK, Json(response))
}

async fn fetch_vectors() -> Result<(StatusCode, Json<Vec<FormattedVectorEntity>>), (StatusCode, Json<String>)> {
    // Appelle la fonction `get_all_vectors` ici et retourne les données sous forme de JSON
    // Charger les informations de configuration
    let account =
        env::var("STORAGE_ACCOUNT").expect("Set env variable STORAGE_ACCOUNT first!");
    let access_key =
        env::var("STORAGE_ACCESS_KEY").expect("Set env variable STORAGE_ACCESS_KEY first!");
    let table_name = env::var("STORAGE_TABLE_NAME").expect("Set env variable STORAGE_TABLE_NAME first!");

    let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
    let table_service = TableServiceClient::new(account, storage_credentials);
    let table_client = table_service.table_client(table_name);

    // Récupérer toutes les entités
    match azure_table::get_all_vectors(&table_client).await {
        Ok(entities) => Ok((StatusCode::OK, Json(entities))),
        Err(e) => {
            // Log l'erreur si nécessaire
            eprintln!("Error fetching vectors: {:?}", e);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Failed to fetch vectors: {}", e))))
        }
    }
}
// handler to simulate user creation
async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
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

// Struct for creating a user
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// Struct for user response
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
