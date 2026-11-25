use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    task: String,

    #[arg(short, long, default_value_t = false)]
    is_done: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Toggle(ToggleTask),
}

#[derive(Args, Debug)]
struct ToggleTask {
    is_done: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Toggle(toggle) => {
            println!("Toggle command: {:?}", toggle.is_done);
        }
    }

    println!("args: {:?}", cli.task);
}
