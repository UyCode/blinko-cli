use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum Operation {
    Create,
    Update,
    Delete,
    SetToken,
    Login,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Operation to perform (create, update, delete, set-token, login)
    #[arg(short = 'o', long, default_value = "create")]
    pub operation: Operation,

    /// Note ID (required for update and delete operations)
    #[arg(short, long)]
    pub id: Option<String>,

    /// Token (only used with set-token operation)
    #[arg(short, long)]
    pub token: Option<String>,
} 