extern crate select;

pub mod text;
//pub mod content;
pub mod predicate;

pub mod extractor {
    use select::document::Document;

    use crate::extractor::text::*;
    use std::ops::Index;

    fn get_text_from_multiple_extractors(document: &Document, text_extractors: Box<[Box<dyn TextExtractor>]>) -> String {
        for text_extractor in text_extractors.iter() {
            let extr = &**text_extractor;
            let text_extraction = extr.extract(document);
            if text_extraction.successful {
                return text_extraction.text;
            }
        }
        return String::new();
    }

    pub fn get_text_from_single_extractor(document: &Document, extractor: Box<dyn TextExtractor>) -> String {
        let text_extraction = extractor.extract(document);
        if text_extraction.successful {
            return text_extraction.text;
        } else {
            return String::new()
        }
    }

    pub fn get_raw_title(document: &Document) -> String {
        let title_extractors: Box<[Box<dyn TextExtractor>; 3]> = Box::new([
            Box::new(TagBasedExtractor { tag: "title" }),
            Box::new(MetaBasedExtractor { attr: "property", value: "og:title" }),
            Box::new(DualTagBasedExtractor { tag1: "post-title", tag2: "headline" }),
        ]);
        return get_text_from_multiple_extractors(document, title_extractors);
    }

    pub fn get_title(document: &Document) -> String {
        return get_raw_title(document);
    }

    pub fn get_language(document: &Document) -> String {
        let meta_extractors: Box<[Box<dyn TextExtractor>; 2]> = Box::new([
            Box::new(TagAttributeBasedExtractor { tag: "html", attr: "lang" }),
            Box::new(MetaBasedExtractor { attr: "http-equiv", value: "content-language"  }),
        ]);
        let full_language = get_text_from_multiple_extractors(document, meta_extractors);
        return match full_language.find("-"){
            Some(idx) => String::from(&full_language[..idx]),
            _ => full_language
        };
    }

    pub fn get_favico(document: &Document) -> String {
        let extractor = LinkRelContainsHrefBasedExtractor { attr: "rel", value: " icon" };
        return get_text_from_single_extractor(document, Box::new(extractor));
    }

    pub fn get_canonical_link(document: &Document) -> String {
        let extractor = LinkRelContainsHrefBasedExtractor { attr: "rel", value: "canonical" };
        return get_text_from_single_extractor(document, Box::new(extractor));
    }


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_title_abcnews() {
            let document = Document::from(include_str!("sites/abcnews.go.com.html"));
            assert_eq!(get_title(&document), "New Jersey Devils Owner Apologizes After Landing Helicopter in Middle of Kids' Soccer Game Forces Cancellation - ABC News");
        }

        #[test]
        fn test_get_meta_language_abcnews() {
            let document = Document::from(include_str!("sites/abcnews.go.com.html"));
            assert_eq!(get_language(&document), "en");
        }

        #[test]
        fn test_get_meta_language_bbc() {
            let document = Document::from(include_str!("sites/bbc.co.uk.html"));
            assert_eq!(get_language(&document), "en");
        }

        #[test]
        fn test_get_meta_language_huffington_jp() {
            let document = Document::from(include_str!("sites/huffingtonpost.jp.html"));
            assert_eq!(get_language(&document), "ja");
        }

        #[test]
        fn test_get_meta_language_vnexpress() {
            let document = Document::from(include_str!("sites/vnexpress.net.html"));
            assert_eq!(get_language(&document), "vi");
        }

        #[test]
        fn test_get_favico() {
            let document = Document::from(include_str!("sites/bizjournals.com.html"));
            assert_eq!(get_favico(&document), "http://assets.bizjournals.com/lib/img/favicon.ico");
        }

        #[test]
        fn test_get_canonical_link_abcnews() {
            let document = Document::from(include_str!("sites/abcnews.go.com.html"));
            assert_eq!(get_canonical_link(&document), "http://abcnews.go.com/US/nj-devils-owner-apologizes-landing-helicopter-middle-kids/story?id=35155591");
        }

    }
}