extern crate time;
extern crate chrono;
use chrono::prelude::*;

use chrono::{DateTime, TimeZone, NaiveDateTime, UTC};

fn main() {
    println!("unix time");

    let local: DateTime<Local> = Local::now();
    println!("{:?}", local);

    // let dt = UTC.ymd(1970, 1, 1).and_hms(0, 0, 0) + ::time::Duration::seconds(1_480_612_472);

    // let dt = DateTime::<UTC>::from_utc(NaiveDateTime::from_timestamp(1_480_612_472, 0), UTC);
    let dt: DateTime<Local> = Local.timestamp(1_480_612_472, 0);
    println!("{:?}", dt);

}
