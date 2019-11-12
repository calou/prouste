use select::predicate::Predicate;
use super::select::node::Node;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AttrContains<N, V>(pub N, pub V);

impl<'a> Predicate for AttrContains<&'a str, &'a str> {
    fn matches(&self, node: &Node) -> bool {

        return match node.attr(self.0){
            Some(value) => value.contains(self.1),
            _ => false
        }
    }
}
