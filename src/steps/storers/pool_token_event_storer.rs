use ahash::AHashMap;
use anyhow::Result;
use aptos_indexer_processor_sdk::utils::errors::ProcessorError;
use diesel::{insert_into, QueryResult};
use diesel_async::{AsyncConnection, AsyncPgConnection, RunQueryDsl};

use crate::{
    db_models::{pool_tokens::PoolToken},
    schema::{pool_tokens},
    utils::{
        database_connection::get_db_connection,
        database_utils::{get_config_table_chunk_size, ArcDbPool},
    },
};

async fn execute_create_pool_token_events_sql(
    conn: &mut AsyncPgConnection,
    items_to_insert: Vec<PoolToken>,
) -> QueryResult<()> {
    conn.transaction(|conn| {
        Box::pin(async move {
            let create_message_query = insert_into(pool_tokens::table)
                .values(items_to_insert.clone())
                .on_conflict(pool_tokens::address)
                .do_nothing();
            create_message_query.execute(conn).await?;
            Ok(())
        })
    })
    .await
}

pub async fn process_create_pool_token_events(
    pool: ArcDbPool,
    per_table_chunk_sizes: AHashMap<String, usize>,
    create_events: Vec<PoolToken>,
) -> Result<(), ProcessorError> {

    let chunk_size = get_config_table_chunk_size::<PoolToken>("pool_tokens", &per_table_chunk_sizes);
    let tasks = create_events
        .chunks(chunk_size)
        .map(|chunk| {
            let pool = pool.clone();
            let items = chunk.to_vec();
            // let user_stats_changes = user_stats_changes.clone();
            tokio::spawn(async move {
                let conn = &mut get_db_connection(&pool).await.expect(
                    "Failed to get connection from pool while processing pool tokens events",
                );
                execute_create_pool_token_events_sql(conn, items).await
            })
        })
        .collect::<Vec<_>>();

    let results = futures_util::future::try_join_all(tasks)
        .await
        .expect("Task panicked executing in chunks");
    for res in results {
        res.map_err(|e| {
            tracing::warn!("Error running query: {:?}", e);
            ProcessorError::ProcessError {
                message: e.to_string(),
            }
        })?;
    }
    Ok(())
}
