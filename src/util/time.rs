use chrono::{Datelike, Duration, Local, Timelike};

pub fn now() -> u64 {
    let now = Local::now();

    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = now.hour();
    let minute = now.minute();
    let second = now.second();

    format!("{:04}{:02}{:02}{:02}{:02}{:02}", year, month, day, hour, minute, second).parse().expect("")
}

pub fn today_start_moment() -> u64 {
    let now = Local::now();

    let year = now.year();
    let month = now.month();
    let day = now.day();
    let hour = 0;
    let minute = 0;
    let second = 0;

    format!("{:04}{:02}{:02}{:02}{:02}{:02}", year, month, day, hour, minute, second).parse().expect("")
}

pub fn n_day_before_start_moment(n: u8) -> u64 {
    let moment = Local::now() - Duration::days(n as i64);

    let year = moment.year();
    let month = moment.month();
    let day = moment.day();
    let hour = 0;
    let minute = 0;
    let second = 0;

    format!("{:04}{:02}{:02}{:02}{:02}{:02}", year, month, day, hour, minute, second).parse().expect("")
}