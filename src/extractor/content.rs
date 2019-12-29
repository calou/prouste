use std::vec::Vec;

use indexmap::IndexMap;
use select::document::Document;
use select::predicate::{Name, Predicate};
use unicode_segmentation::UnicodeSegmentation;

use crate::extractor::stopwords::{count_stopwords, has_more_stopwords_than};

use super::select::node::Node;

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

            let current_score = score_per_node.entry(parent_node.index()).or_insert(0);
            *current_score += up_score;

            if let Some(grandparent_node) = parent_node.parent() {
                let node_score = score_per_node.entry(grandparent_node.index()).or_insert(0);
                *node_score += up_score / 2;
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

pub fn get_cleaned_text_and_links(node: Node, _lang: &str) -> (String, Vec<String>) {
    let excluded_nodes = get_removed_nodes(node);

    let mut text = String::with_capacity(200);
    let mut links: Vec<String> = Vec::new();

    node.descendants().into_iter()
        .filter(|n| !excluded_nodes.contains(&n.index()))
        .for_each(|descendant| {
            if descendant.children().count() == 0 {
                text.push_str(descendant.text().as_str());
                if descendant.is(Name("p")) {
                    text.push('\n');
                }
            }

            for l in descendant.find(Name("a")) {
                if let Some(link) = l.attr("href") {
                    links.push(String::from(link));
                }
            }
        });

    return (text, links);
}

fn get_removed_nodes(node: Node) -> Vec<usize> {
    let mut removed_nodes: Vec<usize> = Vec::new();
    let p_tag_predicate = Name("p");
    let td_tag_predicate = Name("td");
    node.children().into_iter().filter(|child| !child.is(p_tag_predicate)).for_each(|child| {
        let child_text = child.text();
        if !is_high_density_link(&child, count_words(&child_text)) {
            removed_nodes.push(child.index());
            for descendant in child.descendants() {
                removed_nodes.push(descendant.index());
            }
        } else {
            let sub_paragraphes = child.find(p_tag_predicate);
            if !child.is(td_tag_predicate) && sub_paragraphes.size_hint().1.unwrap_or(0) == 0 {
                let indexes = get_index_and_descendant_indexes(child);
                removed_nodes.extend(indexes);
            } else {
                for sub_paragraph in sub_paragraphes {
                    if sub_paragraph.text().len() < 25 {
                        let indexes = get_index_and_descendant_indexes(child);
                        removed_nodes.extend(indexes);
                    }
                }
            }
        }
    });
    return removed_nodes;
}

fn get_index_and_descendant_indexes(child: Node) -> Vec<usize> {
    let descendants = child.descendants();
    let (size, _) = descendants.size_hint();
    let mut indexes: Vec<usize> = Vec::with_capacity(size + 1);
    indexes.push(child.index());
    descendants.into_iter().for_each(|descendant| {
        indexes.push(descendant.index());
    });
    indexes
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