#[macro_use] extern crate quick_error;
use chrono::{ Local,prelude::*, };

use std::str::Chars;
use std::iter::Peekable;

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
    YearLower(usize),
    YearUpper(usize),
    MonthLower(usize),
    MonthUpper(usize),
    Day,
    Hour,
    Minute,
    Second,
    LiteralChar(char),
    Literal(String),
}

#[derive(Debug)]
pub struct SimpleDateFormat {
    parts: Vec<SimpleDateFormatPart>,
}

impl SimpleDateFormat {

    pub fn format_local(&self, date_time: &DateTime<Local>) -> String {
        let mut ret = String::with_capacity(512);

        for part in &self.parts {
            match part {
                SimpleDateFormatPart::LiteralChar(c) => ret.push(*c),
                SimpleDateFormatPart::Literal(s) => ret.push_str(s),
                SimpleDateFormatPart::YearLower(cnt) => ret.push_str(&format_str(date_time.year(), *cnt)),
                SimpleDateFormatPart::MonthLower(cnt) => ret.push_str(&format_str(date_time.month() as i32, *cnt)),
                _ => (),
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
                    parts.push(SimpleDateFormatPart::Literal(literal));
                    literal = String::new();
                }
            } else {
                is_in_quotation_mark = true;
            },
            ',' | '.' | ':' | '-' | ' ' => parts.push(SimpleDateFormatPart::LiteralChar(c)),
            'y' => parts.push(SimpleDateFormatPart::YearLower(get_all_chars(c, &mut chars))),
            'Y' => parts.push(SimpleDateFormatPart::YearUpper(get_all_chars(c, &mut chars))),
            'm' => parts.push(SimpleDateFormatPart::MonthLower(get_all_chars(c, &mut chars))),
            'M' => parts.push(SimpleDateFormatPart::MonthUpper(get_all_chars(c, &mut chars))),
            _ => (),
        }
    }

    Ok(SimpleDateFormat{ parts })
    // Err(ParseError::Format(f.into()))
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

#[test]
fn it_works() {
    // println!("test output: {}", fmt("").unwrap().format_local(&Local::now()));

    println!("{:?}", fmt("y yy-mm 'mm '''"));
    println!("{:?}", fmt("y yy-mm'(-'m')' '[mm]'").unwrap().format_local(&Local::now()));

    assert_eq!(2 + 2, 4);
}
