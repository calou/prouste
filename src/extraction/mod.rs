extern crate select;

mod text;
mod content;
mod predicate;
mod stopwords;

pub mod extractor {
    use select::document::Document;

    use crate::extraction::content::{get_cleaned_text_and_links, get_top_node};
    use crate::extraction::text::*;

    pub fn get_text_from_single_extractor<T: TextExtractor>(document: &Document, extractor: T) -> String {
        let opt = extractor.extract(document);
        opt.unwrap_or_default()
    }

    pub fn get_raw_title(document: &Document) -> String {
        let extractor = TagBasedExtractor { tag: "title" }
            .or(MetaContentBasedExtractor { attr: "property", value: "og:title" })
            .or(DualTagBasedExtractor { tag1: "post-title", tag2: "headline" });
        get_text_from_single_extractor(document, extractor)
    }

    pub fn get_title(document: &Document) -> String {
        get_raw_title(document)
    }

    pub fn get_language(document: &Document) -> String {
        let extractor = TagAttributeBasedExtractor { tag: "html", attr: "lang" }.or(MetaContentBasedExtractor { attr: "http-equiv", value: "content-language" });
        let full_language = get_text_from_single_extractor(document, extractor);
        match full_language.find('-') {
            Some(idx) => String::from(&full_language[..idx]),
            _ => full_language
        }
    }

    pub fn get_favico(document: &Document) -> String {
        let extractor = LinkRelContainsHrefBasedExtractor { attr: "rel", value: " icon" };
        get_text_from_single_extractor(document, extractor)
    }

    pub fn get_canonical_link(document: &Document) -> String {
        let extractor = LinkRelEqualsHrefBasedExtractor { attr: "rel", value: "canonical" };
        get_text_from_single_extractor(document, extractor)
    }

    pub fn get_meta_keywords(document: &Document) -> String {
        let extractor = MetaContentBasedExtractor { attr: "name", value: "keywords" };
        get_text_from_single_extractor(document, extractor)
    }

    pub fn get_top_image(document: &Document) -> String {
        let extractor = TopImageExtractor {};
        get_text_from_single_extractor(document, extractor)
    }

    pub fn get_text_and_links(document: &Document, lang: &str) -> (String, Vec<String>) {
        let top_node = get_top_node(document, lang);
        match top_node {
            Some(node) => get_cleaned_text_and_links(node, lang),
            _ => (String::new(), Vec::new())
        }
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