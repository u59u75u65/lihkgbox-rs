extern crate hyper;
extern crate hyper_native_tls;
extern crate rustc_serialize;

use rustc_serialize::json;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::{Headers, UserAgent};

use std::io::Read;
use std::io::{Error, ErrorKind};

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct ThreadResult {
    pub success: usize,
    pub server_time: usize,
    pub response: ThreadResponse,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct ThreadResponse {
    pub thread_id: usize,
    pub cat_id: usize,
    pub title: String,
    pub user_id: String,
    pub user_nickname: String,
    pub no_of_reply: String,
    pub create_time: usize,
    pub last_reply_time: usize,
    pub page: usize,
    pub item_data: Vec<ThreadResponseItemData>
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct ThreadResponseItemData {
    pub post_id: String,
    pub thread_id: usize,
    pub user_nickname: String,
    pub reply_time: usize,
    pub msg: String
}

fn main() {
    println!("start");

    let url = "https://lihkg.com/api_v1_1/thread/17173/page/1";

    let result = download(&url);

    if result.is_err() {
        panic!("error: {:?}", result.err());
    }

    let html_string = result.unwrap();
    let json: ThreadResult = json::decode(&html_string).expect("fail parse document as json");
    println!("{:?}", json);
}

pub fn download(url: &str) -> Result<String, Error> {
    let mut headers = Headers::new();
    headers.set(UserAgent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_12_3) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2924.87 Safari/537.36".to_owned()));

    let ssl = NativeTlsClient::new().unwrap();
    let connector = HttpsConnector::new(ssl);
    let mut client = Client::with_connector(connector);

    client.set_read_timeout(Some(::std::time::Duration::from_secs(5)));
    client.set_write_timeout(Some(::std::time::Duration::from_secs(5)));

    let result: Result<String, Error> = match client.get(url).headers(headers).send() {
        Ok(mut resp) => {
            let mut s = String::new();
            match resp.read_to_string(&mut s) {
                Ok(size) => Ok(s),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(Error::new(ErrorKind::InvalidData, e)),
    };
    result
}
