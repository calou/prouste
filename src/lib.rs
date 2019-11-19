#![feature(test)]
#[macro_use]
extern crate lazy_static;
extern crate test;
extern crate select;

pub mod article;
pub mod configuration;
pub mod html;
mod embedding;
mod extractor;

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
        let raw_html = fs::read_to_string("src/extractor/sites/abcnews.go.com.html")
            .expect("Something went wrong reading the file");

        let configuration = Configuration { enable_text_extraction: true, enable_embedding_extraction: true };
        let extractor = HtmlExtractor { configuration };
        let ptr = raw_html.as_str();
        b.iter(|| extractor.extract(String::from(ptr)));
    }

    #[bench]
    fn bench_crawl_theguardian(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extractor/sites/theguardian.com.html")
            .expect("Something went wrong reading the file");
        let configuration = Configuration { enable_text_extraction: true, enable_embedding_extraction: true };
        let extractor = HtmlExtractor { configuration };
        let ptr = raw_html.as_str();
        b.iter(|| extractor.extract(String::from(ptr)));
    }


    #[bench]
    fn bench_crawl_inc(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extractor/sites/inc.com.html")
            .expect("Something went wrong reading the file");
        let configuration = Configuration { enable_text_extraction: true, enable_embedding_extraction: true };
        let extractor = HtmlExtractor { configuration };
        let ptr = raw_html.as_str();
        b.iter(|| extractor.extract(String::from(ptr)));
    }
}
