use std::borrow::BorrowMut;
use indexmap::IndexMap;
use std::vec::Vec;
use select::document::Document;
use select::predicate::{Name, Predicate};
use unicode_segmentation::UnicodeSegmentation;
use crate::extractor::stopwords::{count_stopwords, has_more_stopwords_than};
use super::select::node::Node;

const DEFAULT_NODE_SCORE: usize = 0;

pub fn get_top_node<'a>(document: &'a Document, lang: &'a str) -> Option<Node<'a>> {
    let mut top_node: Option<usize> = None;
    let starting_boost: f32 = 1.0;
    let mut i: usize = 0;
    let mut nodes_with_text: IndexMap<usize, String> = IndexMap::new();
    let mut score_per_node: IndexMap<usize, usize> = IndexMap::new();
    for node in document.find(Name("p").or(Name("pre")).or(Name("td"))) {
        let node_text = node.text();
        let text_words_count = count_words(&node_text);
        if has_more_stopwords_than(&node_text, lang, 2) && !is_high_density_link(&node, text_words_count) {
            nodes_with_text.insert(node.index(), node_text);
        }

        let nodes_with_text_count = nodes_with_text.len();

        let bottom_negative_scoring = nodes_with_text_count / 4;
        for (node_index, text) in nodes_with_text.iter() {
            let mut boost_score: f32 = 0.0;
            if is_boostable(&node, lang) {
                boost_score = 50.0 / starting_boost;
            }
            if nodes_with_text_count > 15 {
                let booster: i32 = (bottom_negative_scoring + i - nodes_with_text_count) as i32;
                if booster >= 0 {
                    let x = i32::pow(booster, 2);
                    if x > 40 {
                        boost_score = 5.0;
                    } else {
                        boost_score = -1.0 * (x) as f32;
                    }
                }
            }

            let up_score = count_stopwords(text, lang) + (boost_score) as usize;
            let parent_node = document.nth(*node_index).unwrap().parent().unwrap();

            let index = parent_node.index();
            let new_score = calculate_node_score_in_map(score_per_node.borrow_mut(), index, up_score);
            score_per_node.insert(index, new_score);

            let grandparent_node = parent_node.parent();
            match grandparent_node {
                Some(_gp) => {
                    let index = _gp.index();
                    let new_score = calculate_node_score_in_map(score_per_node.borrow_mut(), index, up_score / 2);
                    score_per_node.insert(index, new_score);
                }
                _ => ()
            }
            i += 1;
        }

        let mut node_top_score: usize = 0;
        for (n, score) in score_per_node.iter() {
            if *score > node_top_score {
                top_node = Some(*n);
                node_top_score = *score;
            }
        }
    }
    return match top_node {
        Some(idx) => Node::new(document, idx),
        _ => None
    };
}

fn is_boostable(node: &Node, lang: &str) -> bool {
    let mut sibling_distance: u32 = 0;
    let mut sibling_option = node.next();
    while sibling_option.is_some() {
        let sibling = sibling_option.unwrap();
        if sibling.name().unwrap_or("") == "p" {
            let sibling_text = sibling.text();
            if has_more_stopwords_than(&sibling_text, lang, 5) {
                return true;
            }
        }
        sibling_option = sibling.next();
        sibling_distance += 1;
        if sibling_distance >= 3 {
            return false;
        }
    }
    return false;
}

fn is_high_density_link(node: &Node, text_words_count: usize) -> bool {
    if text_words_count == 0 {
        return true;
    }
    let mut link_words_count: usize = 0;
    let mut links_count = 0;
    for link in node.find(Name("a")) {
        let link_text = link.text();
        link_words_count += count_words(&link_text);
        links_count += 1;
    }
    let score = (links_count * link_words_count) / text_words_count;
    return score > 1;
}
#[inline]
fn count_words(text: &String) -> usize {
    return text.as_str().unicode_words().count();
}

#[inline]
fn calculate_node_score_in_map(score_per_node: &mut IndexMap<usize, usize>, node_index: usize, increment: usize) -> usize {
    let current_score = score_per_node.get(&node_index).unwrap_or(&DEFAULT_NODE_SCORE);
    return current_score + increment;
}

pub fn get_cleaned_text_and_links(node: Node, _lang: &str) -> (String, Vec<String>) {
    let excluded_nodes = get_removed_nodes(node);

    let mut text = String::with_capacity(200);
    let mut links: Vec<String> = Vec::new();
    for descendant in node.descendants() {
        if !excluded_nodes.contains(&descendant.index()) {
            if descendant.children().count() == 0 {
                text.push_str(descendant.text().as_str());
                if descendant.is(Name("p")) {
                    text.push('\n');
                }
            }

            for l in descendant.find(Name("a")) {
                match l.attr("href") {
                    Some(l) => links.push(String::from(l)),
                    _ => ()
                };
            }
        }
    }
    return (text, links);
}

fn get_removed_nodes(node: Node) -> Vec<usize> {
    let mut removed_nodes: Vec<usize> = Vec::new();
    let p_tag_predicate = Name("p");
    let td_tag_predicate = Name("td");
    for child in node.children() {
        if !child.is(p_tag_predicate) {
            let child_text = child.text();
            if !is_high_density_link(&child, count_words(&child_text)) {
                removed_nodes.push(child.index());
                for descendant in child.descendants() {
                    removed_nodes.push(descendant.index());
                }
            } else {
                let sub_paragraphes = child.find(p_tag_predicate);
                if !child.is(td_tag_predicate) && sub_paragraphes.size_hint().1.unwrap_or(0) == 0 {
                    removed_nodes.push(child.index());
                    for descendant in child.descendants() {
                        removed_nodes.push(descendant.index());
                    }
                } else {
                    for sub_paragraph in sub_paragraphes {
                        if sub_paragraph.text().len() < 25 {
                            removed_nodes.push(sub_paragraph.index());
                            for descendant in sub_paragraph.descendants() {
                                removed_nodes.push(descendant.index());
                            }
                        }
                    }
                }
            }
        }
    }
    return removed_nodes;
}


#[cfg(test)]
mod tests {
    use select::document::Document;

    use super::*;

    #[test]
    fn test_get_cleaned_text_and_links() {
        let document = Document::from(include_str!("sites/theguardian.com.html"));
        let option = get_top_node(&document, "en").unwrap();
        let (text, _links) = get_cleaned_text_and_links(option, "en");
        println!("{}", text);
    }

    #[test]
    fn test_get_cleaned_text_and_links_techcrunch() {
        let document = Document::from(include_str!("sites/techcrunch.com.html"));
        let option = get_top_node(&document, "en").unwrap();
        let (text, _links) = get_cleaned_text_and_links(option, "en");
        println!("{}", text);
    }

    #[test]
    fn test_get_top_node_simple() {
        let document = Document::from("<html><body><div><p>This is a paragraph</p><h1></h1><br/><pre>Paris</pre></div><span></span></html>");
        assert_eq!(get_top_node(&document, "en").unwrap().name().unwrap(), "div");
    }

    #[test]
    fn test_removed_nodes() {
        let document = Document::from(include_str!("sites/theguardian.com.html"));
        let node = get_top_node(&document, "en").unwrap();
        let removed_nodes = get_removed_nodes(node);
        for i in removed_nodes.iter() {
            println!("Removed node : {}", document.nth(*i).unwrap().text());
        }
    }

    #[test]
    fn test_get_top_node_nominal() {
        let document = Document::from(include_str!("sites/theguardian.com.html"));
        let node = get_top_node(&document, "en").unwrap();
        assert_eq!(node.name().unwrap(), "div");
        //println!("{}", node.text());
    }

    #[test]
    fn test_has_more_stopwords_than() {
        let text = String::from("I live in London in England");
        assert!(has_more_stopwords_than(&text, "en", 2));
    }

    #[test]
    fn test_count_stopwords() {
        let text = String::from("I live in London in England");
        assert_eq!(count_stopwords(&text, "en"), 3);
    }
}