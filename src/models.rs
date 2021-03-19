#[derive(Debug)]
pub struct Todo {
    id: usize,
    title: String,
    is_done: bool,
}

impl Todo {
    pub fn new(title: String) -> Todo {
        Todo {
            id: 0,
            title,
            is_done: false,
        }
    }
}
