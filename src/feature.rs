fn ownership() {
    let s = String::from("1111");
    let s = test(s);
    println!("s: {}", s);
}

fn test(s: String) -> String {
    println!("s: {}", s);
    s
}