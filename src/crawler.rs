extern crate encoding;
extern crate select;

use std::string::String;

use encoding::all::UTF_8;
use encoding::label::encoding_from_whatwg_label;
use select::document::Document;
use select::predicate::{Attr, Name};

use crate::article::Article;
use crate::charset::charset;

use self::encoding::{DecoderTrap, EncoderTrap, Encoding};
use self::select::predicate::Predicate;
use crate::extractor::extractor::get_title;

pub fn add_spaces_between_tags(text: String) -> String {
    return text.replace("<img ", "\n<img ")
        .replace("</blockquote>", "</blockquote>\n")
        .replace("</li>", "</li>\n")
        .replace("</p>", "</p>\n")
        .replace("><", "> <");
}

pub fn get_content_type(document: Document) -> String {
    // <meta http-equiv="Content-Type" content="text/html; charset=utf-8" />
    let mut content_type: &str = "";
    for node in document.find(Name("meta").and(Attr("http-equiv", "Content-Type"))) {
        content_type = match node.attr("content") {
            Some(str) => str,
            _ => content_type
        }
    }
    return String::from(content_type);
}

pub fn get_charset_from_content_type(content_type: String) -> String {
    let cs = content_type.to_ascii_lowercase();
    let idx = cs.find("charset=");
    match idx {
        Some(i) => { return charset::normalize(cs.get(i + "charset=".len()..).unwrap()); }
        _ => { return cs; }
    }
}

// GetCharset returns a normalised charset string extracted from the meta tags
pub fn get_charset(document: Document) -> String {
    let ct = get_content_type(document.to_owned());

    if "" != ct && ct.to_ascii_lowercase().contains("charset") {
        return get_charset_from_content_type(ct);
    }

    // <meta charset="utf-8">
    for node in document.find(Name("meta")) {
        if node.attr("charset").is_some() {
            return charset::normalize(node.attr("charset").unwrap());
        }
    }
    return String::new();
}

// Preprocess fetches the HTML page if needed, converts it to UTF-8 and applies
// some text normalisation to guarantee better results when extracting the content
pub fn pre_process(raw_html: String) -> Option<Document> {
    if raw_html == "" {
        return None;
    }
    let sanitized_html = add_spaces_between_tags(raw_html);
    let mut document = Document::from(sanitized_html.to_owned().as_str());
    let cs = get_charset(document.to_owned());
    if "" != cs && "UTF-8" != cs {
        // the net/html parser and goquery require UTF-8 data
        let encoding = encoding_from_whatwg_label(cs.as_str()).unwrap();

        let result = encoding.encode(sanitized_html.as_str(), EncoderTrap::Ignore);
        let reencoded_html = UTF_8.decode(&result.unwrap(), DecoderTrap::Ignore).unwrap();
        document = Document::from(reencoded_html.as_str());
    }
    return Some(document);
}

fn crawl(raw_html: String) -> (Article, String) {
    let option = pre_process(raw_html);
    return match option {
        Some(html) => {
            let document = Document::from(html);
            let mut article = Article::new();
            article.title = get_title(document);
            return (article, String::new());
        },
        _ => (Article::new(), String::from("Impossible to pre-process html"))
    };
}

#[cfg(test)]
mod tests {
    use std::string::String;
    use std::fs;
    use crate::article::Article;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_spaces_between_tags() {
        let html = "<h1>Title</h1><blockquote>quote</blockquote><li>opts</li><img >";
        let result = "<h1>Title</h1> <blockquote>quote</blockquote>\n<li>opts</li>\n\n<img >";
        assert_eq!(add_spaces_between_tags(String::from(html)), result);
    }

    #[test]
    fn test_get_content_type_simple() {
        let document = Document::from("<html><head><meta http-equiv=\"Content-Type\" content=\"text/html; charset=utf-8\" /></head></html>");
        assert_eq!(get_content_type(document), "text/html; charset=utf-8");
    }

    #[test]
    fn test_get_content_type_no_content_type() {
        let document = Document::from("<html><head></head></html>");
        assert_eq!(get_content_type(document), "");
    }

    #[test]
    fn test_get_charset_from_content_type() {
        assert_eq!(get_charset_from_content_type(String::from("text/html; charset=utf-8")), "UTF-8");
    }

    #[test]
    fn test_get_charset() {
        assert_eq!(get_charset(Document::from("<html><head><meta charset=\"UTF8\"></head></html>")), "UTF-8");
        assert_eq!(get_charset(Document::from("<html><head><meta test=\"\"><meta charset=\"dummy\"></head></html>")), "DUMMY");
    }


    #[test]
    fn test_crawl() {
        let raw_html = fs::read_to_string("src/extractor/sites/abcnews.go.com.html")
            .expect("Something went wrong reading the file");

        let (article, _) = crawl(raw_html);
        assert_eq!(article.title, "New Jersey Devils Owner Apologizes After Landing Helicopter in Middle of Kids' Soccer Game Forces Cancellation - ABC News");
    }
}