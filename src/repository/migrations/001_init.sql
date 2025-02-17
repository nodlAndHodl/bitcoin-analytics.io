CREATE TABLE IF NOT EXISTS address_transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address_id VARCHAR(191) NOT NULL,
    transaction_id VARCHAR(191) NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_address_id ON address_transactions (address_id);
CREATE INDEX IF NOT EXISTS idx_transaction_id ON address_transactions (transaction_id);

CREATE TABLE IF NOT EXISTS addresses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    address LONGTEXT NOT NULL,
    total_amount_in DOUBLE DEFAULT NULL,
    total_amount_out DOUBLE DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS blocks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    hash LONGTEXT NOT NULL,
    height BIGINT NOT NULL,
    version BIGINT DEFAULT NULL,
    version_hex LONGTEXT,
    merkle_root LONGTEXT,
    time BIGINT DEFAULT NULL,
    mediantime BIGINT DEFAULT NULL,
    nonce BIGINT DEFAULT NULL,
    bits LONGTEXT,
    difficulty DOUBLE DEFAULT NULL,
    chainwork LONGTEXT,
    n_tx BIGINT DEFAULT NULL,
    previous_block_hash LONGTEXT,
    next_block_hash LONGTEXT,
    tx JSON DEFAULT NULL,
    stripped_size BIGINT DEFAULT NULL,
    size BIGINT DEFAULT NULL,
    weight BIGINT DEFAULT NULL
);

CREATE TABLE IF NOT EXISTS transactions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    txid LONGTEXT,
    hash LONGTEXT,
    version BIGINT DEFAULT NULL,
    size BIGINT DEFAULT NULL,
    vsize BIGINT DEFAULT NULL,
    weight BIGINT DEFAULT NULL,
    locktime BIGINT DEFAULT NULL,
    hex LONGTEXT,
    blockhash LONGTEXT,
    confirmations BIGINT DEFAULT NULL,
    time BIGINT DEFAULT NULL,
    block_id INTEGER DEFAULT NULL,
    FOREIGN KEY (block_id) REFERENCES blocks(id)
);

CREATE INDEX IF NOT EXISTS fk_transactions_block ON transactions (block_id);

CREATE TABLE IF NOT EXISTS vouts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id INTEGER DEFAULT NULL,
    txid VARCHAR(191) DEFAULT NULL,
    value DOUBLE DEFAULT NULL,
    n BIGINT DEFAULT NULL,
    script_pub_key_asm LONGTEXT,
    script_pub_key_hex LONGTEXT,
    script_pub_key_type LONGTEXT,
    script_pub_key_address LONGTEXT,
    FOREIGN KEY (transaction_id) REFERENCES transactions(id)
);

CREATE INDEX IF NOT EXISTS idx_vouts_txid ON vouts (txid);

CREATE TABLE IF NOT EXISTS vins (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id INTEGER DEFAULT NULL,
    txid VARCHAR(191) DEFAULT NULL,
    vout BIGINT DEFAULT NULL,
    script_sig_asm LONGTEXT,
    script_sig_hex LONGTEXT,
    sequence BIGINT DEFAULT NULL,
    coinbase LONGTEXT,
    FOREIGN KEY (transaction_id) REFERENCES transactions(id)
);

CREATE INDEX IF NOT EXISTS idx_vins_txid ON vins (txid);