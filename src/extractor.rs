extern crate select;

use std::string::String;
use select::predicate::{Attr, Name};

pub mod extractor {
    use select::document::Document;
    use super::select::predicate::Name;

    struct TextExtraction {
        successful: bool,
        text: String,
    }

    trait  TextExtractor {
        fn extract(&self, document: Document) -> TextExtraction;
    }

    struct TitleTagExtractor;

    impl TextExtractor for TitleTagExtractor {
        fn extract(&self, document: Document) -> TextExtraction {
            let text = match document.find(Name("title")).next() {
                Some(node) => node.text(),
                _ => String::new()
            };
            return TextExtraction {
                successful: text != "",
                text
            }
        }
    }

    pub fn get_title(document: Document) -> String {
        let text_extractors = [ TitleTagExtractor ];
        for text_extractor in &text_extractors {
            let text_extraction = text_extractor.extract(document.to_owned());
            if text_extraction.successful {
                return text_extraction.text;
            }
        }
        return String::new();
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