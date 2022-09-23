use todo::TodoList;

mod todo;

fn main() {
    let mut todo_list = TodoList::new();
    let todo_id = todo_list.add_todo("asd".to_string());
    let todo_second_id = todo_list.add_todo("new task".to_string());
    println!("{:#?}", todo_list.filter_todos(true));
    todo_list.toggle_todo(&todo_id);
    println!("{:#?}", todo_list.filter_todos(false));
    todo_list.delete_todo(&todo_id);
    println!("{:#?}", todo_list.todos);
}
