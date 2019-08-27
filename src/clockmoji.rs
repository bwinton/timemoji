use std::ops::Add;
use chrono::{DateTime, Duration, Timelike, Local};

const CLOCKS: [&str; 24] = [
    "🕛", "🕧", "🕐", "🕜", "🕑", "🕝", "🕒", "🕞", "🕓", "🕟", "🕔", "🕠",
    "🕕", "🕡", "🕖", "🕢", "🕗", "🕣", "🕘", "🕤", "🕙", "🕥", "🕚", "🕦"
];
const DURATION: usize = 60 * 30;

pub fn get_emoji(time: &Option<DateTime<Local>>) -> &'static str {
    let time = time.unwrap_or_else(Local::now);
    let time = time.add(Duration::minutes(15));
    let seconds = time.time().num_seconds_from_midnight() as usize;
    let index = seconds / DURATION;

    CLOCKS[index % CLOCKS.len()]
}

fn main() {
    println!("{}", get_emoji(&None));
}
