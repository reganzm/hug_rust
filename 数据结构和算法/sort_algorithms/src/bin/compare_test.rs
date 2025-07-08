use std::time::{Duration, Instant};

use bubbule_sort::bubbule_sort;
use shell_sort::shell_sort;
use quick_sort::quick_sort;
use insertion_sort::insert_sort;
use merge_sort::merge_sort;
use select_sort::select_sort;
use heap_sort::heap_sort;
use counting_sort::counting_sort;
use bucket_sort::bucket_sort;
use radix_sort::radix_sort;
use rand::{thread_rng, Rng};
use std::thread;

mod bubbule_sort;
mod bucket_sort;
mod counting_sort;
mod heap_sort;
mod insertion_sort;
mod merge_sort;
mod quick_sort;
mod radix_sort;
mod select_sort;
mod shell_sort;



fn gen_large_nums_usize()->Vec<usize>{
    let mut rng = thread_rng();
    let mut random_nums = Vec::with_capacity(100000);
    for _ in 0..100000{
        random_nums.push(rng.gen_range(usize::MIN..=usize::MAX));
    }
    random_nums
}



fn gen_large_nums_i32()->Vec<i32>{
    let mut rng = thread_rng();
    let mut random_nums = Vec::with_capacity(100000);
    for _ in 0..100000{
        random_nums.push(rng.gen_range(i32::MIN..=i32::MAX));
    }
    random_nums
}


fn main(){
    let start  = Instant::now();
    bubbule_sort(&mut gen_large_nums_i32());
    println!("冒泡排序耗时:{:?}",start.elapsed());

    let start  = Instant::now();
    shell_sort(&mut gen_large_nums_i32());
    println!("希尔排序耗时:{:?}",start.elapsed());

    let start  = Instant::now();
    quick_sort(&mut gen_large_nums_i32(),0,99999);
    println!("快速排序耗时:{:?}",start.elapsed());

    let start  = Instant::now();
    insert_sort(&mut gen_large_nums_i32());
    println!("插入排序耗时:{:?}",start.elapsed());

    let start  = Instant::now();
    merge_sort(&mut gen_large_nums_i32());
    println!("归并排序耗时:{:?}",start.elapsed());

    let start  = Instant::now();
    select_sort(&mut gen_large_nums_i32());
    println!("选择排序耗时:{:?}",start.elapsed());

    let start  = Instant::now();
    heap_sort(&mut gen_large_nums_i32());
    println!("堆排序耗时:{:?}",start.elapsed());



    let start  = Instant::now();
    bucket_sort(&mut gen_large_nums_i32(),|x|x/29);
    println!("桶排序耗时:{:?}",start.elapsed());


    // 计数排序和基数排序不适合这个测试，且通常在元素个数小于10000的情况下使用

    // let start  = Instant::now();
    // counting_sort(&mut gen_large_nums_usize());
    // println!("计数排序排序耗时:{:?}",start.elapsed());

    // let start  = Instant::now();
    // radix_sort(&mut gen_large_nums_usize());
    // println!("基数排序耗时:{:?}",start.elapsed());

    thread::sleep(Duration::from_secs(100));
    
   
}