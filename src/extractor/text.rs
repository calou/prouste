use std::string::String;
use select::document::Document;

pub struct TextExtraction {
    pub successful: bool,
    pub text: String,
}


pub trait TextExtractor {
    fn extract(&self, document: Document) -> TextExtraction;
}