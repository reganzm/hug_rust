use nom::{
    branch::alt,
    bytes::complete::tag,
    combinator::{consumed, map, peek, recognize},
    multi::{many0, many1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};
fn main() {
    // 基于标签的解析
    let msg = "hello,world";
    println!("{:?}", hello_parser(msg));
    println!("{:?}", commer_parse(hello_parser(msg).unwrap().0));
    println!(
        "{:?}",
        world_parse(commer_parse(hello_parser(msg).unwrap().0).unwrap().0)
    );

    // 串联|组合算子 pair
    fn pair_parse(input: &str) -> IResult<&str, (&str, (&str, &str))> {
        pair(tag("hello"), pair(tag(","), tag("world")))(input)
    }
    println!("result pair parse:{:?}", pair_parse(msg));

    // 串联|组合算子tuple，最多支持21个解析器
    fn tuple_parse(input: &str) -> IResult<&str, (&str, &str, &str)> {
        tuple((tag("hello"), tag(","), tag("world")))(input)
    }
    println!("result tuple parse:{:?}", tuple_parse(msg));

    fn tmp(input: &str) -> IResult<&str, &str> {
        tag("hello")(input)
    }
    fn pair_1(input: &str) -> IResult<&str, (&str, &str)> {
        pair(tag("hello"), tag(","))(input)
    }

    let tmp = tmp(msg);

    println!("tag parser:{:?}", tmp);
    println!("pair 1 parser:{:?}", pair_1(msg));

    // 并联 alt
    fn alt_parser(input: &str) -> IResult<&str, &str> {
        alt((tag("hello"), tag("world")))(input)
    }
    println!("alt parser:{:?}", alt_parser(msg));

    // delimited 产生三个解析器 所有解析器成功后才算成功，成功后返回中间解析器的值
    fn delimited_parser(input: &str) -> IResult<&str, &str> {
        delimited(tag("hello"), tag(","), tag("world"))(input)
    }
    println!("delimited parser:{:?}", delimited_parser(msg));

    // separated_pair 产生三个解析器 所有解析器都成功后才算成功，返回第一个和第三个解析器中的值，忽略第二个解析器产生的值
    fn separated_pair_parser(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(tag("hello"), tag(","), tag("world"))(input)
    }
    println!("separated pair parser:{:?}", separated_pair_parser(msg));

    // proceded 产生两个解析器，所有解析器都成功才算成功，返回第二个解析器中的值
    fn proceded_parser(input: &str) -> IResult<&str, &str> {
        preceded(tag("hello"), tag(","))(input)
    }
    println!("proceded parser:{:?}", proceded_parser(msg));

    // terminated 产生两个解析器，所有解析器都成功才算成功，返回第一个解析器产生的值
    fn terminated_parser(input: &str) -> IResult<&str, &str> {
        terminated(tag("hello"), tag(","))(input)
    }
    println!("terminated parser:{:?}", terminated_parser(msg));

    // many0 产生0到多个匹配的解析器 类似于正则表达式中的*
    fn many0_parser(input: &str) -> IResult<&str, Vec<&str>> {
        many0(tag("hello"))(input)
    }
    println!(
        "many0 parser:{:?}",
        many0_parser("hello,hello,world,helloworld,hello")
    );

    // many1 产生1到多个匹配的解析器 类似与正则表达式中的+
    fn many1_parser(input: &str) -> IResult<&str, Vec<&str>> {
        many1(tag("hello"))(input)
    }
    println!("many1 parser:{:?}", many1_parser("helloworld,hello"));

    // peek 产生一个解析器 但是不消耗匹配的源
    fn peek_parser(input: &str) -> IResult<&str, &str> {
        peek(tag("hello"))(input)
    }
    println!("peek parser:{:?}", peek_parser(msg));

    // recognize 将子解析器解析成功的返回值拼接起来作为返回值
    fn recognize_parser(input: &str) -> IResult<&str, &str> {
        recognize(separated_pair(tag("hello"), tag("|"), tag("world")))(input)
    }
    println!(
        "recognize parser result:{:?}",
        recognize_parser("hello|world")
    );

    fn recognize_parser_1(input: &str) -> IResult<&str, &str> {
        recognize(separated_pair(tag("("), tag("你好"), tag(")")))(input)
    }
    println!("recognize parser result:{:?}", recognize_parser_1("(你好)"));

    // consume
    fn consume_parser(input: &str) -> IResult<&str, (&str, (&str, &str))> {
        consumed(separated_pair(tag("hello"), tag("|"), tag("world")))(input)
    }
    println!("sonsume parser:{:?}", consume_parser("hello|world"));
}

fn hello_parser(s: &str) -> IResult<&str, &str> {
    tag("hello")(s)
}

fn commer_parse(s: &str) -> IResult<&str, &str> {
    tag(",")(s)
}

fn world_parse(s: &str) -> IResult<&str, &str> {
    tag("world")(s)
}
