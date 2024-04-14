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

    pub async fn search(&self, embedding: Embedding) -> Result<ScoredPoint> {
        let vec: Vec<f32> = embedding.vec.iter().map(|&x| x as f32).collect();

        let payload_selector = WithPayloadSelector {
            selector_options: Some(SelectorOptions::Enable(true)),
        };

        let search_points = SearchPoints {
            collection_name: COLLECTION.to_string(),
            vector: vec,
            limit: 1,
            with_payload: Some(payload_selector),
            ..Default::default()
        };

        let search_result = self.client.search_points(&search_points).await?;
        let result = search_result.result[0].clone();
        Ok(result)
    }
}
}

