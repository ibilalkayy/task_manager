use clap::{Parser, Args, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub struct Task {
    #[clap(subcommand)]
    pub task_manager: TaskManager,
    pub file_name: String,
}

#[derive(Debug, Subcommand)]
pub enum TaskManager {
    /// Use this command to add a task
    Add(AddTask),

    /// Use this command to list all the tasks
    List(ListTask),

    /// Use this command to mark the task as complete
    Complete(CompleteTask),

    /// Use this command to remove the task
    Remove(RemoveTask),
}

#[derive(Debug, Args)]
pub struct AddTask {
    /// Give an id to the task
    #[arg(short, long)]
    pub id: u32,

    /// Give the task name to add
    #[arg(short, long)]
    pub name: String,

    /// Give the task description to add
    #[arg(short, long)]
    pub description: String,
}

#[derive(Debug, Args)]
pub struct ListTask {
    /// Give an id to the task
    #[arg(short, long)]
    pub id: u32,
}

#[derive(Debug, Args)]
pub struct CompleteTask {
    /// Give an id to the task
    #[arg(short, long)]
    pub id: u32,

    /// Give the status of the task
    #[arg(short, long)]
    pub status: String,
}

#[derive(Debug, Args)]
pub struct RemoveTask {
    /// Give an id to the task
    #[arg(short, long)]
    pub id: u32,
}