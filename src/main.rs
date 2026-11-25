use clap::{Args, Parser, Subcommand};

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
}

#[derive(Args, Debug)]
struct AddTask {
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

struct Task {
    id: u32,
    name: String,
    is_done: bool,
    description: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(add_task) => {
            let task = Task {
                id: 0, // This would typically be generated
                name: add_task.task,
                is_done: add_task.is_done,
                description: add_task.description,
            };

            println!("Add command: {:?}", task.name);
            println!("Is done: {:?}", task.is_done);

            if let Some(desc) = task.description {
                println!("Description: {:?}", desc);
            } else {
                println!("No description provided.");
            }
        }
        Commands::Toggle(toggle_task) => {
            let task = Task {
                id: 0, // This would typically be generated
                name: "Sample Task".into(),
                is_done: toggle_task.is_done,
                description: None,
            };

            println!("Toggle command: {:?}", task.is_done);
        }
    }

}
