diesel migration run \
  --config-file="src/db_migrations/diesel.toml" \
  --database-url="postgresql://sapi-main_owner:npg_9riH5ChPQZyA@ep-fragrant-glade-a8t2nujw-pooler.eastus2.azure.neon.tech/sapi-dev?sslmode=require&channel_binding=require"

  cargo run --release -- -c config.yaml
