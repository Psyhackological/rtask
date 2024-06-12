alias i := db_init
alias ls := list
alias a := add
alias d := delete-done


export DATABASE_URL := "sqlite:todos.db"

db_init:
  sqlx db create
  sqlx migrate run

list:
  cargo run

add task:
  cargo run -- add {{task}}

done id:
  cargo run -- done {{id}}

delete-done:
  cargo run -- delete-done
