

use std::borrow::Borrow;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::vec::Vec;

use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use crate::extractor::stopwords::{has_more_stopwords_than, count_stopwords};

use unicode_segmentation::UnicodeSegmentation;

use super::select::node::Node;

fn get_top_node<'a>(document: &'a Document, lang: &'a str) -> Option<Node<'a>> {
    let mut top_node: Option<usize> = None;
    let mut starting_boost: f32 = 1.0;
    let mut i: usize = 0;
    let mut nodes_with_text: HashMap<usize, String> = HashMap::new();
    let mut score_per_node: HashMap<usize, usize> = HashMap::new();
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
                println!("booster {}", booster);
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
            let parentNode = document.nth(*node_index).unwrap().parent().unwrap();
            update_node_in_map(&mut score_per_node, parentNode.index(), up_score);

            let grandparent_node = parentNode.parent();
            match grandparent_node {
                Some(gp) => {
                    update_node_in_map(&mut score_per_node, parentNode.index(), up_score / 2);
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

fn count_words(text: &String) -> usize {
    return text.as_str().unicode_words().count();
}

fn update_node_in_map(score_per_node: &mut HashMap<usize, usize>, node_index: usize, increment: usize) {
    let default_value: usize = 0;
    let mut current_score = score_per_node.get(&node_index).unwrap_or(&default_value);
    score_per_node.insert(node_index, current_score + increment);
}

#[cfg(test)]
mod tests {
    use select::document::Document;

    use super::*;

    #[test]
    fn test_get_top_node_simple() {
        let document = Document::from("<html><body><div><p>This is a paragraph</p><h1></h1><br/><pre>Paris</pre></div><span></span></html>");
        assert_eq!(get_top_node(&document, "en").unwrap().name().unwrap(), "div");
    }

    #[test]
    fn test_get_top_node_nominal() {
        let document = Document::from(include_str!("sites/theguardian.com.html"));
        let node = get_top_node(&document, "en").unwrap();
        assert_eq!(node.name().unwrap(), "div");
        println!("{}", node.text());
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