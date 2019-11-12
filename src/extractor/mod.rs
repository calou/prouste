extern crate select;

pub mod text;
//pub mod content;

pub mod extractor {
    use select::document::Document;

    use crate::extractor::text::{DualTagBasedExtractor, MetaBasedExtractor, TagBasedExtractor, TextExtractor, TagAttributeBasedExtractor};
    use std::ops::Index;

    fn get_text_from_extractors(document: Document, text_extractors: Box<[Box<dyn TextExtractor>]>) -> String {
        for text_extractor in text_extractors.iter() {
            let extr = &**text_extractor;
            let text_extraction = extr.extract(document.to_owned());
            if text_extraction.successful {
                return text_extraction.text;
            }
        }
        return String::new();
    }

    pub fn get_raw_title(document: Document) -> String {
        let title_extractors: Box<[Box<dyn TextExtractor>; 3]> = Box::new([
            Box::new(TagBasedExtractor { tag: "title" }),
            Box::new(MetaBasedExtractor { attr: "property", value: "og:title" }),
            Box::new(DualTagBasedExtractor { tag1: "post-title", tag2: "headline" }),
        ]);
        return get_text_from_extractors(document, title_extractors);
    }

    pub fn get_title(document: Document) -> String {
        return get_raw_title(document);
    }

    pub fn get_meta_language(document: Document) -> String {
        let meta_extractors: Box<[Box<dyn TextExtractor>; 2]> = Box::new([
            Box::new(TagAttributeBasedExtractor { tag: "html", attr: "lang" }),
            Box::new(MetaBasedExtractor { attr: "http-equiv", value: "content-language"  }),
        ]);
        let full_language = get_text_from_extractors(document, meta_extractors);
        return match full_language.find("-"){
            Some(idx) => String::from(&full_language[..idx]),
            _ => full_language
        };
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_title_abcnews() {
            let document = Document::from(include_str!("sites/abcnews.go.com.html"));
            assert_eq!(get_title(document), "New Jersey Devils Owner Apologizes After Landing Helicopter in Middle of Kids' Soccer Game Forces Cancellation - ABC News");
        }

        #[test]
        fn test_get_meta_language_abcnews() {
            let document = Document::from(include_str!("sites/abcnews.go.com.html"));
            assert_eq!(get_meta_language(document), "en");
        }

        #[test]
        fn test_get_meta_language_bbc() {
            let document = Document::from(include_str!("sites/bbc.co.uk.html"));
            assert_eq!(get_meta_language(document), "en");
        }

        #[test]
        fn test_get_meta_language_huffington_jp() {
            let document = Document::from(include_str!("sites/huffingtonpost.jp.html"));
            assert_eq!(get_meta_language(document), "ja");
        }

        #[test]
        fn test_get_meta_language_vnexpress() {
            let document = Document::from(include_str!("sites/vnexpress.net.html"));
            assert_eq!(get_meta_language(document), "vi");
        }
    }
}