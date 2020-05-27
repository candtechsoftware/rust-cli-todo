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

struct TodoList {
    list: Vec<TodoItem>,
}

impl TodoList {
    fn new() -> TodoList {
        return TodoList { list: Vec::new() };
    }

    fn add_todo_list(&mut self, name: String) {
        let todo_item = TodoItem::new(name);
        self.list.push(todo_item);
    }

    fn mark_as_done(&mut self, index: usize) {
        self.list[index].completed = 'x'
    }

    fn print(&self) {
        for (index, item) in self.list.iter().enumerate() {
            println!("{} [{}] - {}", index, item.completed, item.name);
        }
    }
}

enum Command {
    Get,
    Add(String),
    Done(usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = match args[1].as_str() {
        "get" => Command::Get,
        "add" => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].parse().expect("Error in entering the right type")),
        _ => panic!("You must provide an accepted command"),
    };

    let mut todo_list = TodoList::new();

    todo_list.add_todo_list("Do Something With Rust".to_string());
    todo_list.add_todo_list("Create a CLI with Rust".to_string());

    match command {
        Command::Get => todo_list.print(),
        Command::Add(task) => {
            todo_list.add_todo_list(task);
            todo_list.print();
        }
        Command::Done(index) => {
            todo_list.mark_as_done(index);
            todo_list.print();
        }
    }
}
