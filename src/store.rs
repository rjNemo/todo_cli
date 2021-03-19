use crate::models::Todo;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Store {
    tasks: HashMap<usize, Todo>,
    index: usize,
}

impl Store {
    pub fn new() -> Store {
        let mut store = Store {
            tasks: HashMap::new(),
            index: 1,
        };
        // seed store
        store
            .tasks
            .insert(0, Todo::new(String::from("First thing to do")));
        store
    }

    pub fn add(&mut self, task: Todo) {
        self.tasks.insert(self.index, task);
        self.index += 1;
    }

    pub fn get(&self, index: &usize) -> Option<&Todo> {
        self.tasks.get(index)
    }

    pub fn remove(&mut self, index: &usize) {
        self.tasks.remove(index);
    }

    pub fn display(self) {
        println!("{:?}", self.tasks)
    }
}
