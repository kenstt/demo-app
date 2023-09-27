struct Student {   // 結構體，理論上應該一起存活
no: i32,       // 編譯時已知大小，放Stack，使用Copy trait傳遞（Call by Value）
name: String,  // 編譯時未知大小，放Heap，使用Call by reference
}

fn move_string(s: String) {    // 寫一個fn把s的所有權拿走
    s.to_string();    // 這行執行完s就被丟棄(drop)了
}

fn main1() {
    let student = Student { no: 1, name: "John".to_string() };
    move_string(student.name);    // 把name移走
    println!("student no, {}!", student.no);
    // println!("student name, {}!", student.name);
}

struct LifeStudent<'a> {    // 這裡的撇a就是生命週期參數 你也可以撇b撇c都可以
    no: &'a i32,            // 寫法是要給個&再接'a
    name: &'a String,       // 現在name我們要求要活的跟no 或跟struct一樣久
}

fn move_student(s: LifeStudent) {
    s.no;
}

fn main_struct() {
    let student = LifeStudent { no: &1, name: &"John".to_string() };
    move_string(student.name.to_string());
    println!("student no, {}!", student.no);
    println!("student name, {}!", student.name);
    move_student(student);                       // 這裡move student
    // println!("student no, {}!", student.no);     // 移除會報錯
    // println!("student name, {}!", student.name); // 移除會報錯
}

fn longer_string(s1: String, s2: String) -> String { // 取長的字串
    if s1.len() > s2.len() {
        s1        // 把 s1 所有權移交給fn呼叫者
    } else {
        s2        // 把 s2 所有權移交給fn呼叫者
    }             // s1, s2 都被 drop
}

fn main() {
    let a = String::from("Rust");
    let b = String::from("Svelte");
    // let longer = longer_string(a, b);    // a 和 b 都被move了
    let longer = longer_string(a.clone(), b.clone());

    println!("a: len: {:2}, {:?}", a.len(), a); // :2 是佔用2格位置
    println!("b: len: {:2}, {:?}", b.len(), b); // :? 是Debug，字串會加"
    println!("   longer is >> {:?} <<", longer);
}