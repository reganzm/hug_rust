//! 查询命令解析

use crate::parse::{comma_sep, identifier, Parse};
use nom::{
    bytes::complete::tag_no_case, character::complete::multispace1, error::context, sequence::tuple,
};
use nom_supreme::ParserExt;
use serde::{Deserialize, Serialize};

/// 查询命令结构体
#[derive(Debug, Clone, PartialEq, Default, Hash, Serialize, Deserialize)]
pub struct SelectStatement {
    pub table: String,
    pub fields: Vec<String>,
}

// 实现Display特征
impl std::fmt::Display for SelectStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SELECT ")?;
        write!(f, "{}", self.fields.join(", "))?;
        write!(f, " FROM ")?;
        write!(f, "{}", self.table)?;
        Ok(())
    }
}

// 实现Parse特征
impl<'a> Parse<'a> for SelectStatement {
    fn parse(i: crate::parse::RawSpan<'a>) -> crate::parse::ParseResult<'a, Self> {
        let (remaining_input, (_, _, fields, _, _, _, table)) = context(
            "查询语句解析",
            tuple((
                tag_no_case("select"),
                multispace1,
                comma_sep(identifier).context("查询列名"),
                multispace1,
                tag_no_case("from"),
                multispace1,
                identifier.context("表名"),
            )),
        )(i)?;

        Ok((remaining_input, SelectStatement { table, fields }))
    }
}

#[cfg(test)]
mod test {
    use super::SelectStatement;
    use crate::parse::Parse;

    #[test]
    fn test_select_statement() {
        let select = "select name,age from student";
        let result = SelectStatement::parse_from_raw(&select);
        println!("result:{:#?}", result);
    }
}
