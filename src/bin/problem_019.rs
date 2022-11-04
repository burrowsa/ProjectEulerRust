extern crate chrono;
use chrono::prelude::*;
use chrono::Duration;

// Counting Sundays
// You are given the following information, but you may prefer to do some research for yourself.
//
// * 1 Jan 1900 was a Monday.
// * Thirty days has September,
//   April, June and November.
//   All the rest have thirty-one,
//   Saving February alone,
//   Which has twenty-eight, rain or shine.
//   And on leap years, twenty-nine.
// *  A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.
//
// How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

fn problem_019() -> usize {
    let mut i = Utc.ymd(1901, 1, 1);
    let end = Utc.ymd(2000, 12, 31);
    let mut sundays = 0;
    while i <= end {
        if i.weekday() == Weekday::Sun {
            sundays += 1
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
    sundays
}

fn main() {
    print!("{}", problem_019());
}

#[cfg(test)]
mod tests {
    use crate::problem_019;

    #[test]
    fn test_problem_019() {
        let result = problem_019();
        assert_eq!(result, 171);
    }
}
