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
    pub category: Category,
    // pub is_pagination: bool,
    pub items: Vec<Item>
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct Category {
    // pub cat_id: usize,
    pub name: String,
    // pub postable: bool
}


#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct Item {
    pub thread_id: String,
    pub cat_id: String,
    pub title: String,
    pub user_nickname: String,
    pub no_of_reply: String,
    pub create_time: usize,
    pub last_reply_time: usize,
    pub total_page: usize
}
