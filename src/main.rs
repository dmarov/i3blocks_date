extern crate chrono;
extern crate serde_json;

use chrono::{ DateTime, FixedOffset, Utc };

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let fmt_str = &args[1];
    let offset = &args[2];
    let offset_sec: i32 = offset.parse().unwrap();

    let date: DateTime<Utc> = Utc::now();
    let f = FixedOffset::east(offset_sec);
    let n = date.with_timezone(&f);
    let date_str = n.format(fmt_str).to_string();

    let json = serde_json::json!({
        "version": 1,
        "full_text": date_str,
    });

    print!("{}", json);
}
