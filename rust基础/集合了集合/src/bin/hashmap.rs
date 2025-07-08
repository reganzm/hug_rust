use std::collections::HashMap;

fn main() {
    // 使用关联函数new创建
    let mut map = HashMap::new();
    map.insert("lucky".to_string(), 95);
    map.insert("lily".to_string(), 100);
    println!("map:{:?}", map);

    // 使用迭代器+collect创建
    let props = vec![("lucky".to_string(), 95), ("lily".to_string(), 100)];
    let mut map: HashMap<_, _> = props.into_iter().collect();
    println!("map:{:?}", map);

    // 所有权的转移
    let name = "pop".to_string();
    let score = 88;
    map.insert(name, score);
    println!("map:{:?}", map);
    // 下面代码name报错，name是String类型没有实现Copy,发生所有权转移，不能再继续使用
    // age是标量，实现了Copy不会发所有权转移
    //println!("name:{name} age:{score}");


    // HashMap查询和更新

    // 使用get按key查询，查询不到返回None，查询到返回Some(T)
    let score = map.get("pip");
    println!("score:{:?}",score);
    let score = map.get("pop");
    println!("score:{:?}",score);
    // 返回值是Option枚举类型，如果要获取值可以使用unwrap方法
    println!("score :{}",score.unwrap());

    // 按key更新HashMap中的值
    map.insert("pip".to_string(), 88);
    // 查询pipp，若不存在则插入新值
    let value = map.entry("pipp".to_string()).or_insert(199);
    // 返回了一个可变引用，可以借助它来更新值
    *value = *value + 100;
    println!("插入新值:{:?}",map.get("pipp"));



}
