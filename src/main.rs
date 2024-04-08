use  contents::File;
use  qdrant_client::qdrant::QdrantClient,
use  vector::VectorDB;

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


