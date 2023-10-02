fn add_2(x: &i32) -> i32 {  // 這是fn，簽章是為了符合可以放進下面的map裡
    x + 2
}

fn main() {
    let a = vec![1, 2, 3];
    let b: Vec<i32> = a.iter().map(|x| x + 2).collect();
    let c: Vec<i32> = a.iter().map(add_2).collect();
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}
