use std::time::Duration;

use crate::state::Data;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

// Helper function to convert a `Duration` to a timestamp string
pub fn duration_to_time(duration: Duration) -> String {
    let mut secs = duration.as_secs() % (24 * 3600);
    let hours = secs / 3600;
    secs %= 3600;
    let mins = secs / 60;
    secs %= 60;

    return format!("{}:{:02}:{:02}", hours, mins, secs);
}

pub fn time_to_duration(time: &String) -> Duration {
    let split_str = time
        .split(':')
        .map(|x| x.parse::<u64>().ok().expect(&format!("cannot parse {}", x)))
        .collect::<Vec<_>>();

    let mut secs: u64 = 0;

    match split_str.len() {
        3 => {
            secs = split_str[2];
            secs += split_str[0] * 3600;
            secs += split_str[1] * 60;
        }
        2 => {
            secs = split_str[1];
            secs += split_str[0] * 60;
        }
        1 => {
            secs = split_str[0];
        }
        _ => {}
    }
    Duration::new(secs, 0)
}
