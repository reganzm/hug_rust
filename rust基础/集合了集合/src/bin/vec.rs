fn main() {
    // 动态数组
    // 使用vec!宏创建
    let v = vec![2, 3, 4, 5, 6, 76];
    println!("v:{:?}", v);
    // 使用关联函数new创建
    let mut vv: Vec<i32> = Vec::new();
    // 使用push添加元素
    vv.push(1);
    vv.push(2);
    vv.push(3);
    println!("vv:{:?}", vv);

    // 从动态数组中获取元素
    println!("第6个元素:{:?}", v.get(6));
    // 下面代码会panic，因为下标越界了
    // println!("第6个元素:{:?}",&v[6]);

    // 可变借用和不可变借用同时存在
    // 不可变借用
    let last_element = vv.last();
    // 可变借用增加数据
    vv.push(989);
    // 注释掉下面的代码，程序正常运行
    // 如果打开注释，last_element是不可变借用
    // vv.push是可变借用，违反借用检查规则
    //println!("last element:{:?}",last_element);

    // 遍历动态数组元素
    // 不可变借用遍历
    for ele in &vv {
        println!("ele1:{ele}");
    }
    // 可变借用遍历
    for ele in &mut vv {
        // 修改值
        *ele += 1;
        println!("ele2:{ele}");
    }
    // 带走所有权，后面将不能访问vv
    for ele in vv {
        println!("ele3:{ele}");
    }
    // 报错，vv所有权已被移走
    //println!("vv:{:?}",vv);

    // Vec存储不同类型的对象
    // 第一种：存储枚举值
    let mut pens = vec![];
    pens.push(Pen::Pencil("铅笔".to_string()));
    pens.push(Pen::ColorPen("彩色笔".to_string()));
    for pen in &pens {
        println!("{:?}", pen);
    }
    // 第二种：存储特征对象
    let pens: Vec<Box<dyn Write>> = vec![
        Box::new(Pencil {
            name: "铅笔".to_string(),
        }),
        Box::new(ColorPen {
            name: "彩色笔".to_string(),
        }),
    ];
    for pen in &pens {
        pen.write();
    }

    // 排序

    let mut num1 = vec![3, 4, 5, 6, 77, 9, 1, 2, 910];
    num1.sort();
    println!("升序排列：{:?}", num1);
    // 倒序
    num1.reverse();
    println!("降序排列：{:?}", num1);
    // 调用sort_unstable方法
    num1.sort_unstable();
    println!("升序排列：{:?}", num1);
    let mut num2 = vec![1.1, 2.5, 0.78, 908.9];
    // float数组，调用sort方法会报错，原因是
    // float没有实现Ord特征
    // 根本原因是浮点数没有实现全数值可比较，因为浮点数中有NaN值
    // 但浮点数实现了部分可比较，当确定数组中不含NaN时，可以使用partial_cmp实现比较
    //num2.sort();
    num2.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("num2 升序排列:{:?}", num2);
    num2.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("num2 降序排列:{:?}", num2);

    // 对结构体进行排序
    #[derive(Debug)]
    struct Animal(i32, String);

    let mut animals = vec![
        Animal(1, "dog".to_string()),
        Animal(99, "pig".to_string()),
        Animal(98, "elephant".to_string()),
        Animal(100, "hive".to_string()),
    ];
    // 按动物的物种名称降序排列
    animals.sort_by(|a, b| b.1.cmp(&a.1));
    println!("按物种名称降序:{:?}", animals);

    // vec上常见的方法使用
    // 初始一个带10个容量的Vec数组
    // 动态增加元素如果容量不足，会触发扩容。重新申请一块2倍大小的内存
    // 在将所有元素拷贝到新的内存为止，同时更新vec在栈中的指针
    // 频繁的扩容会降低性能，因此预定义一个合适的容量尤为重要
    let mut num1: Vec<i32> = Vec::with_capacity(10);
    println!(
        "扩容前数据内存地址:{:p} 容量:{} 数组长度:{}",
        &(*num1),
        num1.capacity(),
        num1.len()
    );
    // 批量添加数据
    num1.extend([1, 3, 5, 33, 34, 99, 88, 77, 66]);
    // 扩容到20
    num1.reserve(20);
    // 可以发现扩容前后内存地址不一样，证明重新申请了内存并拷贝了数据
    println!(
        "扩容后数据内存地址:{:p} 容量:{} 数组长度:{}",
        &(*num1),
        num1.capacity(),
        num1.len()
    );
    // 释放剩余的容量
    num1.shrink_to_fit();
    println!("释放容量后容量:{} 数组长度:{}", num1.capacity(), num1.len());

    // 断言是否为空
    assert!(!num1.is_empty());
    // 指定索引插入数据
    num1.insert(1, 9999999);
    assert!(9999999 == *num1.get(1).unwrap());
    // 删除指定位置的数据
    num1.remove(1);
    // 删除并返回尾部的数据,如果没有数据返回None
    let tail_num = num1.pop();
    println!("tail num:{:?}", tail_num);
    // 筛选满足条件的数据，并删除不满足条件的数据
    num1.retain_mut(|x| *x > 10);
    println!("删除小于等于10的数据后:{:?}", num1);
    // 删除指定范围的数据,返回被删除数据的迭代器
    let del_eles: Vec<i32> = num1.drain(0..2).collect();
    println!("被删除的元素:{:?}", del_eles);
    // 支持切片，用切片获取连续的数据
    let slice: &[i32] = &num1[0..2];
    assert!(slice == &[99, 88]);
    // 清空
    num1.clear();
}

#[derive(Debug)]
enum Pen {
    Pencil(String),
    ColorPen(String),
}

trait Write {
    fn write(&self);
}
struct Pencil {
    name: String,
}
impl Write for Pencil {
    fn write(&self) {
        println!("我用{}写字", self.name.as_str())
    }
}
struct ColorPen {
    name: String,
}
impl Write for ColorPen {
    fn write(&self) {
        println!("我用{}写字", self.name.as_str())
    }
}
