extern crate chrono;
extern crate serde_json;

use chrono::{ FixedOffset, Utc };

fn main() -> Result<(), std::io::Error> {

    let args: Vec<String> = std::env::args().collect();

    let fmt_str = &args[1];
    let offset = &args[2];
    let offset_sec: i32 = offset.parse().unwrap();

    let date = Utc::now();
    let date_fixed = FixedOffset::east(offset_sec);
    let date_tz = date.with_timezone(&date_fixed);
    let date_str = date_tz.format(fmt_str).to_string();

    let json = serde_json::json!({
        "version": 1,
        "full_text": date_str,
    });

    print!("{}", json);

    Ok(())
}
