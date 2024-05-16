use nom::character::complete::digit1;
use nom::combinator::map;
use nom::{error::ErrorKind, Err, Parser};

fn main() {
    let mut parser = map(digit1, |s: &str| s.len());
    // the parser will count how many characters were returned by digit1
    assert_eq!(parser.parse("123456abc"), Ok(("abc", 6)));
    assert_eq!(
        parser.parse("abc"),
        Err(Err::Error(("abc", ErrorKind::Digit)))
    );
}
