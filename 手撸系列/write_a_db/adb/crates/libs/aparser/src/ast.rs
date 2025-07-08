//! 将渐变语句、查询语句、插入语句整合起来

use crate::{
    commands::{CreateStatement, InsertStatement, SelectStatement},
    error::FormattedError,
    parse::{peek_then_cut, Parse},
};
use nom::{
    branch::alt,
    character::complete::{char, multispace0},
    combinator::{eof, map},
    error::context,
    multi::many1,
    sequence::{preceded, tuple},
};
use serde::{Deserialize, Serialize};

/// 枚举，命令类型
#[derive(Debug, Clone, Hash, Serialize, Deserialize)]
pub enum SqlCommand {
    Select(SelectStatement),
    Insert(InsertStatement),
    Create(CreateStatement),
}

// 为SqlCommand实现Parse特征

impl<'a> Parse<'a> for SqlCommand {
    fn parse(i: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let (rest, (command, _, _, _)) = context(
            "SQL命令解析",
            preceded(
                multispace0,
                tuple((
                    alt((
                        peek_then_cut("select", map(SelectStatement::parse, SqlCommand::Select)),
                        peek_then_cut("insert", map(InsertStatement::parse, SqlCommand::Insert)),
                        peek_then_cut("create", map(CreateStatement::parse, SqlCommand::Create)),
                    )),
                    multispace0,
                    char(';'),
                    multispace0,
                )),
            ),
        )(i)?;
        Ok((rest, command))
    }
}

// 为SqlCommand实现tryFrom特征，尝试从字符串直接转换为Command
impl<'a> TryFrom<&'a str> for SqlCommand {
    type Error = FormattedError<'a>;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        match SqlCommand::parse_format_error(&value) {
            Ok(command) => Ok(command),
            Err(e) => Err(e),
        }
    }
}
pub fn parse_sql_command(input: &str) -> Result<SqlCommand, FormattedError<'_>> {
    input.try_into()
}

// 为一组命令实现Parse特征
impl<'a> Parse<'a> for Vec<SqlCommand> {
    fn parse(i: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let (rest, (comands, _)) = tuple((many1(SqlCommand::parse), eof))(i)?;
        Ok((rest, comands))
    }
}

pub fn parse_multiple_sqlcommand(input: &str) -> Result<Vec<SqlCommand>, FormattedError<'_>> {
    Vec::<SqlCommand>::parse_format_error(input)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_insert_command() {
        let insert = "insert into student values '三角兽',18;";
        let result = parse_sql_command(&insert);
        println!("result:{:#?}", result);
    }

    #[test]
    fn test_select_command() {
        let select = "select name,age from student;";
        let result = parse_sql_command(&select);
        println!("result:{:#?}", result);
    }

    #[test]
    fn test_create_command() {
        let create = "create table student (name string,age int);";
        let result = parse_sql_command(&create);
        println!("result:{:#?}", result);
    }

    #[test]
    fn test_multiple_command() {
        let create = r#"
        create table student (name string,age int);
        insert into student values '三角兽',18;
        select name,age from student;"#;
        let result = parse_multiple_sqlcommand(&create);
        println!("result:{:#?}", result);
    }
}
