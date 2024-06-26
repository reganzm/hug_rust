//! 解析值

use std::str::FromStr;

use crate::{
    error::MyParseError,
    parse::{peek_then_cut, Parse},
};
use bigdecimal::BigDecimal;
use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::multispace0,
    error::context,
    sequence::{preceded, terminated, tuple},
    IResult, Parser,
};
use nom_locate::LocatedSpan;
use serde::{Deserialize, Serialize};

// 定义类型，用于包装输入str
pub type RawSpan<'a> = LocatedSpan<&'a str>;
// 定义解析结果类型，错误类型用自定义的MyParseError
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T, MyParseError<'a>>;
/// 支持的值类型枚举

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum ParseValue {
    Number(BigDecimal), // 小数
    String(String),     // 字符串
}

/// 为ParsedValue实现Parse特征
impl<'a> Parse<'a> for ParseValue {
    fn parse(input: RawSpan<'_>) -> ParseResult<'_, ParseValue> {
        context(
            "值解析",
            preceded(
                multispace0,
                terminated(
                    alt((peek_then_cut("'", parse_string_value), parse_number_value)),
                    multispace0,
                ),
            ),
        )(input)
    }
}

/// 解析单引号字符串值
fn parse_string_value(input: RawSpan<'_>) -> ParseResult<'_, ParseValue> {
    // 使用nom::error::context上下文包裹
    let (remaining, (_, str_value, _)) = context(
        "字符串字面量解析",
        tuple((
            tag("'"),
            // take_until不消耗最后一个字符，属于前闭后开
            take_until("'").map(|r: RawSpan| ParseValue::String(r.fragment().to_string())),
            tag("'"),
        )),
    )(input)?;
    Ok((remaining, str_value))
}

/// 解析数值
fn parse_number_value(input: RawSpan<'_>) -> ParseResult<'_, ParseValue> {
    let (remaining, digits) =
        context("数值字面量解析", take_while(|c: char| c.is_numeric()))(input)?;
    let digits = digits.fragment();
    Ok((
        remaining,
        ParseValue::Number(BigDecimal::from_str(digits).unwrap()),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_string_value() {
        let s = "'兽兽',18".to_string();
       let result = ParseValue::parse_from_raw(s.as_str());
       println!("result:{:?}",result);
    }


    #[test]
    fn test_parse_number_value() {
       let result = ParseValue::parse_from_raw("1234567");
       println!("result:{:?}",result);
    }
}
