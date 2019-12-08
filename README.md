
# Prouste - HTML text extraction
[![Build Status](https://travis-ci.com/calou/prouste.svg?branch=master)](https://travis-ci.com/calou/prouste)

Prouse extracts the most relevant texts, title, images... from a HTML page.

```rust
let body = reqwest::get("https://www.rust-lang.org/")?
    .text()?;

let html_extractor = HtmlExtractor::default();
let article = html_extractor.from_string(body).unwrap();
println!("title = {:?}", article.title);
println!("top image = {:?}", article.top_image);
println!("text = {:?}", article.text);
```

# Run example from source
```bash
cargo run --example extract_from_url -- https://www.rust-lang.org/
```