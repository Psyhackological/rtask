alias i := init
alias ls := list
alias a := add
alias d := done
alias dd := delete-done

alias c := clippy
alias f := fmt

export DATABASE_URL := "sqlite:todos.db"

init:
  sqlx db create
  sqlx migrate run

list category="":
  if [ "{{category}}" = "" ]; then \
    cargo run -- list ""; \
  else \
    cargo run -- list "{{category}}"; \
  fi

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
