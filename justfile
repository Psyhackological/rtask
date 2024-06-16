alias i := db_init
alias ls := list
alias a := add
alias d := delete-done

alias c := clippy
alias f := fmt

export DATABASE_URL := "sqlite:todos.db"

db_init:
  sqlx db create
  sqlx migrate run

list:
  cargo run

add task category="":
  if [ "{{category}}" = "" ]; then \
    cargo run -- add "{{task}}"; \
  else \
    cargo run -- add "{{task}}" "{{category}}"; \
  fi

done id:
  cargo run -- done {{id}}

delete-done:
  cargo run -- delete-done

clippy:
  cargo clippy

fmt:
  cargo fmt
