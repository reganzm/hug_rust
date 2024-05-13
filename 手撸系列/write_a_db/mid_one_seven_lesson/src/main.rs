use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_while1;
use nom::character::complete::digit1;
use nom::character::streaming::one_of;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::error::ErrorKind;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

pub mod data;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldValue<'a> {
    Bool(bool),
    Int(i64),
    Uint(u64),
    Float(f64),
    String(&'a str),
}

#[derive(Debug, PartialEq)]
pub struct Line<'a> {
    measurement: &'a str,
    tag_set: Vec<(&'a str, &'a str)>,
    field_set: Vec<(&'a str, FieldValue<'a>)>,
    timestamp: i64,
}

fn measurement(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((
        tag("\\ "), // "\ " branch1
        tag("\\,"), // "\," branch2
        tag("\\"),  // "\" . branch3
        // recognize(none_of(" ,\\")),
        take_while1(|c| c != ' ' && c != ',' && c != '\\'),
    ))))(input)
}

fn tag_k_v(input: &str) -> IResult<&str, &str> {
    recognize(many1(alt((
        tag("\\ "), // "\ "
        tag("\\,"), // "\,"
        tag("\\="), // "\="
        tag("\\"),  // "\"
        take_while1(|c| c != ' ' && c != ',' && c != '\\' && c != '='),
    ))))(input)
}

// ,tag_k=tag_v,tag_k=tag_v.......
fn tag_set(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    many1(preceded(
        tag(","),
        separated_pair(tag_k_v, tag("="), tag_k_v),
    ))(input)
}

fn bool_value(input: &str) -> IResult<&str, FieldValue> {
    alt((
        value(FieldValue::Bool(true), tag_no_case("true")),
        value(FieldValue::Bool(false), tag_no_case("false")),
        value(FieldValue::Bool(true), tag_no_case("t")),
        value(FieldValue::Bool(false), tag_no_case("f")),
    ))(input)
}

fn integer<'a>(input: &'a str) -> IResult<&'a str, &'a str> {
    recognize(pair(opt(alt((tag("+"), tag("-")))), digit1))(input)
}

fn parse_int(input: &str) -> IResult<&str, i64> {
    map_res(
        recognize(pair(opt(alt((tag("+"), tag("-")))), digit1)),
        |s: &str| s.parse::<i64>(),
    )(input)
}

fn parse_double(input: &str) -> IResult<&str, f64> {
    map_res(
        alt((
            recognize(tuple((
                opt(one_of("+-")),
                digit1,
                tag("."),
                digit1,
                tag("e"),
                opt(one_of("+-")),
                digit1,
            ))),
            recognize(tuple((opt(one_of("+-")), digit1, tag("."), digit1))),
            recognize(tuple((
                opt(one_of("+-")),
                digit1,
                tag("e"),
                opt(one_of("+-")),
                digit1,
            ))),
            recognize(pair(opt(alt((tag("+"), tag("-")))), digit1)),
        )),
        |s: &str| s.parse::<f64>(),
    )(input)
}

fn int_value(input: &str) -> IResult<&str, FieldValue> {
    terminated(map(parse_int, |i| FieldValue::Int(i)), tag_no_case("i"))(input)
}

fn uint_value(input: &str) -> IResult<&str, FieldValue> {
    map(
        terminated(
            map_res(digit1, |s: &str| s.parse::<u64>()),
            tag_no_case("u"),
        ),
        |u| FieldValue::Uint(u),
    )(input)
}
fn float_value(input: &str) -> IResult<&str, FieldValue> {
    map(parse_double, |f| FieldValue::Float(f))(input)
}

fn parse_str(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("\""),
        recognize(many1(alt((
            tag("\\\""), // "\" . branch3
            take_while1(|c| c != '"'),
        )))),
        tag("\""),
    )(input)
}

fn str_value(input: &str) -> IResult<&str, FieldValue> {
    alt((
        value(
            FieldValue::String(""),
            recognize(pair(tag("\""), tag("\""))),
        ),
        map(parse_str, |s: &str| FieldValue::String(s)),
    ))(input)
}

fn field_value(input: &str) -> IResult<&str, FieldValue> {
    alt((bool_value, int_value, uint_value, float_value, str_value))(input)
}

fn field_set(input: &str) -> IResult<&str, Vec<(&str, FieldValue)>> {
    separated_list1(tag(","), separated_pair(tag_k_v, tag("="), field_value))(input)
}

fn time_stamp(input: &str) -> IResult<&str, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>())(input)
}

fn line(input: &str) -> IResult<&str, Line> {
    map(
        tuple((
            measurement,
            terminated(tag_set, tag(" ")),
            terminated(field_set, tag(" ")),
            time_stamp,
        )),
        |(m, t, f, ts)| Line {
            measurement: m,
            tag_set: t,
            field_set: f,
            timestamp: ts,
        },
    )(input)
}

fn lines(input: &str) -> IResult<&str, Vec<Line>> {
    let input = input.trim();
    let mut res = Vec::new();
    for l in input.lines() {
        let l = l.trim();
        match line(l) {
            Ok((input, line)) => {
                if !input.is_empty() {
                    return Err(nom::Err::Error(nom::error::Error::new(
                        input,
                        ErrorKind::Complete,
                    )));
                } else {
                    res.push(line)
                }
            }
            Err(e) => {
                return Err(e);
            }
        };
    }
    Ok(("", res))
}

#[test]
fn test_parse_double() {
    assert_eq!(parse_double("3"), Ok(("", 3 as f64)));
    assert_eq!(parse_double("3.14"), Ok(("", 3.14)));
    assert_eq!(parse_double("3.14e5"), Ok(("", 3.14e5)));
    assert_eq!(parse_double("-3.14e5"), Ok(("", -3.14e5)));
    assert_eq!(parse_double("-3.14e-5"), Ok(("", -3.14e-5)));
    assert_eq!(parse_double("-13e-6"), Ok(("", -13e-6)));
}

#[test]
fn test_parse_int() {
    assert_eq!(parse_int("+123"), Ok(("", 123)));
}

#[test]
fn test_lines() {
    // println!("{:?}", lines(data::DATA));
    assert!(lines(data::DATA).is_ok())
}

#[test]
fn test_line() {
    assert_eq!(
        line("m,t0=a,t1=b f0=1,f1=2i,f2=\"3\" 12324234"),
        Ok((
            "",
            Line {
                measurement: "m",
                tag_set: vec![("t0", "a"), ("t1", "b")],
                field_set: vec![
                    ("f0", FieldValue::Float(1 as f64)),
                    ("f1", FieldValue::Int(2)),
                    ("f2", FieldValue::String("3")),
                ],
                timestamp: 12324234
            }
        ))
    );
}

#[test]
fn test_field_set() {
    assert_eq!(
        field_set("a=1i,b=false,c=\"c\",d=4,e=45u"),
        Ok((
            "",
            vec![
                ("a", FieldValue::Int(1)),
                ("b", FieldValue::Bool(false)),
                ("c", FieldValue::String("c")),
                ("d", FieldValue::Float(4 as f64)),
                ("e", FieldValue::Uint(45))
            ]
        ))
    )
}

#[test]
fn test_tag_set() {
    assert_eq!(
        tag_set(",a=a,b=b,c=c,"),
        Ok((",", vec![("a", "a"), ("b", "b"), ("c", "c")]))
    );
    assert!(tag_set(",a==a,b=b,c=c,").is_err());
    assert_eq!(
        tag_set(",a=a ,b=b,c=c,"),
        Ok((" ,b=b,c=c,", vec![("a", "a")]))
    );
    assert_eq!(
        tag_set(",a=a\\ ,b=b,c=c,"),
        Ok((",", vec![("a", "a\\ "), ("b", "b"), ("c", "c")]))
    );
}

#[test]
fn test_tag_k_v() {
    assert_eq!(tag_k_v("地点"), Ok(("", "地点")));
    assert_eq!(tag_k_v("地点 "), Ok((" ", "地点")));
    assert_eq!(tag_k_v(r#"地点\ \,\="#), Ok(("", r#"地点\ \,\="#)));
    assert_eq!(tag_k_v(r#"地点\ \,\=,"#), Ok((",", r#"地点\ \,\="#)));
}

#[test]
fn test_bool_value() {
    assert_eq!(bool_value("T"), Ok(("", FieldValue::Bool(true))));
    assert_eq!(bool_value("TRue"), Ok(("", FieldValue::Bool(true))));
}
#[test]
fn test_int_value() {
    assert_eq!(int_value("+12345i"), Ok(("", FieldValue::Int(12345))));
    assert_eq!(int_value("-12345i"), Ok(("", FieldValue::Int(-12345))));
    assert_eq!(int_value("12345i"), Ok(("", FieldValue::Int(12345))));
    assert!(int_value("123456u").is_err());
}

#[test]
fn test_uint_value() {
    assert_eq!(uint_value("4357867U"), Ok(("", FieldValue::Uint(4357867))));
}

#[test]
fn test_str_value() {
    assert_eq!(str_value("\"abc\""), Ok(("", FieldValue::String("abc"))));
    assert_eq!(str_value("\"\""), Ok(("", FieldValue::String(""))));
}

#[test]
fn test_measurement() {
    assert_eq!(measurement("气温,hello"), Ok((",hello", "气温")));
    assert_eq!(
        measurement("气温\\,\\ hello "),
        Ok((" ", "气温\\,\\ hello"))
    );
}

#[test]
fn test_many0_hello() {}

fn main() {}
