extern crate kuchiki;

use kuchiki::traits::*;

fn main() {
    println!("html parse");

    let s = "有呢個需要？<img src=\"/assets/faces/normal/clown.gif\" class=\"hkgmoji\" />".to_string();

    let html = format!("<div>{}</div>",s);

    let content = ::kuchiki::parse_html().from_utf8().one(html.as_bytes());

    println!("{:?}", html);
    println!("{:?}", content);
    println!("{:?}", content.descendants());

    let content_elm_option = content.select("div").ok().map_or(None, |mut x| x.next() );

    if content_elm_option.is_none() {
        panic!("div not found");
    }
    let mut content_elm = content_elm_option.unwrap();

    let mut node = content_elm.as_node();
    for child in node.children() {
        println!("{:?}", child);
    }

}
