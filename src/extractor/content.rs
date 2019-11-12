
use std::vec::Vec;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use super::select::node::Node;
use std::borrow::Borrow;


fn nodes_to_check<'a>(document: Document) -> Box<Vec<Node<'a>>> {
    let mut nodes = Vec::new();
    for node in document.find(Name("p").or(Name("pre")).or(Name("td"))){
        nodes.push(node);
    }
    return Box::new(nodes);
}


#[cfg(test)]
mod tests {
    use select::document::Document;

    use super::*;

    #[test]
    fn nodes_to_check_nominal() {
        let document = Document::from("<html><p></p><h1></h1><br/><pre></pre></html>");
        let nodes = nodes_to_check(document);
        assert_eq!(nodes.len(), 2);
    }
}