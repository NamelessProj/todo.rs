use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fmt, io::Result};

mod command;
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
    Add(command::Add),
    Edit(command::Edit),
    Mark(command::Mark),
    Remove(command::Remove),
    Show(command::Show),
    Search(command::Search),
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
        Commands::Add(add) => {
            let task = Task {
                id: help::next_id(&todos),
                name: add.name,
                done: !add.status.is_none_or(|s| matches!(s, command::Status::Undone)),
                description: add.description,
            };

            todos.push(task.clone());
        }

        Commands::Edit(edit) => {
            if let Some(task) = todos.iter_mut().find(|t| t.id == edit.id) {
                if let Some(name) = edit.name {
                    task.name = name;
                }

                if let Some(desc) = edit.description {
                    task.description = Some(desc);
                }

                if let Some(status) = edit.status {
                    task.done = matches!(status, command::Status::Done);
                }
            } else {
                println!("Task with ID {} not found.", edit.id);
            }
        }

        Commands::Mark(mark) => {
            if let Some(task) = todos.iter_mut().find(|t| t.id == mark.id) {
                task.done = mark.status.is_none_or(|s| matches!(s, command::Status::Done));
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
