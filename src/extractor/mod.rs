extern crate select;

use std::string::String;

pub mod text;

pub mod extractor {
    use select::document::Document;

    use crate::extractor::text::{MetaBasedExtractor, TagBasedExtractor, TextExtractor};

    pub fn get_title(document: Document) -> String {
        let title_tag_extractor = Box::new(TagBasedExtractor { tag: "title" });
        let meta_og_title_extractor = Box::new(MetaBasedExtractor { attr: "property", value:"og:title" });

        let text_extractors: [Box<TextExtractor>;2] = [title_tag_extractor, meta_og_title_extractor];
        for text_extractor in text_extractors.iter() {
            let extr = &**text_extractor;
            let text_extraction = extr.extract(document.to_owned());
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