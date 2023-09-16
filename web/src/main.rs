fn main() {
    println!("Hello, world!");
    println!("call core fn from server: {}",core::add(2, 2));
    println!("call service fn from server: {}",service::add(2, 2));
}
