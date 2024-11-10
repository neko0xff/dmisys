use chrono::prelude::*;

pub fn time_cli() {
    let local_time = Local::now();
    let date_format = local_time.format("%Y-%m-%d");
    let time_format = local_time.format("%H:%M:%S");

    println!("Date: {}", date_format);
    println!("Time: {}", time_format);
    println!("");
}
