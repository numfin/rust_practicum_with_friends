use practicum::todo::TodoList;

#[test]
fn can_create_todo() {
    let mut todo_list = TodoList::new();
    todo_list.add_todo("Do smth");
    assert_eq!(todo_list.todos.len(), 1)
}
