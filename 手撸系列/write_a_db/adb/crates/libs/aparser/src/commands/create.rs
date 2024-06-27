//! create sql解析

use crate::parse::{comma_sep, identifier, Parse, ParseResult, RawSpan};
use derive_more::Display;
use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    character::{
        complete::{char, multispace1},
        streaming::multispace0,
    },
    combinator::map,
    error::context,
    sequence::{preceded, separated_pair, tuple},
};
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

/// 定义建表sql语句中的类型
/// 暂时只支持string 和 int

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display, Copy)]
pub enum FieldType {
    String,
    Int,
}

// 为FieldType实现Parse特征
impl<'a> Parse<'a> for FieldType {
    fn parse(i: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        context(
            "字段类型",
            alt((
                map(tag_no_case("string"), |_| Self::String),
                map(tag_no_case("int"), |_| Self::Int),
            )),
        )(i)
    }
}

/// 定义列结构体
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    pub field_type: FieldType,
}

// 为Column实现Parse特征
impl<'a> Parse<'a> for Column {
    fn parse(i: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        context(
            "列解析",
            map(
                separated_pair(identifier.context("列名"), multispace1, FieldType::parse),
                |(name, field_type)| Self { name, field_type },
            ),
        )(i)
    }
}

// 解析所有的列
fn column_definition(input: RawSpan<'_>) -> ParseResult<'_, Vec<Column>> {
    context(
        "解析所有列",
        map(
            tuple((
                char('('),
                multispace0,
                comma_sep(Column::parse),
                multispace0,
                char(')'),
            )),
            |(_, _, cols, _, _)| cols,
        ),
    )(input)
}

/// 解析建表语句
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CreateStatement {
    pub table: String,
    pub columns: Vec<Column>,
}

/// 为CreateStatement实现Parse特征
impl<'a> Parse<'a> for CreateStatement {
    fn parse(i: RawSpan<'a>) -> ParseResult<'a, Self> {
        map(
            separated_pair(
                preceded(
                    tuple((
                        tag_no_case("create"),
                        multispace1,
                        tag_no_case("table"),
                        multispace1,
                    )),
                    identifier.context("表名"),
                ),
                multispace1,
                column_definition,
            )
            .context("创建表"),
            |(table, columns)| Self { table, columns },
        )(i)
    }
}

#[cfg(test)]
mod test {
    use crate::parse::Parse;

    use super::CreateStatement;

    #[test]
    fn test_create_table() {
        let create_table = "create table student (name string,age int)";
        let result = CreateStatement::parse_from_raw(&create_table);
        println!("result:{:#?}", result);
    }
}
