fn main() {
    // or 和 and方法
    // Option
    let s1 = Some("123");
    let s2 = Some("234");
    let s3: Option<&str> = None;
    assert_eq!(s1.or(s2), s1);
    assert_eq!(s1.and(s2), s2);
    assert_eq!(s1.and(s3), s3);
    assert_eq!(s1.or(s3), s1);
    // Result
    let ok1: Result<&str, &str> = Ok("ok1");
    let ok2: Result<&str, &str> = Ok("ok2");
    let err1: Result<&str, &str> = Err("err1");
    let err2: Result<&str, &str> = Err("err2");
    assert_eq!(ok1.and(ok2), ok2);
    assert_eq!(ok1.or(ok1), ok1);
    assert_eq!(ok1.and(err1), err1);
    assert_eq!(ok1.or(err1), ok1);
    assert_eq!(err1.and(err2), err1);
    assert_eq!(err1.or(err2), err2);

    // or_else 和 and_then
    let fn_some = || Some("234");
    let fn_some2 = |_| Some("234");
    assert_eq!(s2.or_else(fn_some), s2);
    assert_eq!(s2.and_then(fn_some2), fn_some());

    let fn_ok1 = |_| Ok("ok1");
    let fn_ok2 = |_| Ok("ok2");
    let fn_err1 = |_| Err("err1");
    let fn_err2 = |_| Err("err2");
    assert_eq!(ok1.or_else(fn_ok1), ok1);
    assert_eq!(ok2.and_then(fn_ok2), ok2);
    assert_eq!(err1.or_else(fn_err1), err1);
    assert_eq!(err2.and_then(fn_err2), err2);

    // 过滤和转换
    // Option过滤
    let closure_fn = |x: &&str| x.starts_with("123");
    assert_eq!(s1.filter(closure_fn), Some("123"));
    assert_eq!(s2.filter(closure_fn), None);

    // Option和Result map和map_err转换
    // map
    let char_count = |s: &str| -> usize { s.chars().count() };
    assert_eq!(s1.map(char_count), Some(3));
    assert_eq!(s3.map(char_count), None);
    assert_eq!(ok1.map(char_count), Ok(3));
    assert_eq!(err1.map(char_count), Err("err1"));

    // map_err
    let map_err_char_count = |s: &str| s.chars().count();
    assert_eq!(ok1.map_err(map_err_char_count), Ok("ok1"));
    assert_eq!(err1.map_err(map_err_char_count), Err(4));

    // Option和Result map_or和map_or_else提供默认值
    // map_or
    const DEFAULT: usize = 1;
    let fn_closure = |s: &str| s.chars().count();
    assert_eq!(s1.map_or(DEFAULT, fn_closure), 3);
    assert_eq!(s3.map_or(DEFAULT, fn_closure), 1);
    assert_eq!(ok1.map_or(DEFAULT, fn_closure), 3);
    assert_eq!(err1.map_or(DEFAULT, fn_closure), 1);

    // map_or_else
    // 这个闭包可以对Err值进行处理
    let fn_closure_default_value = |s: &str| s.chars().count() + 100;
    assert_eq!(err1.map_or_else(fn_closure_default_value, fn_closure), 104);

    // 将Option转换为Result
    // ok_or 和 ok_or_else

    assert_eq!(s1.ok_or("Error Message"), Ok("123"));
    assert_eq!(s3.ok_or("Error Message"), Err("Error Message"));

    let fn_closure = || "Error Message";
    assert_eq!(s1.ok_or_else(fn_closure), Ok("123"));
    assert_eq!(s3.ok_or_else(fn_closure), Err("Error Message"));
}
