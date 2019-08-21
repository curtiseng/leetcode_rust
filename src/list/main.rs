struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>
}

struct List<T> {
    size: u32,
    head: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    fn new(elem: T) -> Self {
        Node {
            val: elem,
            next: None
        }
    }

    fn add(&mut self, index: u32, elem: T) {
        self.link_before(elem, self.node(index));
    }

    fn link_last(&mut self, elem: T) {

    }

    fn link_before(&mut self, elem: T, node: Self) {

    }

    // Returns the Option_Box_Node at the specified element index.
    fn node(self, index: u32) -> Self {

    }
}

fn main() {

}