use rust_task::{add_todo, complete_todo, delete_done_todos, list_todos};
use rust_task::{Args, Command};
use sqlx::sqlite::SqlitePool;
use std::env;
use structopt::StructOpt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let args = Args::from_args_safe()?;
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

    match args.cmd {
        Some(Command::Add { description }) => {
            println!("Adding new todo with description '{description}'");
            let todo_id = add_todo(&pool, description).await?;
            println!("Added new todo with id {todo_id}");
        }
        Some(Command::Done { id }) => {
            println!("Marking todo {id} as done");
            match complete_todo(&pool, id).await? {
                Some(todo) => println!("Todo marked as done: {}", todo),
                None => println!("Invalid id {id}"),
            }
        }
        Some(Command::DeleteDone) => {
            println!("Deleting all done todos");
            let deleted_count = delete_done_todos(&pool).await?;
            println!("Deleted {deleted_count} todos that were marked as done");
        }
        None => {
            println!("Printing list of all todos");
            list_todos(&pool).await?;
        }
    }

    Ok(())
}
