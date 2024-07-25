use std::ops::Deref;

use select::predicate::Class;

pub fn get_html(word: &str) -> anyhow::Result<String> {
    let body = reqwest::blocking::get(format!("http://dict.youdao.com/search?q={}", word))?;
    assert!(body.status().is_success());

    let text = body.text()?;
    Ok(text)
}

pub fn get_info(html: String) {
    /* let doc = select::document::Document::from(&html);

    for node in doc.find(Class("trans")) {
        dbg!(node.data());
    } */
}
