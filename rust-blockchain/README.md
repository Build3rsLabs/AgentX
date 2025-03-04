# AgentX Blockchain Integration Layer

This Rust-based blockchain integration layer provides the backend services for the AgentX yield optimization platform on MultiversX.

## Features

- **Wallet Management**: Secure wallet creation, import, and transaction signing
- **Protocol Integration**: Direct integration with MultiversX DeFi protocols
- **Position Management**: Create, monitor, and optimize yield positions
- **Transaction Handling**: Submit and track blockchain transactions
- **API Server**: RESTful API for frontend integration

## Architecture

The project follows a clean architecture pattern with the following components:

- **API**: HTTP endpoints for frontend communication
- **Blockchain**: MultiversX blockchain integration
- **Services**: Business logic for positions, protocols, transactions
- **Models**: Data structures and type definitions
- **Database**: PostgreSQL storage for user data and positions
- **Error Handling**: Comprehensive error types and handling

## Getting Started

### Prerequisites

- Rust 1.70+ and Cargo
- PostgreSQL 14+
- MultiversX SDK

### Installation

1. Clone the repository:
   ```
   git clone https://github.com/agentx-finance/agentx-blockchain.git
   cd agentx-blockchain
   ```

2. Set up environment variables (create a `.env` file):
   ```
   DATABASE_URL=postgres://postgres:postgres@localhost:5432/agentx_dev
   JWT_SECRET=your_jwt_secret_here
   BLOCKCHAIN_NETWORK=devnet
   GATEWAY_URL=https://devnet-gateway.multiversx.com
   CHAIN_ID=D
   ```

3. Build the project:
   ```
   cargo build --release
   ```

4. Run database migrations:
   ```
   cargo run -- --dev
   ```

### Usage

Start the API server:
```
cargo run -- serve --port 3030
```

Generate a new wallet:
```
cargo run -- generate-wallet
```

Check balance for an address:
```
cargo run -- balance erd1...
```

## API Endpoints

### Authentication
- `POST /api/auth/register` - Register a new user with wallet address
- `POST /api/auth/login` - Authenticate with wallet signature

### Protocols
- `GET /api/protocols` - List all supported protocols
- `GET /api/protocols/{id}` - Get protocol details
- `GET /api/protocols/{id}/pools` - Get pools for a protocol
- `GET /api/pools` - List all pools across protocols
- `GET /api/pools/{id}` - Get pool details

### Positions
- `POST /api/positions` - Create a new position
- `GET /api/positions` - List user positions
- `GET /api/positions/{id}` - Get position details
- `PUT /api/positions/{id}` - Update position
- `DELETE /api/positions/{id}` - Delete position
- `POST /api/positions/{id}/rebalance` - Rebalance position

### Transactions
- `POST /api/transactions` - Create a new transaction
- `GET /api/transactions` - List user transactions
- `GET /api/transactions/{id}` - Get transaction details
- `GET /api/transactions/hash/{hash}` - Get transaction by hash

## Development

### Project Structure

```
rust-blockchain/
├── src/
│   ├── api/            # API endpoints
│   ├── blockchain/     # MultiversX integration
│   ├── config/         # Configuration handling
│   ├── db/             # Database connection
│   ├── error/          # Error types
│   ├── models/         # Data models
│   ├── services/       # Business logic
│   ├── utils/          # Utility functions
│   ├── wallet/         # Wallet management
│   └── main.rs         # Application entry point
├── migrations/         # Database migrations
├── Cargo.toml          # Project dependencies
└── .env                # Environment variables
```

### Running Tests

```
cargo test
```

### Documentation

Generate and view the documentation:
```
cargo doc --open
```

## Integration with Frontend

The Rust backend exposes a RESTful API that the React frontend communicates with. The frontend uses the MultiversX SDK for wallet connection and transaction signing, while the backend handles the complex blockchain interactions and position management.

## License

This project is licensed under the MIT License - see the LICENSE file for details.