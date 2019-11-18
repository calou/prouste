use crate::article::Article;
use crate::configuration::Configuration;
use crate::crawler::crawl_with_configuration;
use std::borrow::Borrow;

struct HtmlExtractor {
    pub configuration: Configuration,
}

impl HtmlExtractor {
    pub fn extract(self: &Self, raw_html: String) -> Option<Article> {
        return crawl_with_configuration(raw_html, &self.configuration);
    }
}