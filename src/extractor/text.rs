use std::collections::{HashMap, BTreeMap};
use std::string::String;

use select::document::Document;
use select::predicate::{Attr, Name, Predicate};

use crate::extractor::predicate::{AttrContains, ImageTag};

pub struct TextExtraction {
    pub successful: bool,
    pub text: String,
}

pub trait TextExtractor {
    fn extract(&self, document: &Document) -> TextExtraction;
}

#[derive(Debug)]
pub struct TagBasedExtractor {
    pub tag: &'static str,
}

impl TagBasedExtractor {
    fn extract_tag_text(&self, document: &Document) -> String {
        return match document.find(Name(self.tag)).next() {
            Some(node) => node.text(),
            _ => String::new()
        };
    }
}

impl TextExtractor for TagBasedExtractor {
    fn extract(&self, document: &Document) -> TextExtraction {
        let text = self.extract_tag_text(document);
        return TextExtraction {
            successful: text != "",
            text,
        };
    }
}

#[derive(Debug)]
pub struct DualTagBasedExtractor {
    pub tag1: &'static str,
    pub tag2: &'static str,
}

impl DualTagBasedExtractor {
    fn extract_tag_text(&self, document: &Document) -> String {
        return match document.find(Name(self.tag1).or(Name(self.tag2))).next() {
            Some(node) => node.text(),
            _ => String::new()
        };
    }
}

impl TextExtractor for DualTagBasedExtractor {
    fn extract(&self, document: &Document) -> TextExtraction {
        let text = self.extract_tag_text(document);
        return TextExtraction {
            successful: text != "",
            text,
        };
    }
}

#[derive(Debug)]
pub struct MetaContentBasedExtractor {
    pub attr: &'static str,
    pub value: &'static str,
}

impl MetaContentBasedExtractor {
    fn extract_meta_content(&self, document: &Document) -> String {
        return match document.find(Name("meta").and(Attr(self.attr, self.value))).next() {
            Some(node) => String::from(node.attr("content").unwrap_or("")),
            _ => String::new()
        };
    }
}

impl TextExtractor for MetaContentBasedExtractor {
    fn extract(&self, document: &Document) -> TextExtraction {
        let text = self.extract_meta_content(&document);
        return TextExtraction {
            successful: text != "",
            text,
        };
    }
}

#[derive(Debug)]
pub struct TagAttributeBasedExtractor {
    pub tag: &'static str,
    pub attr: &'static str,
}

impl TagAttributeBasedExtractor {
    fn extract_tag_attr(&self, document: &Document) -> String {
        return match document.find(Name(self.tag)).next() {
            Some(node) => String::from(node.attr(self.attr).unwrap_or("")),
            _ => String::new()
        };
    }
}

impl TextExtractor for TagAttributeBasedExtractor {
    fn extract(&self, document: &Document) -> TextExtraction {
        let text = self.extract_tag_attr(&document);
        return TextExtraction {
            successful: text != "",
            text,
        };
    }
}

#[derive(Debug)]
pub struct LinkRelEqualsHrefBasedExtractor {
    pub attr: &'static str,
    pub value: &'static str,
}

impl LinkRelEqualsHrefBasedExtractor {
    fn extract_link_url(&self, document: &Document) -> String {
        return match document.find(Name("link").and(Attr(self.attr, self.value))).next() {
            Some(node) => String::from(node.attr("href").unwrap_or("")),
            _ => String::new()
        };
    }
}

impl TextExtractor for LinkRelEqualsHrefBasedExtractor {
    fn extract(&self, document: &Document) -> TextExtraction {
        let text = self.extract_link_url(&document);
        return TextExtraction {
            successful: text != "",
            text,
        };
    }
}


#[derive(Debug)]
pub struct LinkRelContainsHrefBasedExtractor {
    pub attr: &'static str,
    pub value: &'static str,
}

impl LinkRelContainsHrefBasedExtractor {
    fn extract_link_url(&self, document: &Document) -> String {
        return match document.find(Name("link").and(AttrContains(self.attr, self.value))).next() {
            Some(node) => String::from(node.attr("href").unwrap_or("")),
            _ => String::new()
        };
    }
}

impl TextExtractor for LinkRelContainsHrefBasedExtractor {
    fn extract(&self, document: &Document) -> TextExtraction {
        let text = self.extract_link_url(&document);
        return TextExtraction {
            successful: text != "",
            text,
        };
    }
}

#[derive(Debug)]
pub struct TopImageExtractor;

impl TextExtractor for TopImageExtractor {
    fn extract(&self, document: &Document) -> TextExtraction {
        let mut counts = BTreeMap::new();
        for node in document.find(ImageTag {}).into_iter() {
            println!("matching {} ", node.html());
            match node.name() {
                Some("meta") => {
                    match node.attr("name") {
                        Some("og:image") | Some("twitter:image") | Some("twitter:image:src") => {
                            let key = node.attr("content").unwrap_or("");
                            *counts.entry(String::from(key)).or_insert(0u32) += 1u32;
                        }
                        _ => ()
                    }
                    match node.attr("property") {
                        Some("og:image") | Some("twitter:image") | Some("twitter:image:src") => {
                            let key = node.attr("content").unwrap_or("");
                            *counts.entry(String::from(key)).or_insert(0u32) += 1u32;
                        }
                        _ => ()
                    }
                }
                Some("link") => {
                    match node.attr("rel") {
                        Some("image_src") => {
                            let key = node.attr("href").unwrap_or("");
                            *counts.entry(String::from(key)).or_insert(0u32) += 1u32;
                        }
                        _ => ()
                    }
                }
                _ => ()
            }
        }
        let mut text = String::new();
        let mut max_count = 0u32;
        for (img, c) in counts.iter() {
            println!("{} - {}", c, img);
            if *c > max_count {
                text = img.to_owned();
                max_count = *c;
            }
        }
        return TextExtraction {
            successful: text != "",
            text,
        };
    }
}



#[cfg(test)]
mod tests {
    use select::document::Document;

    use super::*;

    #[test]
    fn extract_with_tag_abcnews() {
        let document = Document::from(include_str!("sites/abcnews.go.com.html"));
        let extractor = TagBasedExtractor { tag: "title" };
        let extraction = extractor.extract(&document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "New Jersey Devils Owner Apologizes After Landing Helicopter in Middle of Kids' Soccer Game Forces Cancellation - ABC News");
    }

    #[test]
    fn extract_with_tag_absent() {
        let document = Document::from("<html></html>");
        let extractor = TagBasedExtractor { tag: "title" };
        let extraction = extractor.extract(&document);
        assert!(!extraction.successful);
    }

    #[test]
    fn extract_with_og_abcnews() {
        let document = Document::from(include_str!("sites/abcnews.go.com.html"));
        let extractor = MetaContentBasedExtractor { attr: "property", value: "og:title" };
        let extraction = extractor.extract(&document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "NHL Owner Apologizes for Landing Helicopter at Kids' Soccer Game");
    }

    #[test]
    fn extract_with_multitags_abcnews() {
        let document = Document::from("<html><b>B first value</b><a>A first value</a><b>B second value</b></html>");
        let extractor = DualTagBasedExtractor { tag1: "a", tag2: "b" };
        let extraction = extractor.extract(&document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "B first value");
    }

    #[test]
    fn extract_with_tag_attr_abcnews() {
        let document = Document::from(include_str!("sites/abcnews.go.com.html"));
        let extractor = TagAttributeBasedExtractor { tag: "html", attr: "lang" };
        let extraction = extractor.extract(&document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "en");
    }

    #[test]
    fn extract_with_link_href_bizjournals() {
        let document = Document::from(include_str!("sites/bizjournals.com.html"));
        let extractor = LinkRelContainsHrefBasedExtractor { attr: "rel", value: " icon" };
        let extraction = extractor.extract(&document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "http://assets.bizjournals.com/lib/img/favicon.ico");
    }


    #[test]
    fn extract_top_image_bizjournals() {
        let document = Document::from(include_str!("sites/bizjournals.com.html"));
        let extractor = TopImageExtractor {};
        let extraction = extractor.extract(&document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "http://media.bizj.us/view/img/2167041/mason-morfit*400xx306-307-0-25.jpg");
    }

    #[test]
    fn extract_top_image_blogspot_co_uk() {
        let document = Document::from(include_str!("sites/blogspot.co.uk.html"));
        let extractor = TopImageExtractor {};
        let extraction = extractor.extract(&document);
        assert!(extraction.successful);
        assert_eq!(extraction.text, "http://3.bp.blogspot.com/-6SCcCupadL0/VUnQdhs_98I/AAAAAAAAA7Q/wCdIXm6v9Sg/s540/Screen%2BShot%2B2015-05-06%2Bat%2B10.22.08%2BAM.png");
    }
}