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
    Add {
        description: String,
        #[structopt(default_value = "")]
        category: String,
    },
    Done {
        id: i64,
    },
    DeleteDone,
    List {
        #[structopt(default_value = "")]
        category: String,
    },
}

#[derive(FromRow, Debug)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub done: bool,
    pub category_name: Option<String>,
}

#[derive(FromRow, Debug)]
pub struct Category {
    pub id: i64,
    pub name: String,
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "- [{}] {}: {}{}",
            if self.done { "x" } else { " " },
            self.id,
            self.description,
            if let Some(category_name) = &self.category_name {
                format!(" (category: {category_name})")
            } else {
                String::new()
            }
        )
    }
}

/// ## Create Category
///
/// Creates a new category in the database.
///
/// ```sql
/// INSERT INTO categories (name)
/// VALUES (?1)
/// ```
pub async fn add_category(pool: &SqlitePool, name: String) -> anyhow::Result<Category> {
    let mut conn = pool.acquire().await?;

    let id = sqlx::query!(
        r#"
INSERT INTO categories (name)
VALUES (?1)
        "#,
        name
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    let category = Category { id, name };

    Ok(category)
}

/// ## Get Category ID
///
/// Retrieves the category ID by name.
///
/// ```sql
/// SELECT id FROM categories WHERE name = ?1
/// ```
pub async fn get_category_id(pool: &SqlitePool, name: String) -> anyhow::Result<Option<i64>> {
    let result = sqlx::query!(
        r#"
SELECT id FROM categories
WHERE name = ?1
        "#,
        name
    )
    .fetch_optional(pool)
    .await?;

    Ok(result.map(|record| record.id))
}

/// ## Create Todo
///
/// Creates a new "todo" entry in the database.
///
/// ```sql
/// INSERT INTO todos (description, category_id)
/// VALUES (?1, ?2)
/// ```
pub async fn add_todo(
    pool: &SqlitePool,
    description: String,
    category: Option<String>,
) -> anyhow::Result<Todo> {
    let mut conn = pool.acquire().await?;

    let category_id = if let Some(cat) = category {
        match get_category_id(pool, cat.clone()).await? {
            Some(id) => Some(id),
            None => Some(add_category(pool, cat).await?.id),
        }
    } else {
        None
    };

    let id = sqlx::query!(
        r#"
INSERT INTO todos (description, category_id)
VALUES (?1, ?2)
        "#,
        description,
        category_id
    )
    .execute(&mut *conn)
    .await?
    .last_insert_rowid();

    let todo = sqlx::query_as!(
        Todo,
        r#"
SELECT todos.id, todos.description, todos.done, categories.name AS category_name
FROM todos
LEFT JOIN categories ON todos.category_id = categories.id
WHERE todos.id = ?1
        "#,
        id
    )
    .fetch_one(&mut *conn)
    .await?;

    Ok(todo)
}

/// ## Update Todo
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
SELECT todos.id, todos.description, todos.done, categories.name AS category_name
FROM todos
LEFT JOIN categories ON todos.category_id = categories.id
WHERE todos.id = ?1
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

/// ## Read Todos
///
/// Prints the list of all "todo" tasks, optionally filtered by category.
///
/// ```sql
/// SELECT todos.id, todos.description, todos.done, categories.name AS category_name
/// FROM todos
/// LEFT JOIN categories ON todos.category_id = categories.id
/// WHERE categories.name = ?1 OR ?1 = ''
/// ORDER BY todos.id
/// ```
pub async fn list_todos(pool: &SqlitePool, category: Option<String>) -> anyhow::Result<Vec<Todo>> {
    let recs = if let Some(cat) = category {
        sqlx::query_as!(
            Todo,
            r#"
SELECT todos.id, todos.description, todos.done, categories.name AS category_name
FROM todos
LEFT JOIN categories ON todos.category_id = categories.id
WHERE categories.name = ?1
ORDER BY todos.id
            "#,
            cat
        )
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as!(
            Todo,
            r#"
SELECT todos.id, todos.description, todos.done, categories.name AS category_name
FROM todos
LEFT JOIN categories ON todos.category_id = categories.id
ORDER BY todos.id
            "#
        )
        .fetch_all(pool)
        .await?
    };

    for todo in &recs {
        println!("{todo}");
    }

    Ok(recs)
}

/// ## Delete Done Todos
///
/// Deletes all tasks marked as "done".
///
/// ```sql
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
