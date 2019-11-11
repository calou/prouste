

#[derive(Debug, Clone, Default)]
pub struct Article {
    pub title: String,
}

impl Article {
    pub fn new() -> Self {
        Self::default()
    }
}