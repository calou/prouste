
# Prouste - HTML text extraction
[![Build Status](https://travis-ci.com/calou/prouste.svg?branch=master)](https://travis-ci.com/calou/prouste)

Prouse extracts the most relevant texts, title, images... from a HTML page.

```rust
let body = reqwest::get("https://www.lemonde.fr/politique/article/2019/12/08/si-la-reforme-des-retraites-est-retiree-edouard-philippe-craint-qu-elle-soit-tres-brutale-plus-tard_6022069_823448.html")?
    .text()?;

let html_extractor = HtmlExtractor::default();
let article = html_extractor.from_string(body).unwrap();
println!("title = {:?}", article.title);
println!("top image = {:?}", article.top_image);
println!("text = {:?}", article.text);
```