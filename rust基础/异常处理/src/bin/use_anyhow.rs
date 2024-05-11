use std::{fs::File, io::Read};
use anyhow::{ Error, Ok, Result};
fn main() -> Result<()> {
    let result = test_fn();
    println!("{:?}",result);
    Ok(())
}

fn test_fn() -> Result<String,Error> {
    let data_path = std::env::var("DATA_PATH")?;
    let mut file = File::open(data_path)?;
    let mut buf = String::new();
    let _ = file.read_to_string(&mut buf);
    println!("content:{}", buf);
    Ok(buf)
}
