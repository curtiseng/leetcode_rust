struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>
}

/// Why we need a List struct(or head, or sentinel) in here?
/// When we pop first node, the node would be `null`. Rust's pointer types must always
/// point to a valid location; there are no `null` pointers. So we can't pop first node in rust,
/// that is the reason that why we need a list struct.In other language, a sentinel would
/// be useful that move towards the simpler implement.
struct List<T> {
    size: u32,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            val: elem,
            next: None
        }
    }

    fn add(&mut self, elem: T) -> bool {
        self.link_last(elem);
        true
    }

    fn insert(&mut self, index: u32, elem: T) -> bool {
        self.link_before(elem, self.node(index));
        true
    }

    fn remove(&mut self, elem: T) -> bool {
        true
    }

    fn link_last(&mut self, elem: T) {

    }

    fn link_before(&mut self, elem: T, node: Self) {

    }

    // Returns the Option_Box_Node at the specified element index.
    fn node(self, index: u32) -> Self {

    }
}

impl<T> List<T> {
    fn new () -> Self {
        List {
            size : 0,
            next: None
        }
    }

    pub fn size(&self) -> u32 {
        self.size
    }
}

fn main() {

}