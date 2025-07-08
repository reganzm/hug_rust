use miette::{Diagnostic, SourceSpan};
use miette::{NamedSource, Result as MietteResult};
use std::error::Error;
use thiserror::Error;
#[derive(Error, Debug, Diagnostic)]
#[error("oops!")]
#[diagnostic(code(src::bin::taste_miette), url(docsrs), help("请认真检查你的代码"))]
struct MyBad {
    // The Source that we're gonna be printing snippets out of.
    // This can be a String if you don't have or care about file names.
    #[source_code]
    src: NamedSource<String>,
    // Snippets and highlights can be included in the diagnostic!
    #[label("这里错误")]
    bad_bit: SourceSpan,
}
fn this_gives_correct_formatting() -> MietteResult<()> {
    let res: Result<(), MyBad> = Err(MyBad {
        src: NamedSource::new("taste_miette.rs", "source\n  text\n    here".to_string()),
        bad_bit: (9, 4).into(),
    });
    res?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let res = this_gives_correct_formatting();
    res?;
    Ok(())
}
