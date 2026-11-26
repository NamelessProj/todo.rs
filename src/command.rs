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

    #[arg(long)]
    pub name: Option<String>,

    #[arg(long)]
    pub description: Option<String>,

    #[arg(long, value_enum)]
    pub status: Option<Status>,
}

#[derive(Args, Debug)]
pub struct Mark {
    pub id: u32,

    /// Mark the task as done (by default if neither "done" or "undone" is set)
    #[arg(long, conflicts_with = "undone")]
    pub done: bool,

    /// Mark the task as undone
    #[arg(long)]
    pub undone: bool,
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

    #[arg(long)]
    pub text: Option<String>,

    #[arg(long, requires = "text")]
    pub name: bool,

    #[arg(long, requires = "text")]
    pub description: bool,
}