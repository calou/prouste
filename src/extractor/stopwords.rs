
extern crate stopwords;
use stopwords::{Language, NLTK, Stopwords};
use std::collections::{HashSet,HashMap};
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref HASHMAP: HashMap< &'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("en", stopwords_from_language(Language::English));
        m.insert("fr", stopwords_from_language(Language::French));
        m.insert("de", stopwords_from_language(Language::German));
        m.insert("es", stopwords_from_language(Language::Spanish));
        m.insert("sw", stopwords_from_language(Language::Swedish));
        m.insert("it", stopwords_from_language(Language::Italian));
        m.insert("pt", stopwords_from_language(Language::Portuguese));
        m.insert("ru", stopwords_from_language(Language::Russian));
        m.insert("nl", stopwords_from_language(Language::Dutch));
        m.insert("fi", stopwords_from_language(Language::Finnish));
       
        m
    };
}

//#[inline(always)]
fn stopwords_from_language(lang: Language) -> Vec<&'static str> {
    return match NLTK::stopwords(lang){
        Some(sw) => {
            let mut stopwords = sw.to_vec();
            stopwords.sort();
            let vec = stopwords.clone();
            return vec.clone();
        },
        _ => Vec::new()
    }
}

fn get_stopwords_from_language(lang: &str) -> Vec<&'static str> {
    return match HASHMAP.get(lang){
        Some(sw) => sw.to_vec(),
        _ => Vec::default()
    };
}

#[inline(always)]
fn count_max_stopwords(text: &String, _lang: &str, n: usize) -> usize {
    let lowercased_text = text.as_str().to_ascii_lowercase();
    let unicode_words = lowercased_text.unicode_words();
    let stopwords: Vec<_> = get_stopwords_from_language(_lang);
    let mut nb_stopwords: usize = 0;
    for word in unicode_words.into_iter() {
        let result = stopwords.binary_search(&&word);
        if result.is_ok() {
            nb_stopwords += 1;
            if nb_stopwords > (n) as usize {
                return nb_stopwords;
            }
        }
    }
    return nb_stopwords;
}

pub fn count_stopwords(text: &String, lang: &str) -> usize {
    return count_max_stopwords(text, lang, 999999);
}

pub fn has_more_stopwords_than(text: &String, lang: &str, n: usize) -> bool {
    let number_of_stopwords = count_max_stopwords(text, lang, n);
    return number_of_stopwords >= n;
}
