use  contents::File;
use  qdrant_client::qdrant::QdrantClient,
use  vector::VectorDB;
use crate::errors::PromptError;
use anyhow::Result;
use crate::{open_ai, AppState};
use std::sync::Arc;
use axum::{Json, extract::State};
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;


mod open_ai;
mod vector;
mod contents;

struct AppState {
    files: Vec<File>,
    vector_db:VectorDB,
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

#[shuttle_runtime::main]
async fn axum(
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
    #[shuttle_qdrant::Qdrant(
        cloud_url = "{secrets.QDRANT_URL}",
        api_key = "{secrets.QDRANT_TOKEN}"
    )]
    qdrant_client: QdrantClient,
) -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
      .route("/", get(hello_world));

    Ok(router.into())  
}

async fn embed_documentation(vector_db: &mut VectorDB, files: &Vec<File>) -> anyhow::Result<()> {
    for file in files {
        let embeddings = open_ai::embed_file(file).await?;
        println!("Embedding: {:?}", file.path);
        for embedding in embeddings.data {
            vector.db.upsert_embedding(embedding, file).await?;
        }
    }

    Ok(())
}

async fn get_contents(
    prompt: &str,
    app_state: &AppState,
) -> anyhow::Result<Receiver<ChatCompletionDelta>> {
    let embedding = open_ai::embed_sentence(prompt).await?;
    let result = app_state.vector_db.search(embedding).await?;
    println!("Result: {:?}", result);
    let contents = app_state
        .files
        .get_contents(&result)
        .ok_or(PromptError {})?;
    open_ai::chat_stream(prompt, contents.as_str()).await
}
