use simpledateformat::{ SimpleDateFormat, fmt, format_human, };
use chrono::prelude::*;
use std::time::Duration;

#[test]
fn test_simpledateformat_format() {
    let t = Utc.timestamp_millis(0);
    assert_eq!("1970/01/01 00:00:00.000 Z", &fmt("yyyy/MM/dd HH:mm:ss.SSS z").unwrap().format(&t));

    let t = Utc.timestamp_millis(1111111111);
    assert_eq!("1970/01/13 20:38:31.111 Z", &fmt("yyyy/MM/dd HH:mm:ss.SSS z").unwrap().format(&t));

    let t = Utc.timestamp_millis(1111111111);
    assert_eq!("1970/01/13 08:38:31.111 Z PM", &fmt("yyyy/MM/dd hh:mm:ss.SSS z a").unwrap().format(&t));

    let t = Utc.timestamp_millis(1590816448678);
    assert_eq!("2020/05/30 05:27:28.678 Z AM", &fmt("yyyy/MM/dd hh:mm:ss.SSS z a").unwrap().format(&t));

    let t = Utc.timestamp_millis(1590816448678);
    assert_eq!("Sat May 30, 2020 05:27:28.678 Z AM", &fmt("EEE MMM dd, yyyy hh:mm:ss.SSS z a").unwrap().format(&t));

    let t = Local.timestamp_millis(1590816448678);
    assert_eq!("Sat May 30, 2020 01:27:28.678 +08:00 PM", &fmt("EEE MMM dd, yyyy hh:mm:ss.SSS z a").unwrap().format(&t));
}

#[test]
fn test_format_human() {
    assert_eq!("0ms", format_human(Duration::from_millis(0)));
    assert_eq!("11ms", format_human(Duration::from_millis(11)));
    assert_eq!("11s 111ms", format_human(Duration::from_millis(11111)));
    assert_eq!("1s", format_human(Duration::from_secs(1)));
    assert_eq!("1min", format_human(Duration::from_secs(60)));
    assert_eq!("1hour", format_human(Duration::from_secs(60 * 60)));
    assert_eq!("1day", format_human(Duration::from_secs(24 * 60 * 60)));
    assert_eq!("2days", format_human(Duration::from_secs(2 * 24 * 60 * 60)));
    assert_eq!("2days 0hour 0min 1s", format_human(Duration::from_secs(2 * 24 * 60 * 60 + 1)));
}

#[test]
fn test_try_from() {
    use std::convert::TryInto;
    let t = Utc.timestamp_millis(0);
    let fmt = "yyyy/MM/dd HH:mm:ss.SSS z";
    let sdf: Result<SimpleDateFormat, _> = fmt.try_into();
    assert_eq!("1970/01/01 00:00:00.000 Z", sdf.unwrap().format(&t));
}
