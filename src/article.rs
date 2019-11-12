

#[derive(Debug, Clone, Default)]
pub struct Article {
    pub title: String,
    pub meta_language: String,

}

impl Article {
    pub fn new() -> Self {
        Self::default()
    }
}