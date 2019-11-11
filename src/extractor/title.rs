extern crate select;

use std::string::String;

use select::document::Document;
use select::predicate::{Attr, Name};

use crate::extractor::text::{TextExtraction, TextExtractor};

pub struct TitleTagExtractor;

impl TextExtractor for TitleTagExtractor {
    fn extract(&self, document: Document) -> TextExtraction {
        let text = match document.find(Name("title")).next() {
            Some(node) => node.text(),
            _ => String::new()
        };
        return TextExtraction {
            successful: text != "",
            text,
        };
    }
}

#[cfg(test)]
mod tests {
    use std::string::String;

    use select::document::Document;

    use super::*;

    #[test]
    fn extract_with_tag_abcnews() {
        let document = Document::from(include_str!("sites/abcnews.go.com.html"));
        let extractor = TitleTagExtractor;
        let extraction = extractor.extract(document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "New Jersey Devils Owner Apologizes After Landing Helicopter in Middle of Kids' Soccer Game Forces Cancellation - ABC News");
    }

    #[test]
    fn extract_with_tag_absent() {
        let document = Document::from("<html></html>");
        let extractor = TitleTagExtractor;
        let extraction = extractor.extract(document);
        assert!(!extraction.successful);
    }
}
