use aptos_indexer_processor_sdk::utils::convert::{standardize_address};
use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};
use crate::schema::pools;

#[derive(AsChangeset, Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = pools)]
/// Database representation of a message
pub struct Pool {
    pub user: String,
    pub pool_addr: String,
    pub token_0: String,
    pub token_1: String,
    pub fee_bps: i64,
    pub bin_step_bps: i64,
    pub active_id: i64,
    pub protocol_fee_bps: i64,
    pub ts: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// On-chain representation of a pool creation event
pub struct PoolCreatedEventOnChain {
    pub user: String,
    pub pool_addr: String,
    pub token_0: String,
    pub token_1: String,
    pub fee_bps: String,
    pub bin_step_bps: String,
    pub active_id: String,
    pub protocol_fee_bps: String,
    pub ts: String,
}

impl PoolCreatedEventOnChain {
    pub fn to_db_message(&self) -> Pool {
        Pool {
            user: standardize_address(self.user.as_str()), 
            pool_addr: standardize_address(&self.pool_addr), 
            token_0: standardize_address(&self.token_0), 
            token_1: standardize_address(&self.token_1), 
            fee_bps: self.fee_bps.parse().unwrap_or(0), 
            bin_step_bps: self.bin_step_bps.parse().unwrap_or(0), 
            active_id: self.active_id.parse().unwrap_or(0), 
            protocol_fee_bps: self.protocol_fee_bps.parse().unwrap_or(0), 
            ts: self.ts.parse().unwrap_or(0)
        }
    }
}