-- Your SQL goes here
CREATE TABLE 
    pools (
        "user" VARCHAR(66) NOT NULL,
        pool_addr VARCHAR(66) PRIMARY KEY,
        token_0 VARCHAR(66) NOT NULL,
        token_1 VARCHAR(66) NOT NULL,
        fee_bps BIGINT NOT NULL,
        bin_step_bps BIGINT NOT NULL,
        active_id BIGINT NOT NULL,
        protocol_fee_bps BIGINT NOT NULL,
        ts BIGINT NOT NULL
    );