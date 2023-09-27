struct Student {   // 結構體，理論上應該一起存活
no: i32,       // 編譯時已知大小，放Stack，使用Copy trait傳遞（Call by Value）
name: String,  // 編譯時未知大小，放Heap，使用Call by reference
}

fn move_string(s: String) {    // 寫一個fn把s的所有權拿走
    s.to_string();    // 這行執行完s就被丟棄(drop)了
}

fn main() {
    let student = Student { no: 1, name: "John".to_string() };
    move_string(student.name);    // 把name移走
    println!("student no, {}!", student.no);
    // println!("student name, {}!", student.name);
}