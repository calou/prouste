

#[derive(Debug, Clone, Default)]
pub struct Article {
    pub title: String,
    pub language: String,
    pub favico: String,
}

impl Article {
    pub fn new() -> Self {
        Self::default()
    }
}