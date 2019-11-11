extern crate select;

use std::string::String;

pub mod text;

pub mod extractor {
    use select::document::Document;

    use crate::extractor::text::{DualTagBasedExtractor, MetaBasedExtractor, TagBasedExtractor, TextExtractor};

    fn get_text_from_extractors(document: Document, text_extractors: Box<[Box<TextExtractor>]>) -> String {
        for text_extractor in text_extractors.iter() {
            let extr = &**text_extractor;
            let text_extraction = extr.extract(document.to_owned());
            if text_extraction.successful {
                return text_extraction.text;
            }
        }
        return String::new();
    }


    pub fn get_title(document: Document) -> String {
        let title_extractors: Box<[Box<TextExtractor>; 3]> = Box::new([
            Box::new(TagBasedExtractor { tag: "title" }),
            Box::new(MetaBasedExtractor { attr: "property", value: "og:title" }),
            Box::new(DualTagBasedExtractor { tag1: "post-title", tag2: "headline" }),
        ]);
        return get_text_from_extractors(document, title_extractors);
    }


    #[cfg(test)]
    mod tests {
        use std::string::String;

        use super::*;

        #[test]
        fn test_get_title_abcnews() {
            let document = Document::from(include_str!("sites/abcnews.go.com.html"));
            assert_eq!(get_title(document), "New Jersey Devils Owner Apologizes After Landing Helicopter in Middle of Kids' Soccer Game Forces Cancellation - ABC News");
        }
    }
}