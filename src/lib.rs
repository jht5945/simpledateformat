#[macro_use] extern crate quick_error;
use chrono::{ Local,prelude::*, };

quick_error! {
    #[derive(Debug)]
    pub enum ParseError {
        Format(m: String) {
            display("Format error: {}", m)
        }
    }
}

enum SimpleDateFormatPart {
    Year,
    Month,
    Day,
    Hour,
    Minute,
    Second,
    Literal(String),
}

pub struct SimpleDateFormat {
    parts: Vec<SimpleDateFormatPart>,
}

impl SimpleDateFormat {

    fn format_local(&self, date_time: &DateTime<Local>) -> String {
        let mut ret = String::with_capacity(512);
        ret.push_str(&format!("{}", date_time.year()));
        ret.push('-');
        ret.push_str(&format!("{}", date_time.month()));
        ret.push('-');
        ret.push_str(&format!("{}", date_time.day()));
        ret
    }
}

pub fn fmt(f: &str) -> Result<SimpleDateFormat, ParseError> {
    Ok(SimpleDateFormat{ parts: vec![] })
    // Err(ParseError::Format(f.into()))
}


#[test]
fn it_works() {
    println!("test output: {}", fmt("").unwrap().format_local(&Local::now()));

    assert_eq!(2 + 2, 4);
}
