//!实现一个通用的密码生成器

use anyhow::{bail, Error, Ok};
use base64::prelude::*;
use clap::Parser;
use rand::prelude::*;

// 计算seed种子的hash值
fn seed_hash(seed: &str) -> usize {
    let mut hash: usize = 0;
    for (i, c) in seed.chars().enumerate() {
        hash += (i + 1) * (c as usize);
    }
    (hash % 31).pow(3)
}

// 密码子,随机的一串字符串，用于生成密码时，从中挑选字符
const CRYPTO:&str = "!@#$%^&*()POIUYTREWQSDAFSGJKZBXNCMVBL<:{}[]|abcedljloweiunbaljkfl1289829179438fsdljflfjasfdsbfJKFLSAFLSAJFELJ*(&(&(*&*^&^%$%$^&^*";

// 产生密码的方法

fn generate_password(seed: &str, length: usize) -> Result<String, Error> {
    // 密码长度不能太短，至少6位数
    if length < 6 {
        bail!("密码长度必须大于6位");
    } else {
        let mut hash = seed_hash(seed);

        // 由hash求passwd
        let mut passwd = String::new();
        let crypto_len = CRYPTO.len();
        while hash > 0 {
            let index = hash % crypto_len;
            let nthc = CRYPTO.chars().nth(index).expect("从密码子中获取字符错误");
            passwd.push(nthc);
            hash /= crypto_len;
        }

        // 将seed和passwd拼接起来
        for c in seed.chars() {
            passwd.push(c);
        }

        // 将passwd编码为base64
        passwd = BASE64_STANDARD.encode(passwd);
        // 替换掉字符串中的+和/为*
        passwd = passwd.replace("+", "*").replace("/", "*");
        if passwd.len() < length {
            let mut rng = rand::thread_rng();
            // 满足长度要求
            while passwd.len() < length {
                let index = rng.gen_range(0..CRYPTO.len());
                passwd.push(CRYPTO.chars().nth(index as usize).expect("获取字符错误"));
            }
        }
        // 返回length个字符作为密码
        Ok(format!("seed:{} pwd:{}", seed, &passwd[..length]))
    }
}




// 命令行解析
#[derive(Debug,Parser)]
#[clap(version,about)]
struct Args{
    //生成密码的种子
    #[clap(short,long)]
    seed:String,

    // 密码长度
    #[clap(short,long,default_value_t=15)]
    length:usize

}
fn main()->Result<(),Error> {

    let args = Args::parse();
    // 种子长度必须大于等于4
    if args.length < 4{
        bail!("种子:{} 长度太短，请换一个",&args.seed);
    }

    let (seed,lenght) = (args.seed,args.length);


    let result = generate_password(&seed, lenght);
    println!("{:?}", result);
    Ok(())
}
