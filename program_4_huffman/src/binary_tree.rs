#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Node<T> where T: Ord
{
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> where T: Ord{
    pub fn get_info(&self) -> (&T, &Option<Box<Node<T>>>, &Option<Box<Node<T>>>) {
        (&self.data, &self.left, &self.right)
    }

    pub fn new_node(data: T, left: Option<Box<Node<T>>>, right: Option<Box<Node<T>>>) -> Node<T> { 
        let new_node = Node{
            data: data,
            left: left,
            right: right,
        };
        new_node
    }

    pub fn traverse_tree() {

    }
}