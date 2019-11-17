
extern crate stopwords;
use stopwords::{Language, NLTK, Stopwords};
use std::collections::{HashSet,HashMap};
use unicode_segmentation::UnicodeSegmentation;

lazy_static! {
    static ref HASHMAP: HashMap< &'static str, HashSet<&'static &'static str>> = {
        let mut m = HashMap::new();
        m.insert("en", NLTK::stopwords(Language::English).unwrap().iter().collect());
        m.insert("fr", NLTK::stopwords(Language::French).unwrap().iter().collect());
        m.insert("de", NLTK::stopwords(Language::German).unwrap().iter().collect());
        m.insert("es", NLTK::stopwords(Language::Spanish).unwrap().iter().collect());
        m.insert("sw", NLTK::stopwords(Language::Swedish).unwrap().iter().collect());
        m.insert("it", NLTK::stopwords(Language::Italian).unwrap().iter().collect());
        m.insert("pt", NLTK::stopwords(Language::Portuguese).unwrap().iter().collect());
        m.insert("ru", NLTK::stopwords(Language::Russian).unwrap().iter().collect());
        m.insert("nl", NLTK::stopwords(Language::Dutch).unwrap().iter().collect());
        m.insert("fi", NLTK::stopwords(Language::Finnish).unwrap().iter().collect());
       
        m
    };
}

fn get_stopwords_from_language(lang: &str) -> HashSet<&'static &'static str> {
    return match HASHMAP.get(lang){
        Some(sw) => sw.to_owned(),
        _ => HashSet::new()
    };
}

fn count_max_stopwords(text: &String, _lang: &str, n: usize) -> usize {
    let unicode_words = text.as_str().unicode_words();
    let stopwords: HashSet<_> = get_stopwords_from_language(_lang);
    let mut nb_stopwords: usize = 0;
    for word in unicode_words.into_iter() {
        if nb_stopwords > (n) as usize {
            return nb_stopwords;
        }
        if stopwords.contains(&word.to_ascii_lowercase().as_str()) {
            nb_stopwords += 1;
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
