use anyhow::Result;
use crate::errors::EmbeddingError;
use openai::{
    chat::{ChatCompletion, ChatCompletionBuilder, ChatCompletionDelta, ChatCompletionMessage},
    embeddings::{Embedding, Embeddings},
};
use tokio::sync::mpsc::Receiver;


pub fn setup(secrets: &SecretStore) -> Result<()> {

    let openai_key = secrets
       .get("OPENAAI_API_KEY")
       .ok_or(SetupError("OPENAI key not available "))?;
    openai::set_key(openai_key);
    Ok(())
}

pub async fn embed_file(file: &File) -> Result<Embeddings> {
    let sentence_as_str: Vec<&str> = file.sentences.iter().map(|s| s.as_str()).collect();
    Embeddings::create("text-embedding-ada-002", sentence_as_str, "shuttle")
         .await
         .map_err(|_| EmbeddingError {}.into())
}

pub async fn embed_sentence(prompt: &str) -> Result<Embeddings> {
    Embedding::create("text-embedding-ada-002", prompt, "shuttle")
        .await
        .map_err(|_| EmbeddingError {}.into)
}

type Conversation = Receiver<ChatCompletionDelta>;

pub async fn chat_stream() {
    
}
