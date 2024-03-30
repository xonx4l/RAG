use qdrant_client::qdrant::QdrantClient;

pub struct VectorDB {
    client: QdrantClient,
    id:u64,
}

impl VectorDB {
    pub fn new (client: QdrantClient)-> Self {
        Self { client, id: 0}
    }
}
