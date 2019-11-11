pub mod title;
pub mod text;

extern crate select;

use std::string::String;

pub mod extractor {
    use select::document::Document;
    use crate::extractor::text::TextExtractor;
    use crate::extractor::title::TitleTagExtractor;


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