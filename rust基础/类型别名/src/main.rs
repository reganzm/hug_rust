

fn main() {
    type Hour = u8;
    type Minute = u8;

    let a: u8 = 23;
    let hour: Hour = 23;
    println!("a == hour : {}", a == hour);
    let b: u8 = 59;
    let minute: Minute = 59;
    println!("b == minute : {}", b == minute);

}
