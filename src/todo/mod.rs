use std::collections::HashMap;

pub struct TodoList<'a> {
    pub todos: HashMap<String, Todo<'a>>,
}
impl<'a> TodoList<'a> {
    pub fn new() -> Self {
        Self {
            todos: Default::default(),
        }
    }
    pub fn add_todo(&mut self, name: &'a str) -> String {
        let todo = Todo::new(name);
        let inserted_id = todo.id.clone();
        self.todos.insert(inserted_id.clone(), todo);
        inserted_id
    }
    pub fn delete_todo(&mut self, id: &str) {
        if self.todos.remove(id).is_some() {
            println!("Todo was deleted from list")
        }
    }
    pub fn toggle_todo(&mut self, id: &str) {
        self.todos
            .get_mut(id)
            .map(|todo| todo.toggle_todo())
            .unwrap();
    }
    pub fn filter_todos(&self, show_everything: bool) -> Vec<&Todo<'a>> {
        self.todos
            .iter()
            .map(|(_, todo)| todo)
            .filter(|todo| match todo.status {
                TodoStatus::Pending => true,
                _ => show_everything,
            })
            .collect()
    }
}

impl Default for TodoList<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Todo<'a> {
    pub id: String,
    pub status: TodoStatus,
    pub name: &'a str,
}
impl<'a> Todo<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            id: cuid::cuid().unwrap(),
            name,
            status: TodoStatus::Pending,
        }
    }
    pub fn toggle_todo(&mut self) {
        self.status = match self.status {
            TodoStatus::Pending => TodoStatus::Done,
            TodoStatus::Done => TodoStatus::Pending,
        };
    }
}

#[derive(Debug)]
pub enum TodoStatus {
    Done,
    Pending,
}
