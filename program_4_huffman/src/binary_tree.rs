struct node<T> {
    data: T,
    left: Option<Box<node<T>>>,
    right: Option<Box<node<T>>>,
}

