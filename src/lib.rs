use sqlx::sqlite::SqlitePool;
use sqlx::FromRow;
use std::fmt;
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

#[derive(FromRow, Debug)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "- [{}] {}: {}",
            if self.done { "x" } else { " " },
            self.id,
            self.description
        )
    }
}

/// ## Create
///
/// Creates a new "todo" entry in the database.
///
/// ```sql
/// INSERT INTO todos ( description )
/// VALUES ( ?1 )
/// ```
pub async fn add_todo(pool: &SqlitePool, description: String) -> anyhow::Result<Todo> {
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

    let todo = Todo {
        id,
        description,
        done: false,
    };

    Ok(todo)
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
pub async fn complete_todo(pool: &SqlitePool, id: i64) -> anyhow::Result<Option<Todo>> {
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

    if rows_affected > 0 {
        let todo = sqlx::query_as!(
            Todo,
            r#"
SELECT id, description, done
FROM todos
WHERE id = ?1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        Ok(Some(todo))
    } else {
        Ok(None)
    }
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
pub async fn list_todos(pool: &SqlitePool) -> anyhow::Result<Vec<Todo>> {
    let recs = sqlx::query_as!(
        Todo,
        r#"
SELECT id, description, done
FROM todos
ORDER BY id
        "#
    )
    .fetch_all(pool)
    .await?;

    for todo in &recs {
        println!("{}", todo);
    }

    Ok(recs)
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
