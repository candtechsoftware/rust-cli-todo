use std::env;

struct TodoItem {
    name: String,
    completed: char,
}
impl TodoItem {
    fn new(name: String) -> TodoItem {
        return TodoItem {
            name: name,
            completed: ' ',
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();

    let todo_item = TodoItem::new("Do someting in Rust".to_string());

    let todo_list = vec![todo_item];

    if command == "get" {
        for item in todo_list {
            println!("[{}] - {}", item.completed, item.name);
        }
    }
}
