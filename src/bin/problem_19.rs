extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;

fn main() {
    let mut i = Utc.ymd(1901, 1, 1);
    let end = Utc.ymd(2000, 12, 31);
    let mut sundays = 0;
    while i <= end {
        match i.weekday() {
            Weekday::Sun => sundays += 1,
            _ => (),
        }
        i += match i.month() {
            1 => Duration::days(31),
            2 => {
                if (i.year() % 4 == 0) && (i.year() % 400 == 0 || i.year() % 100 != 0) {
                    Duration::days(29)
                } else {
                    Duration::days(28)
                }
            }
            3 => Duration::days(31),
            4 => Duration::days(30),
            5 => Duration::days(31),
            6 => Duration::days(30),
            7 => Duration::days(31),
            8 => Duration::days(31),
            9 => Duration::days(30),
            10 => Duration::days(31),
            11 => Duration::days(30),
            12 => Duration::days(31),
            _ => Duration::days(0),
        };
    }

    print!("{}", sundays);
}
