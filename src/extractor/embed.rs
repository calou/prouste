use regex::Regex;
use select::document::Document;
use select::predicate::{Name, Predicate, Text};

use super::select::predicate::{Child, Class};

#[derive(PartialEq, Debug, Clone, Default)]
pub(crate) struct Embedding {
    pub(crate) url: String,
    pub(crate) text: String,
}

fn get_tweets(document: &Document) -> Vec<Embedding> {
    let mut embeddings: Vec<Embedding> = Vec::new();
    for tag in document.find(Name("blockquote").and(Class("twitter-tweet"))) {
        let mut text: String = String::default();
        let mut url: String = String::default();
        let re = Regex::new(r"\s\s+").unwrap();
        match tag.find(Name("p")).next() {
            Some(node) => text = String::from(re.replace_all(node.text().trim(), " ")),
            _ => ()
        }
        match tag.find(Child(Name("blockquote"), Name("a"))).next() {
            Some(node) => url = String::from(node.attr("href").unwrap_or("")),
            _ => ()
        }
        embeddings.push(Embedding { url, text })
    }
    return embeddings;
}


fn get_instagram_posts(document: &Document) -> Vec<Embedding> {
    let mut embeddings: Vec<Embedding> = Vec::new();
    let re = Regex::new(r"\s\s+").unwrap();
    for tag in document.find(Name("blockquote").and(Class("instagram-media"))) {
        match tag.find(Child(Name("p"), Name("a"))).next() {
            Some(node) => {
                embeddings.push(Embedding {
                    url: String::from(node.attr("href").unwrap_or("")),
                    text:  String::from(re.replace_all(node.text().trim(), " ")),
                });
            }
            _ => ()
        }
    }
    return embeddings;
}


#[cfg(test)]
mod tests {
    use select::document::Document;

    use super::*;

    #[test]
    fn get_tweets_lemonde() {
        let document = Document::from(include_str!("sites/lemonde.fr.html"));
        let tweets = get_tweets(&document);

        assert_eq!(tweets.len(), 2);
        let tweet = tweets.get(1).unwrap();
        assert_eq!(tweet.url, "//twitter.com/pibzedog/status/1196112642324254720");
        assert_eq!(tweet.text, "La Maison du peuple de la Flèche d’Or a été vidée. La police est partie. Dedans, des vigiles, dehors, la foule d’un… https://t.co/uH0KEcAQ4L");
    }

    #[test]
    fn get_tweets_telegraph() {
        let document = Document::from(include_str!("sites/telegraph.co.uk.html"));
        let tweets = get_tweets(&document);

        assert_eq!(tweets.len(), 35);
        let tweet = tweets.get(0).unwrap();
        assert_eq!(tweet.url, "https://twitter.com/lindsaylohan/status/746167573453094912");
        assert_eq!(tweet.text, "One thing for sure the #referendum results are very close, but also showing a difference of opinion across #Britain");
    }

    #[test]
    fn get_instagram_posts_telegraph() {
        let document = Document::from(include_str!("sites/telegraph.co.uk.html"));

        let instagram_posts = get_instagram_posts(&document);
        assert_eq!(instagram_posts.len(), 5);
        let post = instagram_posts.get(0).unwrap();
        assert_eq!(post.url, "https://www.instagram.com/p/BHA-BtNh3h1/");
        assert_eq!(post.text, "#besmart pay attention and work hard to buy @chanelofficial #remain where's Sunderland? Does Sarah Palin live there? Lol");
    }
}