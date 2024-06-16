# TODOs Example

## Setup

1. Declare the database URL

- GNU/Linux /w Bash

```bash
export DATABASE_URL="sqlite:todos.db"
```

- For Command Prompt (`cmd.exe`)

 ```cmd
 set DATABASE_URL=sqlite:todos.db
 ```

- PowerShell

 ```powershell
 $env:DATABASE_URL="sqlite:todos.db"
 ```
2. Create the database.

```sh
sqlx db create
```

3. Run SQL migrations

```sh
sqlx migrate run
```

## Usage

### Using Cargo Commands

Add a todo:

```sh
cargo run -- add "todo description"
```

Add a todo with a category:

```sh
cargo run -- add "todo description" "category name"
```

Complete a todo:

```sh
cargo run -- done <todo_id>
```

List all todos:

```sh
cargo run
```

Delete all completed todos:

```sh
cargo run -- delete-done
```

### Using Justfile Aliases

Initialize the database and run migrations:

```sh
just init
just i
```

Add a todo:

```sh
just add "todo description"
just a "todo description"
```

Add a todo with a category:

```sh
just add "todo description" "category name"
just a "todo description" "category name"
```

Complete a todo:

```sh
just done <todo_id>
just d <todo_id>
```

List all todos:

```sh
just list
just ls
```

List all todos filtered by category:

```sh
just list "category name"
just ls "category name"
```

Delete all completed todos:

```sh
just delete-done
just dd
```

Run Clippy to check for common mistakes:

```sh
just clippy
just c
```

Format the code using Rustfmt:

```sh
just format
just f
```

