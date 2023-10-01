fn main1() {
    let num: u8 = 200;               // u8 上限是 255
    let num2: u8 = 100;              // u8 上限是 255
    // println!("num: {}", num + num2); // u8 + u8 還是 u8，上限255
}

fn main2() {
    let num: u8 = 5;
    let input = "255";
    let num2: u8 = input.parse().unwrap();
    println!("num: {}", num + num2);
}

fn main() {
    let num: u16 = 300;
    let result = u8::try_from(num);
    println!("num: {:?}", result);
}