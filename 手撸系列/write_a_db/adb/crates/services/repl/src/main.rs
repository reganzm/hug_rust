use miette::{Context, IntoDiagnostic};
use rustyline::{
    completion::FilenameCompleter, error::ReadlineError, highlight::Highlighter, Completer,
    CompletionType, Config, Editor, Helper, Hinter, Validator,
};

const HISTORY_FILE: &str = "./history.txt";
#[derive(Completer, Helper, Hinter, Validator)]
struct MyHelper(#[rustyline(Completer)] FilenameCompleter);
impl Highlighter for MyHelper {}

fn main() -> miette::Result<()> {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .build();

    let mut rl = Editor::with_config(config)
        .into_diagnostic()
        .wrap_err("初始化REPL")?;

    rl.set_helper(Some(MyHelper(FilenameCompleter::new())));

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).into_diagnostic()?;
                // todo 增加输入行处理函数
                //handle_line(line, &mut exec)
            }
            Err(ReadlineError::Interrupted) => {
                // CTRL-C 跳过本次输入
            }
            Err(ReadlineError::Eof) => {
                //"CTRL-D" 终止输入
                break;
            }
            Err(err) => {
                println!("遇到错误: {err:?}");
                break;
            }
        }
    }
    rl.save_history(HISTORY_FILE)
        .into_diagnostic()
        .wrap_err("Saving REPL history")?;

    Ok(())
}
