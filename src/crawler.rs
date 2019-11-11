extern crate select;

use std::string::String;

use select::document::Document;
use select::predicate::{Attr, Name};
use self::select::predicate::Predicate;

use crate::charset::normalise_charset;
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
        Some(i) => {return normalise_charset(cs.get(i+"charset=".len()..).unwrap());}
        _ => {return String::new();}
    }
    /*cs = strings.TrimPrefix(cs, "text/html;charset=")
    cs = strings.TrimPrefix(cs, "text/xhtml;charset=")
    cs = strings.TrimPrefix(cs, "application/xhtml+xml;charset=")
    */
//return NormaliseCharset(cs)
    return cs;
}

#[cfg(test)]
mod tests {
    use std::string::String;

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
        let document = Document::from("<html><head></head></html>");
        assert_eq!(get_charset_from_content_type(String::from("text/html; charset=utf-8")), "UTF-8");
    }
}