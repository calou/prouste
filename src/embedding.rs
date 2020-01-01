use regex::Regex;
use select::document::Document;
use select::node::Node;
use select::predicate::{Name, Predicate};
use serde::{Deserialize, Serialize};

use super::select::predicate::{Child, Class};

lazy_static! {
    static ref SPACES_REGEX: Regex = Regex::new(r"\s\s+").unwrap();
}

#[derive(PartialEq, Debug, Clone, Default, Deserialize, Serialize)]
pub struct Embedding {
    pub url: String,
    pub text: String,
}

pub fn get_tweets(document: &Document) -> Vec<Embedding> {
    let mut embeddings: Vec<Embedding> = Vec::new();
    let blockquote_predicate = Name("blockquote");
    let p_predicate = Name("p");
    let child_link_predicate = Child(blockquote_predicate, Name("a"));
    for tag in document.find(blockquote_predicate.and(Class("twitter-tweet"))) {
        let mut text: String = String::default();
        let mut url: String = String::default();
        if let Some(node) = tag.find(p_predicate).next() {
             text = get_sanitized_text(node);
        }
        if let Some(node) = tag.find(child_link_predicate).next() {
            url = get_href(node);
        }
        embeddings.push(Embedding { url, text })
    }
    return embeddings;
}

pub fn get_instagram_posts(document: &Document) -> Vec<Embedding> {
    let mut embeddings: Vec<Embedding> = Vec::new();
    let child_link_predicate = Child(Name("p"), Name("a"));
    for tag in document.find(Name("blockquote").and(Class("instagram-media"))) {
        if let Some(node) = tag.find(child_link_predicate).next() {
            embeddings.push(Embedding {
                url: get_href(node),
                text: get_sanitized_text(node),
            });
        }
    }
    return embeddings;
}

fn get_href(node: Node) -> String {
    String::from(node.attr("href").unwrap_or(""))
}

fn get_sanitized_text(node: Node) -> String {
    String::from(SPACES_REGEX.replace_all(node.text().trim(), " "))
}
/*
pub(crate) fn get_youtube_videos(document: &Document) -> Vec<Embedding> {
    let mut embeddings: Vec<Embedding> = Vec::new();
    let re = Regex::new(r"\s\s+").unwrap();
    for tag in document.find(Name("iframe").and(Attr("data-provider", "youtube"))) {
        match tag.find(Child(Name("p"), Name("a"))).next() {
            Some(node) => {
                embeddings.push(Embedding {
                    url: String::from(node.attr("src").unwrap_or("")),
                    text:  String::from(node.attr("data-title").unwrap_or("")),
                });
            }
            _ => ()
        }
    }
    return embeddings;
}
*/

#[cfg(test)]
mod tests {
    use select::document::Document;

    use super::*;

    #[test]
    fn get_tweets_telegraph() {
        let document = Document::from(include_str!("extractor/sites/telegraph.co.uk.html"));
        let tweets = get_tweets(&document);

        assert_eq!(tweets.len(), 35);
        let tweet = tweets.get(0).unwrap();
        assert_eq!(tweet.url, "https://twitter.com/lindsaylohan/status/746167573453094912");
        assert_eq!(tweet.text, "One thing for sure the #referendum results are very close, but also showing a difference of opinion across #Britain");
    }

    #[test]
    fn get_instagram_posts_telegraph() {
        let document = Document::from(include_str!("extractor/sites/telegraph.co.uk.html"));

        let instagram_posts = get_instagram_posts(&document);
        assert_eq!(instagram_posts.len(), 5);
        let post = instagram_posts.get(0).unwrap();
        assert_eq!(post.url, "https://www.instagram.com/p/BHA-BtNh3h1/");
        assert_eq!(post.text, "#besmart pay attention and work hard to buy @chanelofficial #remain where's Sunderland? Does Sarah Palin live there? Lol");
    }
    /*
        #[test]
        fn get_youtube_videos() {
            let document = Document::from(include_str!("sites/figaro.fr.html"));

            let videos = get_instagram_posts(&document);
            assert_eq!(videos.len(), 5);
            let video = videos.get(0).unwrap();
            assert_eq!(video.url, "https://www.instagram.com/p/BHA-BtNh3h1/");
            assert_eq!(video.text, "#besmart pay attention and work hard to buy @chanelofficial #remain where's Sunderland? Does Sarah Palin live there? Lol");
        }*/
}