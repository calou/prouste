use std::string::String;

fn add_spaces_between_tags(text: String) -> String {
    return text.replace("</blockquote>", "</blockquote>\n")
        .replace("<img ", "\n<img ")
        .replace("</li>", "</li>\n")
        .replace("</p>", "</p>\n")
        .replace("><", "> <");
}


#[cfg(test)]
mod tests {
    use std::string::String;

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add_spaces_between_tags() {
        let html = "<h1>Title</h1><blockquote>quote</blockquote><li>opts</li><img >";
        let result = "<h1>Title</h1> <blockquote>quote</blockquote>\n<li>opts</li>\n\n<img >";
        assert_eq!(add_spaces_between_tags(String::from(html)), result);
    }
}