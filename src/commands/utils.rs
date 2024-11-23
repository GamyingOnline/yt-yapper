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
