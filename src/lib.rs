use sqlx::sqlite::SqlitePool;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(subcommand)]
    pub cmd: Option<Command>,
}

#[derive(StructOpt)]
pub enum Command {
    Add { description: String },
    Done { id: i64 },
    DeleteDone,
}

/// ## Create
///
/// Creates a new "todo" entry in the database.
///
/// ```sql
/// INSERT INTO todos ( description )
/// VALUES ( ?1 )
/// ```
pub async fn add_todo(pool: &SqlitePool, description: String) -> anyhow::Result<i64> {
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
pub async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<bool> {
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
pub async fn list_todos(pool: &SqlitePool) -> anyhow::Result<()> {
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
/// Deletes all tasks marked as "done". ```sql
/// DELETE FROM todos
/// WHERE done = TRUE
/// ```
pub async fn delete_done_todos(pool: &SqlitePool) -> anyhow::Result<i64> {
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
