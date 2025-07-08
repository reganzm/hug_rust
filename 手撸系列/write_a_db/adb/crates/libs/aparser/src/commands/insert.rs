//! 插入语句解析

use crate::{
    parse::{comma_sep, identifier, Parse},
    parse_value::Value,
};
use nom::{
    bytes::complete::tag_no_case,
    character::complete::multispace1,
    error::context,
    sequence::{preceded, tuple},
};
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

/// 插入语句结构体
#[derive(Debug, Clone, PartialEq, Default, Hash, Serialize, Deserialize)]
pub struct InsertStatement {
    pub table: String,
    pub values: Vec<Value>,
}

// 为InsertStatement实现Parse特征
impl<'a> Parse<'a> for InsertStatement {
    fn parse(i: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let (remaining_input, (_, _, table, _, values)) = context(
            "插入语句解析",
            tuple((
                tag_no_case("insert"),
                preceded(multispace1, tag_no_case("into")),
                preceded(multispace1, identifier.context("表名")),
                preceded(multispace1, tag_no_case("values")),
                preceded(multispace1, comma_sep(Value::parse).context("值")),
            )),
        )(i)?;
        Ok((remaining_input, InsertStatement { table, values }))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_statement_parse() {
        let insert = "insert into student values '三角兽',18";
        let result = InsertStatement::parse_from_raw(&insert);
        println!("result:{:#?}", result);
    }
}
