#![allow(clippy::all)]
#![allow(dead_code, unused_variables)]

// fn fn_once_say_hello(f: impl FnOnce(String) -> String) -> String
fn fn_once_say_hello<F>(f: F) -> String
    where F: FnOnce(String) -> String,
{
    let greeting = f("Hello".to_string());
    greeting + "!"
}

fn fn_say_hello<F>(f: F) -> String
    where F: Fn(String) -> String,
{
    let greeting = f("Hello".to_string());
    greeting + "!"
}

fn fn_mut_say_hello<F>(mut f: F) -> String
    where F: FnMut(String) -> String,
{
    let greeting = f("Hello".to_string());
    greeting + "!"
}

fn main() {
    let mut student = String::from("Nico");
    let greeting = fn_mut_say_hello(|g| {
        student.insert_str(0, "gorgeous ");
        "".to_string() + &student + ": " + &g
    });
    println!("[FnMut] greeting: {}", greeting);
    println!("[FnMut] student: {}", student);
}


fn main_fn() {
    let student = String::from("Nico");
    let once = fn_say_hello(|g| {
        // student.insert_str(0, "gorgeous ");
        // student + " " + &g               // student moved here
        "".to_string() + &student + ": " + &g
    });
    println!("[Fn] greeting: {}", once);
}

fn main_once() {
    let student = String::from("Nico");
    let once = fn_once_say_hello(|greeting| "".to_string() + &student + ": " + &greeting);
    let once_2 = fn_once_say_hello(|greeting| student + ": " + &greeting);
    println!("once: {}", once);
    println!("once_2: {}", once_2);
    // println!("student: {}", student);
}
