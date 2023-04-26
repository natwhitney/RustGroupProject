#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new_node(data: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Node<T> { 
        let new_node = Node{
            data: data,
            left: left,
            right: right,
        };
        new_node
    }
}