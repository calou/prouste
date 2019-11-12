
use std::vec::Vec;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use super::select::node::Node;


fn nodes_to_check<'a>(document: Document) -> Vec<Box<Node<'a>>> {
    let mut _nodes = Vec::new();
    for node in document.to_owned().find(Name("p").or(Name("pre")).or(Name("td"))){
        _nodes.push(Box::new(node));
    }
    return _nodes;
}


#[cfg(test)]
mod tests {
    use std::string::String;

    use select::document::Document;

    use super::*;

    #[test]
    fn nodes_to_check_nominal() {
        let document = Document::from("<html><p></p><h1></h1><br/><pre></pre></html>");
        let nodes = nodes_to_check(document);

    }
}