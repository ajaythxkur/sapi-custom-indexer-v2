// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Int4,
        #[max_length = 255]
        address -> Varchar,
        points -> Nullable<Int4>,
        completed_quest_ids -> Nullable<Array<Nullable<Int4>>>,
        #[max_length = 255]
        referral_code -> Varchar,
        #[max_length = 255]
        referred_by -> Nullable<Varchar>,
        is_admin -> Nullable<Bool>,
        createdAt -> Timestamptz,
        updatedAt -> Timestamptz,
    }
}

diesel::table! {
    ledger_infos (chain_id) {
        chain_id -> Int8,
    }
}

diesel::table! {
    module_upgrade_history (module_addr, module_name, upgrade_number) {
        #[max_length = 300]
        module_addr -> Varchar,
        #[max_length = 300]
        module_name -> Varchar,
        upgrade_number -> Int8,
        module_bytecode -> Bytea,
        module_source_code -> Text,
        module_abi -> Json,
        tx_version -> Int8,
    }
}

diesel::table! {
    package_upgrade_history (package_addr, package_name, upgrade_number) {
        #[max_length = 300]
        package_addr -> Varchar,
        #[max_length = 300]
        package_name -> Varchar,
        upgrade_number -> Int8,
        upgrade_policy -> Int8,
        package_manifest -> Text,
        source_digest -> Text,
        tx_version -> Int8,
    }
}

diesel::table! {
    processor_status (processor) {
        #[max_length = 50]
        processor -> Varchar,
        last_success_version -> Int8,
        last_updated -> Timestamp,
        last_transaction_timestamp -> Nullable<Timestamp>,
    }
}

diesel::table! {
    pools (pool_addr) {
        #[max_length = 66]
        user -> Varchar,
        #[max_length = 66]
        pool_addr -> Varchar,
        #[max_length = 66]
        token_0 -> Varchar,
        #[max_length = 66]
        token_1 -> Varchar,
        fee_bps -> Int8,
        bin_step_bps -> Int8,
        active_id -> Int8,
        protocol_fee_bps -> Int8,
        ts -> Int8,
    }
}

diesel::table! {
    pool_tokens (address) {
        #[max_length = 66]
        address -> Varchar,
        name -> Varchar,
        symbol -> Varchar,
        decimals -> Int8,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    ledger_infos,
    module_upgrade_history,
    package_upgrade_history,
    processor_status,
    pools,
    pool_tokens,
);
