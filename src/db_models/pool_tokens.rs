use aptos_indexer_processor_sdk::utils::convert::{standardize_address};
use diesel::{AsChangeset, Insertable};
use field_count::FieldCount;
use serde::{Deserialize, Serialize};
use crate::schema::pool_tokens;

#[derive(AsChangeset, Clone, Debug, Deserialize, FieldCount, Insertable, Serialize)]
#[diesel(table_name = pool_tokens)]
/// Database representation of a message
pub struct PoolToken {
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
/// On-chain representation of a pool creation event
pub struct PoolTokenEventOnChain {
     pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: String,
}

impl PoolTokenEventOnChain {
    pub fn to_db_message(&self) -> PoolToken {
        PoolToken {
            address: standardize_address(self.address.as_str()), 
            name: self.name.clone(), 
            symbol: self.symbol.clone(), 
            decimals: self.decimals.parse().unwrap_or(0), 
        }
    }
}