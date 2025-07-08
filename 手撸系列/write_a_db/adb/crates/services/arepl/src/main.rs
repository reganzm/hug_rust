use aparser::{
    ast::{parse_sql_command, SqlCommand},
    error::FormattedError,
};
use miette::{Context, GraphicalReportHandler, IntoDiagnostic};
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
                // 解析输入的命令
                let result = parse_sql_command(&line);
                display(result);
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

fn display(res: Result<SqlCommand, FormattedError>) {
    match res {
        Ok(exec_res) => println!("{:#?}", exec_res),
        Err(e) => {
            let mut s = String::new();
            GraphicalReportHandler::new()
                .with_cause_chain()
                .with_context_lines(10)
                .render_report(&mut s, &e)
                .unwrap();
            println!("{s}");
        }
    }
}
