use colored::Colorize;
use std::io::{self, Write};

struct Task {
    title: String,
    description: String,
    is_done: bool,
}

impl Task {
    fn check(&mut self) {
        self.is_done = true;
    }

    fn uncheck(&mut self) {
        self.is_done = false;
    }

    fn new(title: String, description: String) -> Self {
        Task {
            title,
            description,
            is_done: false,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "[{}] {}\n\t{}",
            if self.is_done { "*" } else { " " },
            self.title,
            self.description
        )
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let options = [
        ("1", "Add task"),
        ("2", "Delete task"),
        ("3", "List tasks"),
        ("4", "Check task"),
        ("5", "Uncheck task"),
        ("Q", "Quit"),
    ];

    loop {
        print_menu(&options);
        let choice = read_input("\nChoose an option: ");
        match choice.as_str() {
            "1" => add_task(&mut tasks),
            "2" => delete_task(&mut tasks),
            "3" => {
                clear_screen();
                print_tasks(&tasks);
            }
            "4" => update_task_status(&mut tasks, true),
            "5" => update_task_status(&mut tasks, false),
            "Q" | "q" => break,
            _ => {
                clear_screen();
                println!("{}", "Invalid choice".red());
            }
        }
    }
}

fn add_task(tasks: &mut Vec<Task>) {
    let title = read_input("Enter task title: ");
    let description = read_input("Enter task description: ");
    tasks.push(Task::new(title, description));
    clear_screen();
    println!("{}", "Task added successfully".green());
}

fn delete_task(tasks: &mut Vec<Task>) {
    if tasks.is_empty() {
        println!("{}", "No tasks to delete.".red());
        return;
    }

    print_tasks(tasks);
    if let Some(index) = read_task_index(tasks.len(), "Enter task number to delete: ") {
        tasks.remove(index);
        clear_screen();
        println!("{}", "Task deleted successfully".green());
    }
}

fn update_task_status(tasks: &mut Vec<Task>, is_check: bool) {
    if tasks.is_empty() {
        println!("{}", "No tasks available.".red());
        return;
    }

    print_tasks(tasks);
    let action = if is_check { "check" } else { "uncheck" };
    if let Some(index) = read_task_index(tasks.len(), &format!("Enter task number to {}: ", action)) {
        let task = &mut tasks[index];
        if is_check {
            task.check();
        } else {
            task.uncheck();
        }
        clear_screen();
        print_tasks(tasks);
        println!("\n{}", format!("Task {}ed successfully", action).green());
    }
}

fn read_task_index(task_count: usize, prompt: &str) -> Option<usize> {
    let input = read_input(prompt);
    match input.trim().parse::<usize>() {
        Ok(index) if index < task_count => Some(index),
        _ => {
            println!("{}", "Invalid task number. Please try again.".red());
            None
        }
    }
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt.blue());
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}

fn print_menu(options: &[(&str, &str)]) {
    println!("\nPlease choose an option: ");
    for (key, value) in options {
        println!("{} - {}", key, value);
    }
}

fn print_tasks(tasks: &[Task]) {
    if tasks.is_empty() {
        println!("\n{}", center_text("No tasks available", 50, '=').red());
    } else {
        println!("\n{}", center_text("Tasks", 50, '=').green());
        for (i, task) in tasks.iter().enumerate() {
            println!("{} - {}", i, task.to_string());
        }
        println!("{}", "=".repeat(50).green());
    }
}

fn center_text(text: &str, width: usize, fillchar: char) -> String {
    let padding = width.saturating_sub(text.len());
    let left_pad = padding / 2;
    let right_pad = padding - left_pad;
    format!(
        "{}{}{}",
        fillchar.to_string().repeat(left_pad),
        text,
        fillchar.to_string().repeat(right_pad)
    )
}
