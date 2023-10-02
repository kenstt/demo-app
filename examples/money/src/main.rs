use rust_decimal::prelude::*;
use rust_decimal_macros::dec;

fn main() {
    let a = 0.1;
    let b = 0.1;
    let c = 0.1;

    println!("a + b + c = {}", a + b + c);
    println!("a / 0.01 = {}", a / 0.03 );

    let d1 = dec!(0.1);
    let d2 = dec!(0.1);
    let d3 = dec!(0.1);

    println!("d1 + d2 + d3 = {}", d1 + d2 + d3);
    let d4 = dec!(0.03);
    println!("d1 / d4 = {}", d1 / d4 );
}
