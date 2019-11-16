
#[derive(Debug, Clone, Default)]
pub struct Article {
    pub title: String,
    pub text: String,
    pub language: String,
    pub favico: String,
    pub canonical_link: String,
    pub meta_keywords: String,
    pub top_image: String,
    pub links: Vec<String>,
}

impl Article {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn title(self: Self) -> String {
        self.title
    }
    pub fn language(self: Self) -> String {
        self.language
    }
    pub fn text(self: Self) -> String {
        self.text
    }
}