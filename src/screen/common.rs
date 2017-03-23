use rustc_serialize::base64::{self, ToBase64};
use rustc_serialize::hex::FromHex;

use std::fs::File;
use std::fs;
use std::io::{Error, ErrorKind};
use std::io::Read;

use ::hyper::Client;
use ::hyper::header::Connection;

fn imgcat(buffer: Vec<u8>, size_key: &str, size_value: usize) -> String {
    let e = buffer.as_slice().to_base64(base64::STANDARD);
    return String::from(format!("\x1b]1337;File=inline=1;{size_key}={size_value};:{code}\x07", size_key = size_key, size_value = size_value, code = e));
}

pub fn imgcatFromPath(path: &str, width: usize) -> String {
    let mut f = match File::open(path) {
        Err(why) => panic!("couldn't open: {}", why),
        Ok(file) => file,
    };
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);

    return imgcat(buffer, &"width", width);
}

pub fn imgcatFromUrl(url: &str, height: usize) -> String {
    let key = url.to_string().into_bytes().to_base64(base64::URL_SAFE);
    let path = format!("data/img/{file_name}", file_name = key);

    return match File::open(path) {
        // Err(why) => String::from(format!("[{url}]", url = url)),
        Err(why) => String::from(format!("[{code}]", code = why)),
        Ok(mut file) => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer);
            return imgcat(buffer, &"height", height);
        },
    };
}

pub fn reset_screen() {
    print!("{}{}{}", ::termion::clear::All, ::termion::style::Reset, ::termion::cursor::Show);
}

pub fn clear_screen () {
    print!("{}", ::termion::clear::All);
}
