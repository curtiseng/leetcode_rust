fn ownership(s: String) -> String {
    println!("s: {}", s);
    s
}

/// 所有权、引用、可变引用

fn main() {
    let s = String::from("1111");
    let s = ownership(s);
    println!("s: {}", s);
}