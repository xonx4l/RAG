use qdrant_client::qdrant::QdrantClient;
use crate::errors::EmbeddingError;
pub struct VectorDB {
    client: QdrantClient,
    id:u64,
}

impl VectorDB {
    pub fn new (client: QdrantClient)-> Self {
        Self { client, id: 0}

    pub async fn upsert_embedding(&mut self ,embedding:Embedding, file: &File) -> Result<()> {
        let payload: Payload = json!({
            "id": file.path.clone(),
        })
        .try_into()
        .map_err(|_| EmbeddingError {})?;

        println!("Embedded: {}", file.path);

        let vec: Vec<f32> = embedding.vec.iter().map(|&x| x as f32). collect();

        let points = vec![PointStruct::new(self.id, vec, payload)];
        self.client.upsert_points(COLLECTION, None, points, None).await?;
        self.id +=1;

        Ok(())
    }
    }
}
