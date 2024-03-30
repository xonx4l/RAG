mod vector;
mod contents;

use  contents::File;
use  qdrant_client::qdrant::QdrantClient,

struct AppState {
    files: Vec<File>,
    vector_db:VectorDB,
}

async fn hello_world() -> &'static str {
    "Hello world!"
}

