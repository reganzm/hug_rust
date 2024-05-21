//! 时序数据库协议解析
use nom::{
    branch::alt,
    bytes::complete::{tag, tag_no_case, take_while1},
    character::complete::{digit1, one_of},
    combinator::{map, map_res, opt, recognize, value},
    error::ErrorKind,
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
use std::time::Instant;
use std::{thread, time::Duration};
pub mod data;

#[derive(Debug, Clone, Copy, PartialEq)]
enum FieldValue<'a> {
    Bool(bool),      // 布尔数据类型
    Int(i64),        // 有符号整数
    Uint(u64),       // 无符号整数
    Float(f64),      // 浮点数
    String(&'a str), // 字符串
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
        tag("\\ "), // 匹配出现的\ 字符
        tag("\\,"), // 匹配出现的，逗号字符，逗号字符使用\转义了
        tag("\\"),
        take_while1(|c| c != ',' && c != ',' && c != '\\'), // 一直取出字符做匹配，直到不满足条件为止
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

fn parse_int(input: &str) -> IResult<&str, i64> {
    map_res(
        recognize(pair(opt(alt((tag("+"), tag("-")))), digit1)),
        |s: &str| s.parse::<i64>(),
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
            ))), // 识别小数开头的科学计数法
            recognize(tuple((opt(one_of("+-")), digit1, tag("."), digit1))), //识别小数
            recognize(tuple((
                opt(one_of("+-")),
                digit1,
                tag("e"),
                opt(one_of("+-")),
                digit1,
            ))), // 识别整数开头的科学计数法
            recognize(pair(opt(alt((tag("+"), tag("-")))), digit1)), // 单独的整数识别为浮点数
        )),
        |s: &str| s.parse::<f64>(),
    )(input)
}

fn float_value(input: &str) -> IResult<&str, FieldValue> {
    map(parse_double, |f| FieldValue::Float(f))(input)
}

fn parse_str(input: &str) -> IResult<&str, &str> {
    delimited(
        tag("\""),
        recognize(many1(alt((take_while1(|c| c != '"'),)))),
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
            measurement,                     // 解析表名
            terminated(tag_set, tag(" ")),   // 解析静态标签，忽略最后的空格
            terminated(field_set, tag(" ")), // 解析动态标签，忽略最后的空格
            time_stamp,                      // 解析时间戳
        )),
        |(m, t, f, ts)| Line {
            measurement: m,
            tag_set: t,
            field_set: f,
            timestamp: ts,
        }, // 使用map函数，使用tuple中的每个解析器的值分别构造Line的一个字段
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

fn main() {
    // 构造10万行时序数据并解析
    // 记录开始时间
    let start = Instant::now();
    let results = lines(data::DATA).unwrap().1;
    println!(
        "耗时:{:?} ,  解析结果数:{:?}",
        start.elapsed(),
        results.len()
    );
    println!("解析样例数据:{:#?}", results.last());

    thread::sleep(Duration::from_secs(20));
}







#[test]
fn test_mesurement() {
    assert_eq!(measurement("天气"), Ok(("", "天气")));
    assert_eq!(measurement("天气\\\\ \\,啊"), Ok(("", "天气\\\\ \\,啊")));
    assert_eq!(measurement("天气,预报"), Ok((",预报", "天气")));
    assert_eq!(measurement("天气\\\\,预报"), Ok(("", "天气\\\\,预报")));
    assert_eq!(measurement("天气\\\\ 预报"), Ok(("", "天气\\\\ 预报")));
}

#[test]
fn test_tag_k_v() {
    assert_eq!(tag_k_v("时间"), Ok(("", "时间")));
    assert_eq!(tag_k_v("时间 "), Ok((" ", "时间")));
    assert_eq!(tag_k_v(r#"时间\ \,\=\"#), Ok(("", r#"时间\ \,\=\"#))); // r#支持原生写法
    println!("{:?}", r#"时间\ \,\=\"#);
    assert_eq!(tag_k_v(r#"时间\ \,\=,"#), Ok((",", r#"时间\ \,\="#)));
}

#[test]
fn test_tag_set() {
    assert_eq!(
        tag_set(",name=小红,age=12,height=160,"),
        Ok((
            ",",
            vec![("name", "小红"), ("age", "12"), ("height", "160")]
        ))
    );
    assert_eq!(
        tag_set(",name=小红 ,age=12,height=160,"),
        Ok((" ,age=12,height=160,", vec![("name", "小红")]))
    );
    assert_eq!(
        tag_set(",name=小红\\ ,age=12,height=160,"),
        Ok((
            ",",
            vec![("name", "小红\\ "), ("age", "12"), ("height", "160")]
        ))
    );
}

#[test]
fn test_bool_value() {
    assert_eq!(bool_value("T"), Ok(("", FieldValue::Bool(true))));
    assert_eq!(bool_value("TRue"), Ok(("", FieldValue::Bool(true))));
    assert_eq!(bool_value("FalSE"), Ok(("", FieldValue::Bool(false))));
}

#[test]
fn test_parse_int() {
    assert_eq!(parse_int("+12345"), Ok(("", 12345)));
    assert_eq!(parse_int("-12345"), Ok(("", -12345)));
    assert_eq!(parse_int("12345"), Ok(("", 12345)));
}

#[test]
fn test_int_value() {
    assert_eq!(int_value("+12345i"), Ok(("", FieldValue::Int(12345))));
    assert_eq!(int_value("-12345i"), Ok(("", FieldValue::Int(-12345))));
    assert_eq!(int_value("12345i"), Ok(("", FieldValue::Int(12345))));
}

#[test]
fn test_uint_value() {
    assert_eq!(uint_value("957867U"), Ok(("", FieldValue::Uint(957867))));
}

#[test]
fn test_parse_double() {
    assert_eq!(parse_double("5"), Ok(("", 5 as f64)));
    assert_eq!(parse_double("7.4"), Ok(("", 7.4)));
    assert_eq!(parse_double("2.31e5"), Ok(("", 2.31e5)));
    assert_eq!(parse_double("-3.14e5"), Ok(("", -3.14e5)));
    assert_eq!(parse_double("-3.14e-5"), Ok(("", -3.14e-5)));
    assert_eq!(parse_double("-1e-6"), Ok(("", -1e-6)));
}


#[test]
fn test_float_value() {
    assert_eq!(float_value("-1e-6"), Ok(("", FieldValue::Float(-1e-6))));
    assert_eq!(float_value("-3.14e5"), Ok(("", FieldValue::Float(-3.14e5))));
}

#[test]
fn test_str_value() {
    assert_eq!(
        str_value("\"hello rust\""),
        Ok(("", FieldValue::String("hello rust")))
    );
    assert_eq!(str_value("\"\""), Ok(("", FieldValue::String(""))));
}

#[test]
fn test_field_set() {
    assert_eq!(
        field_set("name=\"小红\",age=12u,height=165.4,weight=50.1,score=89i"),
        Ok((
            "",
            vec![
                ("name", FieldValue::String("小红")),
                ("age", FieldValue::Uint(12)),
                ("height", FieldValue::Float(165.4 as f64)),
                ("weight", FieldValue::Float(50.1 as f64)),
                ("score", FieldValue::Int(89))
            ]
        ))
    )
}

#[test]
fn test_time_stamp() {
    assert_eq!(
        time_stamp("1640995200000000000"),
        Ok(("", 1640995200000000000))
    );
}

#[test]
fn test_line() {
    println!(
        "{:?}",
        line("onview,xxx=1,yyyy=2.3 aaa=3,bbb=4 1640995200000000000")
    );
}