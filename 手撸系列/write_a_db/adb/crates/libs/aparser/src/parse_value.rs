//! 解析值

use std::str::FromStr;

use crate::parse::{peek_then_cut, Parse, ParseResult, RawSpan};
use bigdecimal::BigDecimal;
use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::complete::multispace0,
    error::context,
    sequence::{preceded, terminated, tuple},
    Parser,
};
use serde::{Deserialize, Serialize};

/// 支持的值类型枚举
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum Value {
    Number(BigDecimal), // 小数
    String(String),     // 字符串
}

/// 为Value实现Parse特征
impl<'a> Parse<'a> for Value {
    fn parse(input: RawSpan<'_>) -> ParseResult<'_, Value> {
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
fn parse_string_value(input: RawSpan<'_>) -> ParseResult<'_, Value> {
    // 使用nom::error::context上下文包裹
    let (remaining, (_, str_value, _)) = context(
        "字符串字面量解析",
        tuple((
            tag("'"),
            // take_until不消耗最后一个字符，属于前闭后开
            take_until("'").map(|r: RawSpan| Value::String(r.fragment().to_string())),
            tag("'"),
        )),
    )(input)?;
    Ok((remaining, str_value))
}

/// 解析数值
fn parse_number_value(input: RawSpan<'_>) -> ParseResult<'_, Value> {
    let (remaining, digits) =
        context("数值字面量解析", take_while(|c: char| c.is_numeric()))(input)?;
    let digits = digits.fragment();
    Ok((
        remaining,
        Value::Number(BigDecimal::from_str(digits).unwrap()),
    ))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_string_value() {
        let s = "'兽兽',18".to_string();
        let result = Value::parse_from_raw(s.as_str());
        println!("result:{:?}", result);
    }

    #[test]
    fn test_parse_number_value() {
        let result = Value::parse_from_raw("1234567");
        println!("result:{:?}", result);
    }
}
