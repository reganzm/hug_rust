use bigdecimal::BigDecimal;

fn main() {
    let two = BigDecimal::from(2);
    println!("sqrt(2) = {}", two.sqrt().unwrap());

    let three = BigDecimal::from(3);
    println!("sqrt(3) = {}", three.sqrt().unwrap());

    let four = BigDecimal::from(4);
    println!("sqrt(4) = {}", four.sqrt().unwrap());
}
