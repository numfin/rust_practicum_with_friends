pub struct TodoList {
    pub todos: Vec<Todo>,
}
impl TodoList {
    pub fn new() -> Self {
        Self { todos: vec![] }
    }
    pub fn add_todo(&mut self, name: String) -> String {
        self.todos.push(Todo::new(name));
        self.todos.last().unwrap().id.clone()
    }
    pub fn delete_todo(&mut self, id: &String) {
        let todo_index = self.todos.iter().position(|todo| &todo.id == id);
        if let Some(todo_index) = todo_index {
            self.todos.remove(todo_index);
        }
    }
    pub fn toggle_todo(&mut self, id: &String) {
        let todo = self.todos.iter_mut().find(|todo| &todo.id == id);
        if let Some(v) = todo {
            v.toggle_todo()
        }
    }
    pub fn filter_todos(&self, show_done: bool) -> Vec<&Todo> {
        self.todos
            .iter()
            .filter(|todo| if show_done { true } else { !todo.is_done })
            .collect()
    }
}

#[derive(Debug)]
pub struct Todo {
    pub id: String,
    pub is_done: bool,
    pub name: String,
}
impl Todo {
    pub fn new(name: String) -> Self {
        Self {
            id: cuid::cuid().unwrap(),
            name,
            is_done: false,
        }
    }
    pub fn toggle_todo(&mut self) {
        self.is_done = !self.is_done;
    }
}
