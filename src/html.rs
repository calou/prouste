use chardet::{charset2encoding, detect};
use encoding::DecoderTrap;
use encoding::label::encoding_from_whatwg_label;
use select::document::Document;

use crate::article::{Article, Embeddings};
use crate::configuration::Configuration;
use crate::embedding::*;
use crate::extraction::extractor::*;

pub struct HtmlExtractor {
    pub configuration: Configuration,
}

impl Default for HtmlExtractor {
    fn default() -> Self { HtmlExtractor { configuration: Configuration::default() } }
}

impl HtmlExtractor {
    pub fn from_string(self: &Self, raw_html: String) -> Option<Article> {
        let option = self.pre_process(raw_html);
        match option {
            Some(document) => self.process(&document, &self.configuration),
            _ => None
        }
    }

    pub fn from_bytes(self: &Self, bytes: Vec<u8>) -> Option<Article> {
        match Document::from_read(::std::io::Cursor::new(bytes.to_owned())) {
            Ok(document) => self.process(&document, &self.configuration),
            _ => self.from_non_utf8_bytes(bytes)
        }
    }

    fn from_non_utf8_bytes(self: &Self, bytes: Vec<u8>) -> Option<Article> {
        let result = detect(&bytes);
        match encoding_from_whatwg_label(charset2encoding(&result.0)) {
            Some(encoding) => {
                let utf8reader = encoding.decode(&bytes, DecoderTrap::Ignore).expect("Error");
                match self.pre_process(utf8reader) {
                    Some(document) => self.process(&document, &self.configuration),
                    _ => None
                }
            }
            _ => None
        }
    }

    fn pre_process(self: &Self, raw_html: String) -> Option<Document> {
        if raw_html == "" {
            return None;
        }
        let document = Document::from(raw_html.as_str());
        Some(document)
    }

    fn process(self: &Self, document: &Document, config: &Configuration) -> Option<Article> {
        let mut article = Article::new();

        article.language = get_language(&document);
        if config.enable_meta_extraction {
            article.favico = get_favico(&document);
            article.canonical_link = get_canonical_link(&document);
            article.meta_keywords = get_meta_keywords(&document);
            article.top_image = get_top_image(&document);
        }
        if config.enable_text_extraction {
            article.title = get_title(&document);
            let (text, links) = get_text_and_links(&document, article.language.as_ref());
            article.text = text;
            article.links = links;
        }
        if config.enable_embeddings_extraction {
            article.embeddings = Embeddings {
                tweets: get_tweets(&document),
                instagram_posts: get_instagram_posts(&document),
            }
        }
        Some(article)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::configuration::Configuration;

    use super::*;

    #[test]
    fn test_crawl_abc() {
        let extractor = HtmlExtractor::default();

        let raw_html = fs::read_to_string("src/extraction/sites/abcnews.go.com.html")
            .expect("Something went wrong reading the file");
        let option = extractor.from_string(raw_html);
        let article = option.unwrap();
        assert_eq!(article.title, "New Jersey Devils Owner Apologizes After Landing Helicopter in Middle of Kids' Soccer Game Forces Cancellation - ABC News");
        assert_eq!(article.canonical_link, "http://abcnews.go.com/US/nj-devils-owner-apologizes-landing-helicopter-middle-kids/story?id=35155591");
        assert_eq!(article.meta_keywords, "nj devils owner lands helicopter kids soccer game, helicopter youth soccer game, newark, new jersey, nj nj devils, nhl, josh harris, helicopter cancels soccer game, st benedict preparatory school, sta u13, youth soccer, us news, national news, local news");
        assert_eq!(article.top_image, "http://a.abcnews.go.com/images/US/ht_devils_helicopter_landing_hb_151112_16x9_992.jpg");
        for link in article.links {
            println!("{}", link);
        }
    }

    #[test]
    fn test_crawl_bizjournal() {
        let configuration = Configuration { enable_text_extraction: true, enable_embeddings_extraction: true, enable_meta_extraction: true };
        let extractor = HtmlExtractor { configuration };

        let raw_html = fs::read_to_string("src/extraction/sites/bizjournals.com.html")
            .expect("Something went wrong reading the file");
        let option = extractor.from_string(raw_html);
        let article = option.unwrap();
        assert_eq!(article.favico, "http://assets.bizjournals.com/lib/img/favicon.ico");
    }

    #[test]
    fn test_crawl_vnexpress() {
        let configuration = Configuration { enable_text_extraction: true, enable_embeddings_extraction: true, enable_meta_extraction: true };
        let extractor = HtmlExtractor { configuration };

        let raw_html = fs::read_to_string("src/extraction/sites/vnexpress.net.html")
            .expect("Something went wrong reading the file");
        let option = extractor.from_string(raw_html);
        let article = option.unwrap();
        assert_eq!(article.title, "Khánh Ly đến viếng mộ Trịnh Công Sơn - VnExpress Giải Trí");
        assert_eq!(article.language, "vi");
    }

    #[test]
    fn test_crawl_closermag() {
        let configuration = Configuration { enable_text_extraction: true, enable_embeddings_extraction: true, enable_meta_extraction: true };
        let extractor = HtmlExtractor { configuration };

        let raw_html = fs::read_to_string("src/extraction/sites/closermag.fr.html")
            .expect("Something went wrong reading the file");
        let option = extractor.from_string(raw_html);
        println!("{}", option.unwrap().text);
    }

    #[test]
    fn test_crawl_charset_koi8_r() {
        let configuration = Configuration { enable_text_extraction: true, enable_embeddings_extraction: true, enable_meta_extraction: true };
        let extractor = HtmlExtractor { configuration };

        let raw_content = fs::read("src/extraction/sites/charset_koi8_r.html")
            .expect("Something went wrong reading the file");
        let option = extractor.from_bytes(raw_content);
        println!("{}", option.unwrap().text);
    }
}