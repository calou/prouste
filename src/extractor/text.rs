use std::string::String;

use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

pub struct TextExtraction {
    pub successful: bool,
    pub text: String,
}

pub trait TextExtractor {
    fn extract(&self, document: Document) -> TextExtraction;
}

#[derive(Debug)]
pub struct TagBasedExtractor {
    pub tag: &'static str,
}

impl TagBasedExtractor {
    fn extract_tag_text(&self, document: Document) -> String {
        return match document.to_owned().find(Name(self.tag)).next() {
            Some(node) => node.text(),
            _ => String::new()
        };
    }
}

impl TextExtractor for TagBasedExtractor {
    fn extract(&self, document: Document) -> TextExtraction {
        let text = self.extract_tag_text(document);
        return TextExtraction {
            successful: text != "",
            text,
        };
    }
}

#[derive(Debug)]
pub struct MetaBasedExtractor {
    pub attr: &'static str,
    pub value: &'static str,
}

impl MetaBasedExtractor {
    fn extract_meta_content(&self, document: Document) -> String {
        return match document.to_owned().find(Name("meta").and(Attr(self.attr, self.value))).next() {
            Some(node) => String::from(node.attr("content").unwrap_or("")),
            _ => String::new()
        };
    }
}

impl TextExtractor for MetaBasedExtractor {
    fn extract(&self, document: Document) -> TextExtraction {
        let text = self.extract_meta_content(document);
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
        let extractor = TagBasedExtractor { tag: "title" };
        let extraction = extractor.extract(document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "New Jersey Devils Owner Apologizes After Landing Helicopter in Middle of Kids' Soccer Game Forces Cancellation - ABC News");
    }

    #[test]
    fn extract_with_tag_absent() {
        let document = Document::from("<html></html>");
        let extractor = TagBasedExtractor { tag: "title" };
        let extraction = extractor.extract(document);
        assert!(!extraction.successful);
    }


    #[test]
    fn extract_with_og_abcnews() {
        let document = Document::from(include_str!("sites/abcnews.go.com.html"));
        let extractor = MetaBasedExtractor { attr: "property", value: "og:title" };
        let extraction = extractor.extract(document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "NHL Owner Apologizes for Landing Helicopter at Kids' Soccer Game");
    }
}