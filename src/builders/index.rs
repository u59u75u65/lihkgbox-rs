use std::io::Cursor;

use model::ListTopicItem;
use model::ListTopicTitleItem;
use model::ListTopicAuthorItem;
use model::UrlQueryItem;

use regex::Regex;
use url::Url;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Index {}

impl Index {
    pub fn new() -> Self {
        Index {}
    }
    pub fn build(&mut self, document: &::models::topic::Result) -> Result<Vec<ListTopicItem>, &'static str> {

        let result : Vec<ListTopicItem> = document.response.items.iter().map(|item| {

            let url_query_item = UrlQueryItem {
                channel: item.cat_id.clone(),
                message: item.thread_id.clone()
            };

            let title_item = ListTopicTitleItem {
                url: Default::default(),
                url_query: url_query_item,
                text: item.title.clone(),
                num_of_pages: item.total_page
            };

            let author_item = ListTopicAuthorItem {
                url: Default::default(),
                name: item.user_nickname.clone()
            };

            ListTopicItem {
                title: title_item,
                author: author_item,
                last_replied_date: Default::default(),
                last_replied_time: Default::default(),
                reply_count: item.no_of_reply.clone(),
                rating: Default::default()
            }
        } ).collect::<Vec<_>>();

        Ok(result)
    }
}
