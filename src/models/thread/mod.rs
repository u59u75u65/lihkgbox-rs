#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct Result {
    pub success: usize,
    pub server_time: usize,
    pub response: Response,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct Response {
    pub thread_id: String,
    pub cat_id: String,
    pub title: String,
    pub user_id: String,
    pub user_nickname: String,
    pub no_of_reply: String,
    pub create_time: usize,
    pub last_reply_time: usize,
    pub total_page: usize,
    pub page: usize,
    pub item_data: Vec<ItemData>,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct ItemData {
    pub post_id: String,
    pub thread_id: String,
    pub user_nickname: String,
    pub reply_time: usize,
    pub msg: String,
}
