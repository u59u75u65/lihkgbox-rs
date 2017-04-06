use std::io::Cursor;

use kuchiki::NodeRef;
use kuchiki::NodeDataRef;
use kuchiki::NodeData;
use kuchiki::ElementData;

use model::ShowItem;
use model::ShowReplyItem;
use model::UrlQueryItem;
use reply_model::*;

use regex::Regex;
use url::Url;
use std::collections::HashMap;

use kuchiki::traits::TendrilSink;

use chrono::prelude::*;

#[derive(Clone)]
pub struct Show { }

impl Show {
    pub fn new() -> Self {
        Show {}
    }

    pub fn build(&mut self, document: &::models::thread::Result,  url: &str) -> Result<ShowItem, &'static str> {

        let url_query_item = UrlQueryItem {
            channel: document.response.cat_id.clone(),
            message: document.response.thread_id.clone()
        };

        let replies = {
            let replies_option = self.parse_show_reply_items(&document);

            if replies_option.is_err() {
                let e = replies_option.err().unwrap();
                error!("{:?}", e);
                return Err(e);
            }

            replies_option.unwrap()
        };

        let show_item = ShowItem {
            url_query: url_query_item,
            replies: replies,
            page: document.response.page,
            max_page: document.response.total_page,
            reply_count: document.response.no_of_reply.clone(),
            title: document.response.title.clone(),
        };

        Ok(show_item)        
    }

}


impl Show {

    fn parse_show_reply_items(&self, document: &::models::thread::Result) -> Result<Vec<ShowReplyItem>, &'static str>  {

        let show_replies = document.response.item_data.iter().enumerate().map(reply_items_handler).collect::<Vec<_>>();

        let err_show_reply_option = show_replies.iter().filter(|x| x.is_err()).next();

        if err_show_reply_option.is_some() {
            return Err(&"fail to parse show reply items, reason: 'error show reply item' was found");
        }

        let result = show_replies.iter().map(|x| x.clone().unwrap() ).collect::<Vec<_>>();


        Ok(result)
    }
}


fn reply_items_handler((index,item_data): (usize, &::models::thread::ItemData)) -> Result<ShowReplyItem, &'static str> {

    let msg = item_data.msg.clone();

    let html = format!("<div>{}</div>",msg);

    let content = ::kuchiki::parse_html().from_utf8().one(html.as_bytes());

    info!("{:?}", content);

    let content_elm_option = content.select("div").ok().map_or(None, |mut x| x.next() );

    if content_elm_option.is_none() {
        return Err("fail to parse show reply item, reason: 'content_elm' not found");
    }
    let mut content_elm = content_elm_option.unwrap();

    let mut vec: Vec<NodeType> = Vec::new();

    vec = recursive(content_elm.as_node());

    let dt: DateTime<Local> = Local.timestamp(item_data.reply_time as i64, 0);

    let dt_str = dt.format("%d/%m/%Y %H:%M").to_string();

    Ok(
        ShowReplyItem {
            userid: Default::default(),
            username: item_data.user_nickname.clone(),
            content: item_data.msg.clone(),
            body: vec,
            published_at: dt_str
        }
    )
}


fn recursive(elm: &NodeRef) -> Vec<NodeType> {

    let mut vec: Vec<NodeType> = Vec::new();

    for (index, child) in elm.children().enumerate() {
        // println!("[{}] => {:?}", index, child);
        let node_data = child.data().clone();

        match node_data {
            NodeData::Element(element_data) => {
                // println!("[{}] => [ELEMENT] => {:?}", index, element_data);
                // println!("[{}] => [ELEMENT] => {:?}", index, child);

                if element_data.name.local.trim().eq("blockquote") {

                    // println!("[{}] => [ELEMENT] => {:?}", index, child.children());
                    let subvec = recursive(&child);
                    let node = NodeType::BlockQuote(BlockQuoteNode { data: subvec });
                    vec.push(node);
                } else if element_data.name.local.trim().eq("br") {
                    let node = NodeType::Br(BrNode {});
                    vec.push(node);
                } else if element_data.name.local.trim().eq("img") {

                    let attrs = (&element_data.attributes).borrow();
                    let url = attrs.get("src").unwrap_or("");
                    let alt = attrs.get("class").unwrap_or("");
                    let node = NodeType::Image(ImageNode { data: url.to_string(), alt: alt.to_string() });
                    vec.push(node);

                } else {
                    // println!("[{}] => [ELEMENT] => {:?}", index, child);
                    let mut subvec = recursive(&child);
                    vec.append(&mut subvec);
                }
            }
            NodeData::Text(rc) => {
                // println!("[{}] => [TEXT] => {:?}", index, rc);
                let d = rc.clone();
                let b = d.borrow();

                let s = b.trim().to_string();

                if s == "\n" {
                    // s = "\\n".to_string()
                    continue;
                }

                let node = NodeType::Text(TextNode { data: s });
                vec.push(node);
            }
            _ => {}
        }
    }
    vec
}
