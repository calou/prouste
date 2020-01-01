use select::predicate::Predicate;

use super::select::node::Node;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttrContains<N, V>(pub N, pub V);

impl<'a> Predicate for AttrContains<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {
        match node.attr(self.0) {
            Some(value) => value.contains(self.1),
            _ => false
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageTag;

impl<'a> Predicate for ImageTag {
    fn matches(&self, node: &Node) -> bool {
        match node.name() {
            Some("link") => {
                if let Some(b) = node.attr("rel") {
                    return b == "image_src";
                }
                return false;
            }
            Some("meta") => {
                if let Some(value) = node.attr("property") {
                    return value == "og:image";
                }
                if let Some(value) = node.attr("name") {
                    return value == "twitter:image" || value == "twitter:image:src";
                }
                return false;
            }
            _ => false
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ImageWithLink();

impl Predicate for ImageWithLink {
    fn matches(&self, node: &Node) -> bool {
        if let Some("a") = node.name() {
            if node.attr("href").is_some() {
                return true;
            }
        }
        false
    }
}