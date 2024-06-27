//! 定义通用的解析方法

use nom::{
    self,
    bytes::complete::{tag_no_case, take_while1},
    character::complete::{char, multispace0},
    combinator::{all_consuming, map, peek},
    multi::separated_list1,
    sequence::{pair, tuple},
    Finish, IResult,
};
use nom_locate::LocatedSpan;

use crate::error::{format_parse_error, FormattedError, MyParseError};

// 定义类型，用于包装输入str
pub type RawSpan<'a> = LocatedSpan<&'a str>;
// 定义解析结果类型，错误类型用自定义的MyParseError
pub type ParseResult<'a, T> = IResult<RawSpan<'a>, T, MyParseError<'a>>;

// 解析sql标识符的通用函数
pub fn identifier(i: RawSpan) -> ParseResult<String> {
    map(take_while1(|c: char| c.is_alphanumeric()), |s: RawSpan| {
        s.fragment().to_string()
    })(i)
}

/// 偷窥tag是否存在，如果存在则调用传入的解析器
pub fn peek_then_cut<'a, T, O, E, F>(
    peek_tag: T,
    f: F,
) -> impl FnMut(RawSpan<'a>) -> IResult<RawSpan<'a>, O, E>
where
    T: nom::InputLength + Clone,
    F: nom::Parser<RawSpan<'a>, O, E>,
    E: nom::error::ParseError<RawSpan<'a>> + nom_supreme::tag::TagError<RawSpan<'a>, T>,
    LocatedSpan<&'a str>: nom::Compare<T>,
{
    map(pair(peek(tag_no_case(peek_tag)), f), |(_, f_res)| f_res)
}

/// 按照逗号分隔
pub fn comma_sep<'a, O, E, F>(f: F) -> impl FnMut(RawSpan<'a>) -> IResult<RawSpan<'a>, Vec<O>, E>
where
    F: nom::Parser<RawSpan<'a>, O, E>,
    E: nom::error::ParseError<RawSpan<'a>>,
{
    separated_list1(tuple((multispace0, char(','), multispace0)), f)
}

/// 定义通用的解析trait，用于将sql转换为不同的指令
/// 使用Sized对trait进行限定，实现该trait的结构体都是大小可知的
pub trait Parse<'a>: Sized {
    fn parse(i: RawSpan<'a>) -> ParseResult<'a, Self>;

    // 输入原始字符串，包装成RawSpan类型，并调用自身的parse关联函数解析
    fn parse_from_raw(input: &'a str) -> ParseResult<'a, Self> {
        let t = LocatedSpan::new(input);
        Self::parse(t)
    }

    // 统一错误处理，将错误信息处理为格式化的错误
    fn parse_format_error(i: &'a str) -> Result<Self, FormattedError<'a>> {
        let input = LocatedSpan::new(i);
        match all_consuming(Self::parse)(input).finish() {
            Ok((_, query)) => Ok(query),
            Err(e) => Err(format_parse_error(i, e)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_identifier() {
        // 使用LocatedSpan将输入字符串转换为LocatedSpan(RawSpan)
        let result = identifier(LocatedSpan::new("select * from student"));
        println!("result:{:#?}", result);
    }

    #[test]
    fn test_peek_then_cut() {}
}
