use clap::{ArgGroup, Args, ValueEnum};

#[derive(Clone, Debug, ValueEnum)]
pub enum Status {
    Done,
    Undone,
}

#[derive(Args, Debug)]
pub struct Add {
    /// The name of the task to add
    pub task: String,

    #[arg(short, long)]
    pub description: Option<String>,

    /// Marked the task as done
    #[arg(long, default_value_t = false)]
    pub done: bool,
}

#[derive(Args, Debug)]
#[command(group(ArgGroup::new("changes").required(true).multiple(true).args(["name", "description", "status"])))]
pub struct Edit {
    pub id: u32,

    #[arg(short, long)]
    pub name: Option<String>,

    #[arg(short, long)]
    pub description: Option<String>,

    #[arg(short, long, value_enum)]
    pub status: Option<Status>,
}

#[derive(Args, Debug)]
pub struct Mark {
    pub id: u32,

    /// Set the status (by default "done")
    #[arg(short, long)]
    pub status: Option<Status>,
}

#[derive(Args, Debug)]
pub struct Remove {
    pub id: u32,
}

#[derive(Args, Debug)]
pub struct Show {
    /// Optional ID of the task to show. If not provided, all tasks will be shown.
    pub id: Option<u32>,
}

#[derive(Args, Debug)]
#[command(group(ArgGroup::new("target").required(true).args(["id", "text"])))]
pub struct Search {
    #[arg(short, long)]
    pub id: Option<u32>,

    #[arg(short, long)]
    pub text: Option<String>,

    #[arg(short, long, requires = "text")]
    pub name: bool,

    #[arg(short, long, requires = "text")]
    pub description: bool,
}