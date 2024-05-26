use sqlx::sqlite::SqlitePool;
use std::env;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Add { description: String },
    Done { id: i64 },
    DeleteDone,
}

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
            if complete_todo(&pool, id).await? {
                println!("Todo {id} is marked as done");
            } else {
                println!("Invalid id {id}");
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

/// ## Create
///
/// Creates a new "todo" entry in the database.
///
/// ```sql
/// INSERT INTO todos ( description )
/// VALUES ( ?1 )
/// ```
async fn add_todo(pool: &SqlitePool, description: String) -> anyhow::Result<i64> {
    let mut conn = pool.acquire().await?;

    // Insert the task, then obtain the ID of this row
    let id = sqlx::query!(
        r#"
INSERT INTO todos ( description )
VALUES ( ?1 )
        "#,
        description
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    Ok(id)
}

/// ## Update
///
/// Marks a task as "done".
///
/// ```sql
/// UPDATE todos
/// SET done = TRUE
/// WHERE id = ?1
/// ```
async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
    let rows_affected = sqlx::query!(
        r#"
UPDATE todos
SET done = TRUE
WHERE id = ?1
        "#,
        id
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

/// ## Read
///
/// Prints the list of all "todo" tasks.
///
/// ```sql
/// SELECT id, description, done
/// FROM todos
/// ORDER BY id
/// ```
async fn list_todos(pool: &SqlitePool) -> anyhow::Result<()> {
    let recs = sqlx::query!(
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for rec in recs {
        println!(
            "- [{}] {}: {}",
            if rec.done { "x" } else { " " },
            rec.id,
            &rec.description,
        );
    }

    Ok(())
}

/// ## Delete
///
/// Deletes all tasks marked as "done".
///
/// ```sql
/// DELETE FROM todos
/// WHERE done = TRUE
/// ```
async fn delete_done_todos(pool: &SqlitePool) -> anyhow::Result<i64> {
    let rows_affected = sqlx::query!(
        r#"
DELETE FROM todos
WHERE done = TRUE
        "#
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(rows_affected as i64)
}
