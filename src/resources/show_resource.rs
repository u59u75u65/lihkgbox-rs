use resources::*;
use resources::web_resource::*;
use resources::common::*;
use caches::common::*;

pub struct ShowResource<'a, T: 'a + Cache> {
    wr: &'a mut WebResource,
    cache: &'a mut Box<T>
}

impl<'a, T: 'a + Cache> ShowResource<'a, T> {
    pub fn new(wr: &'a mut WebResource, cache: &'a mut Box<T>) -> Self {
        ShowResource {
            wr: wr,
            cache: cache            
        }
    }
    fn post_url(&self, postid: &String, page: usize) -> String {
        // https://lihkg.com/api_v1_1/thread/17173/page/1
        let posturl = format!("https://lihkg.com/api_v1_1/thread/{postid}/page/{page}",
                              postid = postid,
                              page = page);
        posturl
    }

}

impl<'a, T: 'a + Cache> Resource for ShowResource<'a, T> {
    fn fetch(&mut self, item: &ChannelItem) -> ChannelItem {
        info!("show resource #fetch");
        match item.extra.clone() {
            Some(o) => {
                match o {
                    ChannelItemType::Show(extra) => {
                        let html_path = format!("data/cache/html/{postid}/", postid = extra.postid);
                        let show_file_name = format!("show_{page}.html", page = extra.page);

                        let postid = extra.postid.clone();

                        let (from_cache, result) = match self.cache.read(&html_path, &show_file_name) {
                            Ok(result) => (true, result),
                            Err(_) => {
                                let posturl = self.post_url(&extra.postid, extra.page);
                                let result = self.wr.get(&posturl);
                                (false, result.into_bytes())
                            }
                        };

                        if !from_cache {
                            let result2 = result.clone();
                            self.cache.write(&html_path, &show_file_name, result2).expect("fail to write cache");
                        }

                        let result_item = ChannelItem {
                            extra: Some(ChannelItemType::Show(ChannelShowItem { postid: postid, page: extra.page })),
                            result: String::from_utf8(result).expect("fail to build result item, reason: invalid string"),
                        };
                        result_item
                    },
                    _ => Default::default()
                }
            }
            None => Default::default()

        }
    }
}
