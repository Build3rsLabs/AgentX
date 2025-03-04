-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    address TEXT NOT NULL UNIQUE,
    nonce TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Create protocols table
CREATE TABLE IF NOT EXISTS protocols (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    logo_url TEXT NOT NULL,
    description TEXT NOT NULL,
    tvl DOUBLE PRECISION NOT NULL,
    apy DOUBLE PRECISION NOT NULL,
    risk TEXT NOT NULL,
    tokens TEXT[] NOT NULL,
    website_url TEXT NOT NULL,
    contract_address TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Create pools table
CREATE TABLE IF NOT EXISTS pools (
    id TEXT PRIMARY KEY,
    protocol_id TEXT NOT NULL REFERENCES protocols(id),
    name TEXT NOT NULL,
    tvl DOUBLE PRECISION NOT NULL,
    apy DOUBLE PRECISION NOT NULL,
    tokens TEXT[] NOT NULL,
    risk TEXT NOT NULL,
    contract_address TEXT,
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,
    is_active BOOLEAN NOT NULL DEFAULT TRUE
);

-- Create positions table
CREATE TABLE IF NOT EXISTS positions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    protocol_id TEXT NOT NULL REFERENCES protocols(id),
    name TEXT NOT NULL,
    position_type TEXT NOT NULL,
    tokens TEXT[] NOT NULL,
    deposited DOUBLE PRECISION NOT NULL,
    current_value DOUBLE PRECISION NOT NULL,
    apy DOUBLE PRECISION NOT NULL,
    strategy TEXT NOT NULL,
    entry_date TIMESTAMPTZ NOT NULL,
    last_rebalance TIMESTAMPTZ NOT NULL,
    rebalance_frequency TEXT NOT NULL,
    allocation JSONB NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'::JSONB,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Create transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    tx_hash TEXT NOT NULL UNIQUE,
    tx_type TEXT NOT NULL,
    amount TEXT,
    token TEXT,
    status TEXT NOT NULL,
    protocol_id TEXT REFERENCES protocols(id),
    position_id UUID REFERENCES positions(id),
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_users_address ON users(address);
CREATE INDEX IF NOT EXISTS idx_protocols_is_active ON protocols(is_active);
CREATE INDEX IF NOT EXISTS idx_pools_protocol_id ON pools(protocol_id);
CREATE INDEX IF NOT EXISTS idx_pools_is_active ON pools(is_active);
CREATE INDEX IF NOT EXISTS idx_positions_user_id ON positions(user_id);
CREATE INDEX IF NOT EXISTS idx_positions_protocol_id ON positions(protocol_id);
CREATE INDEX IF NOT EXISTS idx_transactions_user_id ON transactions(user_id);
CREATE INDEX IF NOT EXISTS idx_transactions_tx_hash ON transactions(tx_hash);
CREATE INDEX IF NOT EXISTS idx_transactions_position_id ON transactions(position_id);