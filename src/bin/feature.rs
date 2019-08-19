fn ownership(s: String) -> String {
    println!("s: {}", s);
    s
}

fn main() {
    let s = String::from("1111");
    let s = ownership(s);
    println!("s: {}", s);
}