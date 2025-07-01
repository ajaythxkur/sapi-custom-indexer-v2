-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE 
    pool_tokens (
        "address" VARCHAR(66)  PRIMARY KEY NOT NULL,
        name VARCHAR NOT NULL,
        symbol VARCHAR NOT NULL,
        decimals INT NOT NULL
    );