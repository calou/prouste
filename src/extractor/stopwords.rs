extern crate stopwords;
use stopwords::{Language, NLTK, Stopwords};
use std::collections::HashSet;

use unicode_segmentation::UnicodeSegmentation;

fn get_stopwords_from_language(lang: &str) -> HashSet<&&str> {
    return match lang {
        "en" => NLTK::stopwords(Language::English).unwrap().iter().collect(),
        "fr" => NLTK::stopwords(Language::French).unwrap().iter().collect(),
        "de" => NLTK::stopwords(Language::German).unwrap().iter().collect(),
        "es" => NLTK::stopwords(Language::Spanish).unwrap().iter().collect(),
        "sw" => NLTK::stopwords(Language::Swedish).unwrap().iter().collect(),
        "it" => NLTK::stopwords(Language::Italian).unwrap().iter().collect(),
        "pt" => NLTK::stopwords(Language::Portuguese).unwrap().iter().collect(),
        "ru" => NLTK::stopwords(Language::Russian).unwrap().iter().collect(),
        "nl" => NLTK::stopwords(Language::Dutch).unwrap().iter().collect(),
        "fi" => NLTK::stopwords(Language::Finnish).unwrap().iter().collect(),
        _ => HashSet::new()
    };
}


fn count_max_stopwords(text: &String, _lang: &str, n: usize) -> usize {
    let unicode_words = text.as_str().unicode_words();
    let stopwords: HashSet<_> = get_stopwords_from_language("en");
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
