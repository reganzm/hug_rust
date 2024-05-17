use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{consumed, peek, recognize,map},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
fn main() {
    let msg = "hello,rust";
    let msg1 = "hellohello,rust";
    println!("tag:{:?}", hello_parser(msg));
    println!("peek:{:?}", peek_parser(msg));
    println!("pair:{:?}", pair_parser(msg));
    println!("tuple:{:?}", tuple_parser(msg));
    println!("alt:{:?}", alt_parser(msg));
    println!("delimited:{:?}", delimited_parser(msg));
    println!("separated:{:?}", separated_pair_parser(msg));
    println!("preceded:{:?}", preceded_parser(msg));
    println!("terminated:{:?}", ternimated_parser(msg));
    println!("recognize:{:?}", recognize_parser(msg));
    println!("consumed:{:?}", comsumed_parser(msg));
    println!("many0:{:?}", many0_parser(msg1));
    println!("many1:{:?}", many1_parser(msg1));
    println!("map:{:?}",map_transformation(msg1));
}
// 解析器——消耗
fn hello_parser(input: &str) -> IResult<&str, &str> {
    tag("hello")(input)
}

// 解析器——产生一个解析器 但是不消耗匹配的源
fn peek_parser(input: &str) -> IResult<&str, &str> {
    peek(tag("hello"))(input)
}

// 组合算子——串联——pair——消耗 支持两个解析器
fn pair_parser(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    pair(tag("hello"), pair(tag(","), tag("rust")))(input)
}

// 组合算子——串联——tuple——消耗 最多支持21个解析器
fn tuple_parser(input: &str) -> IResult<&str, (&str, &str, &str)> {
    tuple((tag("hello"), tag(","), tag("rust")))(input)
}

// 组合算子——并联——alt——消耗 只要一个解析器成功即成功
fn alt_parser(input: &str) -> IResult<&str, &str> {
    alt((tag("hello"), tag(","), tag("rust")))(input)
}

// 组合算子——delimited——消耗 传入三个解析器 所有解析器成功后才算成功，成功后返回中间解析器的值
fn delimited_parser(input: &str) -> IResult<&str, &str> {
    delimited(tag("hello"), tag(","), tag("rust"))(input)
}

// 组合算子——separated_pair——消耗 传入三个解析器 所有解析器都成功后才算成功，返回第一个和第三个解析器中的值，忽略第二个解析器产生的值
fn separated_pair_parser(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(tag("hello"), tag(","), tag("rust"))(input)
}

// 组合算子——proceded——消耗 传入两个解析器，所有解析器都成功才算成功，返回第二个解析器中的值
fn preceded_parser(input: &str) -> IResult<&str, &str> {
    preceded(tag("hello"), tag(","))(input)
}

// 组合算子——terminated——消耗 传入两个解析器，所有解析器都成功才算成功，返回第一个解析器产生的值
fn ternimated_parser(input: &str) -> IResult<&str, &str> {
    terminated(tag("hello"), tag(","))(input)
}

// 组合算子——recognize——消耗 将子解析器解析成功的返回值拼接起来作为返回值
fn recognize_parser(input: &str) -> IResult<&str, &str> {
    recognize(separated_pair_parser)(input)
}

// 组合算子——consumed——非消耗 返回子解析器的值且不消耗传入的数据源
fn comsumed_parser(input: &str) -> IResult<&str, (&str, (&str, &str))> {
    consumed(separated_pair_parser)(input)
}

// 多匹配算子——many0——消耗 产生0到多个匹配的解析器 类似于正则表达式中的*
fn many0_parser(input: &str) -> IResult<&str, Vec<&str>> {
    many0(tag("hello"))(input)
}

// 多匹配算子——many1——消耗 产生1到多个匹配的解析器 类似与正则表达式中的+
fn many1_parser(input: &str) -> IResult<&str, Vec<&str>> {
    many1(tag("hello"))(input)
}

// map转换函数
fn map_transformation(input:&str)->IResult<&str,usize>{
    map(many0(tag("hello")),|s:Vec<_>|s.len())(input)
}