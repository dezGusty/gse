
> cargo install diesel_cli --no-default-features --features "sqlite-bundled"

> diesel setup

Generate the migration

> diesel migration generate create_players

fill in up & down.sql

> diesel migration run
