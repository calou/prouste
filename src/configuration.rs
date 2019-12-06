pub struct Configuration {
    pub enable_text_extraction: bool,
    pub enable_embeddings_extraction: bool,
    pub enable_meta_extraction: bool,
}

impl Default for Configuration {
    fn default() -> Self { Configuration { enable_text_extraction: true, enable_embeddings_extraction: true, enable_meta_extraction: true } }
}