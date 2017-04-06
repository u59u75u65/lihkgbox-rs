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
pub struct ThreadLatestResult {
    pub success: usize,
    pub server_time: usize,
    pub response: ThreadLatestResponse,
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct ThreadLatestResponse {
    pub category: ThreadLatestResponseCategory,
    // pub is_pagination: bool,
    pub items: Vec<ThreadLatestResponseItem>
}

#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct ThreadLatestResponseCategory {
    // pub cat_id: usize,
    pub name: String,
    // pub postable: bool
}


#[derive(Debug)]
#[derive(RustcDecodable)]
#[derive(RustcEncodable)]
#[derive(Clone)]
#[derive(Default)]
pub struct ThreadLatestResponseItem {
    pub thread_id: usize,
    pub cat_id: usize,
    pub title: String,
    pub user_nickname: String,
    pub no_of_reply: String,
    pub create_time: usize,
    pub last_reply_time: usize,
    pub total_page: usize
}

fn main() {
    println!("start");

    let url = "https://lihkg.com/api_v1_1/thread/latest?cat_id=1&page=1&count=50";

    let result = download(&url);

    if result.is_err() {
        panic!("error: {:?}", result.err());
    }

    let html_string = result.unwrap();
    let json: ThreadLatestResult = json::decode(&html_string).expect("fail parse document as json");
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
