diesel migration run \
  --config-file="src/db_migrations/diesel.toml" \
  --database-url=""

  cargo run --release -- -c config.yaml
