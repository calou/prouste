use crate::embedding::Embedding;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Debug, Clone, Default, Deserialize, Serialize)]
pub struct Embeddings {
    pub tweets: Vec<Embedding>,
    pub instagram_posts: Vec<Embedding>,
}

#[derive(PartialEq, Debug, Clone, Default, Deserialize, Serialize)]
pub struct Article {
    pub title: String,
    pub text: String,
    pub language: String,
    pub favico: String,
    pub canonical_link: String,
    pub meta_keywords: String,
    pub top_image: String,
    pub links: Vec<String>,
    pub embeddings: Embeddings,
}

impl Article {
    pub fn new() -> Self {
        Self::default()
    }
}