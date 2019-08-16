struct Stack {
    items: Vec<String>,
    count: i32,
    n: i32
}

fn main() {
    let mut stack = Stack::new(2);
    stack.push(String::from("hello"));
    stack.push(String::from("world"));
    println!("count: {}", stack.count);
    println!("item: {}", stack.pop());
    println!("item: {}", stack.pop());
    println!("item: {}", stack.pop());
}

impl Stack {
    fn new(n: i32) -> Self {
        Stack {
            items: Vec::new(),
            count: 0,
            n
        }
    }

    fn pop(&mut self) -> String {
        if self.count == 0 {
            println!("Stack has been empty!!!");
        }
        self.count = self.count - 1;
        self.items.pop().unwrap_or_else(||String::from("Stack has been empty!!!"))
    }

    fn push(&mut self, x: String) {
        if self.count == self.n {
            println!("Overflow stack size!!!");
            return;
        }
        self.items.push(x);
        self.count = self.count + 1;
    }
}


