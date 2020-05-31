#[macro_use] extern crate quick_error;
use chrono::prelude::*;
use std::{
    time::Duration,
    str::Chars,
    iter::Peekable,
};

quick_error! {
    #[derive(Debug)]
    pub enum ParseError {
        Format(m: String) {
            display("Format error: {}", m)
        }
    }
}

#[derive(Debug)]
enum SimpleDateFormatPart {
    Ear,
    Year(usize),
    Month(usize),
    Day(usize),
    HourLower(usize),
    HourUpper(usize),
    Minute(usize),
    Second(usize),
    Millis(usize),
    AmPm(usize),
    WeekDay(usize),
    YearDay(usize),
    Zone(usize),
    LiteralChar(char),
    Literal(String),
}

pub fn format_human(d: Duration) -> String {
    let mut ret = vec![];
    let millis = d.as_millis();
    if millis == 0 {
        return "0ms".into();
    }
    let return_ret = |mut v: Vec<String>| {
        v.reverse();
        v.join(" ")
    };
    let left_millis = millis % 1000;
    if left_millis > 0 {
        ret.push(format!("{}ms", left_millis))
    }
    let secs = millis / 1000;
    if secs == 0 { return return_ret(ret); }
    let left_secs = secs % 60;
    if left_secs != 0 || !ret.is_empty() {
        ret.push(format!("{}s", left_secs));
    }
    let mins = secs / 60;
    if mins == 0 { return return_ret(ret); }
    let left_mins = mins % 60;
    if left_mins != 0 || !ret.is_empty() {
        ret.push(format!("{}min", left_mins));
    }
    let hours = mins / 60;
    if hours == 0 { return return_ret(ret); }
    let left_hours = hours % 24;
    if left_hours != 0 || !ret.is_empty() {
        ret.push(format!("{}hour", left_hours));
    }
    let days = hours / 24;
    if days != 0 {
        ret.push(format!("{}day{}", days, if days == 1 { "" } else { "s" }));
    }
    return_ret(ret)
}

#[derive(Debug)]
pub struct SimpleDateFormat {
    parts: Vec<SimpleDateFormatPart>,
}

impl SimpleDateFormat {

    pub fn new(f: &str) -> Result<SimpleDateFormat, ParseError> {
        fmt(f)
    }

    pub fn format<Tz>(&self, date_time: &DateTime<Tz>) -> String where Tz: TimeZone {
        let mut ret = String::with_capacity(512);

        for part in &self.parts {
            date_time.timezone();
            match part {
                SimpleDateFormatPart::Ear => ret.push_str("AD"), // ?
                SimpleDateFormatPart::LiteralChar(c) => ret.push(*c),
                SimpleDateFormatPart::Literal(s) => ret.push_str(s),
                SimpleDateFormatPart::Year(cnt) => ret.push_str(&format_year(date_time.year(), *cnt)),
                SimpleDateFormatPart::Month(cnt) => if *cnt <= 2 {
                    ret.push_str(&format_str(date_time.month() as i32, *cnt))
                } else {
                    ret.push_str(format_month(date_time.month(), *cnt));
                },
                SimpleDateFormatPart::Day(cnt) => ret.push_str(&format_str(date_time.day() as i32, *cnt)),
                SimpleDateFormatPart::HourLower(cnt) => ret.push_str(&format_str(date_time.hour12().1 as i32, *cnt)),
                SimpleDateFormatPart::HourUpper(cnt) => ret.push_str(&format_str(date_time.hour() as i32, *cnt)),
                SimpleDateFormatPart::Minute(cnt) => ret.push_str(&format_str(date_time.minute() as i32, *cnt)),
                SimpleDateFormatPart::Second(cnt) => ret.push_str(&format_str(date_time.second() as i32, *cnt)),
                SimpleDateFormatPart::Millis(cnt) => ret.push_str(&format_str((date_time.nanosecond() / 1_000_000) as i32, *cnt)),
                SimpleDateFormatPart::AmPm(cnt) => ret.push_str(format_ampm(date_time.hour12().0, *cnt)),
                SimpleDateFormatPart::WeekDay(cnt) => ret.push_str(format_week_day(date_time.weekday(), *cnt)),
                SimpleDateFormatPart::YearDay(cnt) => ret.push_str(&format_str(date_time.ordinal() as i32, *cnt)),
                SimpleDateFormatPart::Zone(cnt) => ret.push_str(&format_zone(date_time, *cnt)),
            }
        }
        ret
    }
}

pub fn fmt(f: &str) -> Result<SimpleDateFormat, ParseError> {
    let mut parts = vec![];

    let mut is_in_quotation_mark = false;
    let mut literal = String::new();
    let mut chars = f.chars().peekable();
    while let Some(c) = chars.next() {
        if is_in_quotation_mark && c != '\'' {
            literal.push(c);
            continue;
        }
        match c {
            '\'' => if is_in_quotation_mark {
                if let Some('\'') = chars.peek() {
                    literal.push(c);
                    chars.next(); // eat '\''
                } else {
                    is_in_quotation_mark = false;
                    if literal.is_empty() {
                        literal.push('\'');
                    }
                    parts.push(SimpleDateFormatPart::Literal(literal));
                    literal = String::new();
                }
            } else {
                is_in_quotation_mark = true;
            },
            ',' | '.' | ':' | '-' | ' ' | '/' => parts.push(SimpleDateFormatPart::LiteralChar(c)),
            'G' => parts.push(SimpleDateFormatPart::Ear),
            'y' => parts.push(SimpleDateFormatPart::Year(get_all_chars(c, &mut chars))),
            'M' => parts.push(SimpleDateFormatPart::Month(get_all_chars(c, &mut chars))),
            'd' => parts.push(SimpleDateFormatPart::Day(get_all_chars(c, &mut chars))),
            'h' => parts.push(SimpleDateFormatPart::HourLower(get_all_chars(c, &mut chars))),
            'H' => parts.push(SimpleDateFormatPart::HourUpper(get_all_chars(c, &mut chars))),
            'm' => parts.push(SimpleDateFormatPart::Minute(get_all_chars(c, &mut chars))),
            's' => parts.push(SimpleDateFormatPart::Second(get_all_chars(c, &mut chars))),
            'S' => parts.push(SimpleDateFormatPart::Millis(get_all_chars(c, &mut chars))),
            'a' => parts.push(SimpleDateFormatPart::AmPm(get_all_chars(c, &mut chars))),
            'E' => parts.push(SimpleDateFormatPart::WeekDay(get_all_chars(c, &mut chars))),
            'D' => parts.push(SimpleDateFormatPart::YearDay(get_all_chars(c, &mut chars))),
            'z' => parts.push(SimpleDateFormatPart::Zone(get_all_chars(c, &mut chars))),
            _ => return Err(ParseError::Format(format!("Illegal char: {}", c))),
        }
    }

    Ok(SimpleDateFormat{ parts })
}

fn format_zone<Tz>(date_time: &DateTime<Tz>, _cnt: usize) -> String where Tz: TimeZone {
    format!("{:?}", date_time.offset())
}

fn format_ampm(is_pm: bool, _cnt: usize) -> &'static str {
    if is_pm { "PM" } else { "AM" }
}

fn format_week_day(n: Weekday, cnt: usize) -> &'static str {
    let is_short = cnt == 3;
    match n {
        Weekday::Mon => if is_short { "Mon" } else { "Monday"    },
        Weekday::Tue => if is_short { "Tue" } else { "Tuesday"   },
        Weekday::Wed => if is_short { "Wed" } else { "Wednesday" },
        Weekday::Thu => if is_short { "Thu" } else { "Thursday"  },
        Weekday::Fri => if is_short { "Fri" } else { "Friday"    },
        Weekday::Sat => if is_short { "Sat" } else { "Saturday"  },
        Weekday::Sun => if is_short { "Sun" } else { "Sunday"    },
   }
}

fn format_year(n: i32, cnt: usize) -> String {
    let y = if cnt == 2 { n % 100 } else { n };
    format_str(y, cnt)
}

fn format_month(n: u32, cnt: usize) -> &'static str {
    let is_short = cnt == 3;
    match n {
         1 => if is_short { "Jan" } else { "January"   },
         2 => if is_short { "Feb" } else { "February"  },
         3 => if is_short { "Mar" } else { "March"     },
         4 => if is_short { "Apr" } else { "April"     },
         5 => "May", //if is_short { "May" } else { "May"       },
         6 => if is_short { "Jun" } else { "June"      },
         7 => if is_short { "Jul" } else { "July"      },
         8 => if is_short { "Aug" } else { "August"    },
         9 => if is_short { "Sep" } else { "September" },
        10 => if is_short { "Oct" } else { "October"   },
        11 => if is_short { "Nov" } else { "November"  },
        12 => if is_short { "Dec" } else { "December"  },
        _ => "ERR!UNKNOWN MONTH",
    }
}

fn format_str(n: i32, cnt: usize) -> String {
    let ret = format!("{}", n);
    if cnt > ret.len() {
        "0".repeat(cnt - ret.len()) + &ret
    } else {
        ret
    }
}

fn get_all_chars(c: char, chars: &mut Peekable<Chars>) -> usize {
    let mut cnt = 1_usize;
    while let Some(next_char) = chars.peek() {
        if *next_char == c {
            cnt += 1;
            chars.next();
        } else {
            break;
        }
    }
    cnt
}
