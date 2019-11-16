#[derive(PartialEq, Debug, Clone, Default)]
pub struct Article {
    pub(crate) title: String,
    pub(crate) text: String,
    pub(crate) language: String,
    pub(crate) favico: String,
    pub(crate) canonical_link: String,
    pub(crate) meta_keywords: String,
    pub(crate) top_image: String,
    pub(crate) links: Vec<String>,
}

impl Article {
    pub fn new() -> Self {
        Self::default()
    }
}