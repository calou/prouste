extern crate select;

use std::string::String;
//use select::predicate::{Attr, Name};

pub mod extractor {
    use select::document::Document;

    pub fn get_title(document: Document) -> String {
        return String::new();
    }


    #[cfg(test)]
    mod tests {
        use std::string::String;
        use super::*;

        #[test]
        fn test_get_title_abcnews() {
            let document = Document::from(include_str!("sites/abcnews.go.com.html"));
            assert_eq!(get_title(document), "New Jersey Devils Owner Apologizes After Landing Helicopter in Middle of Kids' Soccer Game Forces Cancellation - ABC News");
        }
    }
}