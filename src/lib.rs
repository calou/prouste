#![feature(test)]
#[macro_use]
extern crate lazy_static;
extern crate test;
extern crate select;
extern crate chardet;
extern crate serde;
extern crate indexmap;

pub mod article;
pub mod configuration;
pub mod html;
mod embedding;
mod extraction;

#[cfg(test)]
mod tests {
    use std::fs;
    use test::Bencher;
    use std::string::String;
    use crate::configuration::Configuration;
    use crate::html::HtmlExtractor;

    use super::*;

    #[bench]
    fn bench_crawl_abc(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extraction/sites/abcnews.go.com.html")
            .expect("Something went wrong reading the file");

        let configuration = Configuration::default();
        let extractor = HtmlExtractor { configuration };
        let ptr = raw_html.as_str();
        b.iter(|| extractor.from_string(String::from(ptr)));
    }

    #[bench]
    fn bench_crawl_theguardian(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extraction/sites/theguardian.com.html")
            .expect("Something went wrong reading the file");
        let configuration = Configuration::default();
        let extractor = HtmlExtractor { configuration };
        let ptr = raw_html.as_str();
        b.iter(|| extractor.from_string(String::from(ptr)));
    }

    #[bench]
    fn bench_crawl_inc(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extraction/sites/inc.com.html")
            .expect("Something went wrong reading the file");
        let configuration = Configuration::default();
        let extractor = HtmlExtractor { configuration };
        let ptr = raw_html.as_str();
        b.iter(|| extractor.from_string(String::from(ptr)));
    }

    #[bench]
    fn bench_crawl_charset_koi8_r(b: &mut Bencher) {
        let raw_content = fs::read("src/extraction/sites/charset_koi8_r.html")
            .expect("Something went wrong reading the file");

        let configuration = Configuration::default();
        let extractor = HtmlExtractor { configuration };
        b.iter(|| extractor.from_bytes(raw_content.to_vec()));
    }

    #[bench]
    fn bench_crawl_telegraph(b: &mut Bencher) {
        let raw_content = fs::read("src/extraction/sites/telegraph.co.uk.html")
            .expect("Something went wrong reading the file");

        let extractor = HtmlExtractor::default();
        b.iter(|| extractor.from_bytes(raw_content.to_vec()));
    }

}
