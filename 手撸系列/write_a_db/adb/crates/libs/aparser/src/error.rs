//! 统一错误错误
//!
use miette::Diagnostic;
use nom_supreme::error::{BaseErrorKind, ErrorTree, GenericErrorTree, StackContext};
use thiserror::Error;

use crate::parse::RawSpan;

// 定义错误类型
pub type MyParseError<'a> = ErrorTree<RawSpan<'a>>;

// 定义miette断言的结构体
#[derive(Error, Debug, Diagnostic)]
#[error("解析错误")]
pub struct FormattedError<'b> {
    // 错误所在源码
    #[source_code]
    src: &'b str,
    // 错误位置和错误类型
    #[label("{kind}")]
    span: miette::SourceSpan,

    kind: BaseErrorKind<&'b str, Box<dyn std::error::Error + Send + Sync + 'static>>,

    // 关联的其它信息
    #[related]
    others: Vec<FormattedErrorContext<'b>>,
}

#[derive(Debug, Error, Diagnostic)]
#[error("解析错误上下文信息")]
pub struct FormattedErrorContext<'b> {
    #[source_code]
    src: &'b str,
    #[label("{context}")]
    span: miette::SourceSpan,

    context: StackContext<&'b str>,
}

/// 将nom的解析错误转换为自定义错误
pub fn format_parse_error<'a>(input: &'a str, e: MyParseError<'a>) -> FormattedError<'a> {
    match e {
        GenericErrorTree::Base { location, kind } => {
            let offset = location.location_offset().into();
            FormattedError {
                src: &input,
                span: miette::SourceSpan::new(offset, 0),
                kind,
                others: Vec::new(),
            }
        }
        GenericErrorTree::Alt(alt_errors) => alt_errors
            .into_iter()
            .map(|e| format_parse_error(input, e))
            .max_by_key(|formated| formated.others.len())
            .unwrap(),
        GenericErrorTree::Stack { base, contexts } => {
            let mut base = format_parse_error(input, *base);
            let mut contexts = contexts
                .into_iter()
                .map(|(loc, context)| {
                    let offset = loc.location_offset().into();
                    FormattedErrorContext {
                        src: input,
                        span: miette::SourceSpan::new(offset, 0),
                        context,
                    }
                })
                .collect();
            base.others.append(&mut contexts);
            base
        }
    }
}
