use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{io::Result};

mod file;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add(AddTask),
    Toggle(ToggleTask),
    Remove(RemoveTask),
    Show(ShowTask),
}

#[derive(Args, Debug)]
struct AddTask {
    /// The name of the task to add
    #[arg(short, long)]
    task: String,

    #[arg(short, long)]
    description: Option<String>,

    #[arg(long, default_value_t = false)]
    is_done: bool,
}

#[derive(Args, Debug)]
struct ToggleTask {
    #[arg(short, long)]
    id: u32,

    #[arg(long)]
    is_done: bool,
}

#[derive(Args, Debug)]
struct RemoveTask {
    #[arg(short, long)]
    id: u32,
}

#[derive(Args, Debug)]
struct ShowTask {
    /// Optional ID of the task to show. If not provided, all tasks will be shown.
    #[arg(short, long)]
    id: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    name: String,
    is_done: bool,
    description: Option<String>,
}

fn next_id(todos: &[Task]) -> u32 {
    todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut todos = file::load()?;

    match cli.command {
        Commands::Add(add_task) => {
            let task = Task {
                id: next_id(&todos),
                name: add_task.task,
                is_done: add_task.is_done,
                description: add_task.description,
            };

            todos.push(task.clone());
        }

        Commands::Toggle(toggle_task) => {
            if let Some(task) = todos.iter().find(|t| t.id == toggle_task.id) {
                let mut updated_task = task.clone();
                updated_task.is_done = toggle_task.is_done;

                if let Some(pos) = todos.iter().position(|t| t.id == toggle_task.id) {
                    todos[pos] = updated_task;
                }
            } else {
                println!("Task with ID {} not found.", toggle_task.id);
            }
        }

        Commands::Remove(remove_task) => {
            let task = Task {
                id: remove_task.id,
                name: "Sample Task".into(),
                is_done: false,
                description: None,
            };

            println!("Remove command: {:?}", task.id);
        }

        Commands::Show(show_task) => {
            if let Some(id) = show_task.id {
                let task = Task {
                    id,
                    name: "Sample Task".into(),
                    is_done: false,
                    description: None,
                };

                println!("Show command for task ID: {:?}", task.id);
            } else {
                println!("Show command for all tasks.");
            }
        }
    }

    file::save(&todos)?;

    Ok(())
}
