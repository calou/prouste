use select::predicate::Predicate;

use super::select::node::Node;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttrContains<N, V>(pub N, pub V);

impl<'a> Predicate for AttrContains<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        return match node.attr(self.0) {
            Some(value) => value.contains(self.1),
            _ => false
        };
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageTag;

impl<'a> Predicate for ImageTag {
    fn matches(&self, node: &Node) -> bool {
        return match node.name() {
            Some("link") => {
                return match node.attr("rel") {
                    Some(b) => b == "image_src",
                    _ => false
                };
            }
            Some("meta") => {
                match node.attr("property") {
                    Some(value) => {
                        return value == "og:image";
                    }
                    _ => ()
                };
                match node.attr("name") {
                    Some(value) => {
                        return value == "twitter:image" || value == "twitter:image:src";
                    }
                    _ => ()
                };
                return false;
            }
            _ => false
        };
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageWithLink();

impl Predicate for ImageWithLink {
    fn matches(&self, node: &Node) -> bool {
        if let Some("a") = node.name() {
            if let Some(_) = node.attr("href"){
                return true;
            }
        }
        return false;
    }
}
/*
let tag_name_predicate = Name("link").or("meta");
let attribute_predicate = Attr("rel", "image_src")
.or(Attr("name", "twitter:image"))
.or(Attr("property", "og:image"));
let predicate = tag_name_predicate.and(attribute_predicate);
*/