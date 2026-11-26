use clap::{ArgGroup, Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fmt, io::Result};

mod file;
mod help;

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
    Mark(MarkTask),
    Remove(RemoveTask),
    Show(ShowTask),
    Search(Search),
}

#[derive(Args, Debug)]
struct AddTask {
    /// The name of the task to add
    task: String,

    #[arg(short, long)]
    description: Option<String>,

    /// Marked the task as done
    #[arg(long, default_value_t = false)]
    done: bool,
}

#[derive(Args, Debug)]
struct MarkTask {
    id: u32,

    /// Mark the task as done (by default if neither "done" or "undone" is set)
    #[arg(long, conflicts_with = "undone")]
    done: bool,

    /// Mark the task as undone
    #[arg(long)]
    undone: bool,
}

#[derive(Args, Debug)]
struct RemoveTask {
    id: u32,
}

#[derive(Args, Debug)]
struct ShowTask {
    /// Optional ID of the task to show. If not provided, all tasks will be shown.
    id: Option<u32>,
}

#[derive(Args, Debug)]
#[command(group(ArgGroup::new("target").required(true).args(["id", "text"])))]
struct Search {
    #[arg(short, long)]
    id: Option<u32>,

    #[arg(long)]
    text: Option<String>,

    #[arg(long, requires = "text")]
    name: bool,

    #[arg(long, requires = "text")]
    description: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    name: String,
    done: bool,
    description: Option<String>,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mark = if self.done { "x" } else { " " };
        write!(f, "[{}] #{} {}", mark, self.id, self.name)?;

        if let Some(desc) = &self.description {
            write!(f, "\n    {}", desc)?;
        }

        Ok(())
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut todos = file::load()?;
    let mut need_saving = true;

    match cli.command {
        Commands::Add(add_task) => {
            let task = Task {
                id: help::next_id(&todos),
                name: add_task.task,
                done: add_task.done,
                description: add_task.description,
            };

            todos.push(task.clone());
        }

        Commands::Mark(mark) => {
            if let Some(task) = todos.iter().find(|t| t.id == mark.id) {
                let mut updated_task = task.clone();
                updated_task.done = !mark.undone;

                if let Some(pos) = todos.iter().position(|t| t.id == mark.id) {
                    todos[pos] = updated_task;
                }
            } else {
                println!("Task with ID {} not found.", mark.id);
            }
        }

        Commands::Remove(remove_task) => {
            if let Some(_task) = todos.iter().find(|t| t.id == remove_task.id) {
                todos.retain(|t| t.id != remove_task.id);
            } else {
                println!("Task with ID {} not found.", remove_task.id);
            }
        }

        Commands::Show(show_task) => {
            need_saving = false;

            if let Some(id) = show_task.id {
                if let Some(task) = todos.iter().find(|t| t.id == id) {
                    println!("{}", task);
                } else {
                    println!("No task with the ID: {}", id);
                }
            } else {
                help::print_tasks(&todos);
            }
        }

        Commands::Search(search) => {
            need_saving = false;

            let found = help::search(&todos, &search);

            if found.is_empty() {
                println!("Nothing found.");
            } else {
                for t in found {
                    println!("{t}");
                }
            }
        }
    }

    if need_saving {
        file::save(&todos)?;
    }

    Ok(())
}
