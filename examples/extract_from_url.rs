#![deny(warnings)]
extern crate reqwest;

use std::env;
use prouste::html::HtmlExtractor;

fn main()  -> Result<(), Box<dyn std::error::Error>>{
    let url = env::args().skip(1).next();
    let body = reqwest::get(url.unwrap().as_str())?
        .text()?;

    let html_extractor = HtmlExtractor::default();
    let article = html_extractor.from_string(body).unwrap();
    println!("title = {:?}", article.title);
    println!("top image = {:?}", article.top_image);
    println!("text = {:?}", article.text);

    Ok(())
}