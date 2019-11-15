#![feature(test)]

extern crate test;

pub mod article;
pub mod crawler;
mod charset;
mod extractor;


#[cfg(test)]
mod tests {
    use std::fs;
    use test::Bencher;
    use std::string::String;
    use crate::crawler::crawl;

    use super::*;

    #[bench]
    fn bench_crawl_abc(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extractor/sites/abcnews.go.com.html")
            .expect("Something went wrong reading the file");
        let ptr = raw_html.as_str();
        b.iter(|| crawl(String::from(ptr)));
    }

    #[bench]
    fn bench_crawl_theguardian(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extractor/sites/theguardian.com.html")
            .expect("Something went wrong reading the file");
        let ptr = raw_html.as_str();
        b.iter(|| crawl(String::from(ptr)));
    }


    #[bench]
    fn bench_crawl_inc(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extractor/sites/inc.com.html")
            .expect("Something went wrong reading the file");
        let ptr = raw_html.as_str();
        b.iter(|| crawl(String::from(ptr)));
    }
 /*
    #[bench]
    fn bench_crawl_inc(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extractor/sites/inc.com.html")
            .expect("Something went wrong reading the file");

        b.iter(|| crawl(raw_html));
    }

    #[bench]
    fn bench_crawl_theguardian(b: &mut Bencher) {
        let raw_html = fs::read_to_string("src/extractor/sites/theguardian.com.html")
            .expect("Something went wrong reading the file");

        b.iter(|| crawl(raw_html));
    }
    */
}
