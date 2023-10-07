#![allow(clippy::all)]
#![allow(dead_code, unused_variables)]

fn add_2(x: &i32) -> i32 {  // 這是fn，簽章是為了符合可以放進下面的map裡
    x + 2
}

fn main1() {
    let a = vec![1, 2, 3];
    let b: Vec<i32> = a.iter().map(|x| x + 2).collect();
    let c: Vec<i32> = a.iter().map(add_2).collect();
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
}

fn main2() {
    let a = vec![1, 2, 3];
    let times_2 = |x| x * 2;    // closure直接assign給變數
    let b: Vec<i32> = a.iter().map(|x| x * 2).collect();
    let c: Vec<i32> = a.iter().map(times_2).collect();
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    // let f = vec![1.2,3.4];
    // let d: Vec<f64> = f.iter().map(times_2).collect();
}

fn main() {
    let a = vec![1, 2, 3];
    let b: Vec<i32> = a.iter().map(|x| {
        println!("正在處理 x: {:?}", x);
        x + 2    // 多行的closure，記得最後一行表達式是回傳值
    }).collect();
    let c: Vec<i32> = a.iter()
        .inspect(|x| println!(" x在map前: {:?}", x))
        .map(|x| x + 2)
        .inspect(|x| println!(" x在map後: {:?}", x))
        .collect();
    let d: Vec<i32> = a.iter().map(|_| 0).collect();
    // let e: Vec<i32> = a.iter().map(|| 0).collect();
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    println!("d: {:?}", d);
}