mod api;
mod blockchain;
mod config;
mod db;
mod elizaos;
mod error;
mod models;
mod services;
mod smart_contracts;
mod utils;
mod wallet;

use clap::{Parser, Subcommand};
use config::AppConfig;
use elizaos::{ElizaOS, ElizaOSConfig};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Enable development mode
    #[arg(long)]
    dev: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the API server
    Serve {
        /// Port to listen on
        #[arg(short, long, default_value_t = 3030)]
        port: u16,
    },
    /// Generate a new wallet
    GenerateWallet,
    /// Get balance for an address
    Balance {
        /// The address to check
        address: String,
    },
    /// Find best yield opportunities
    FindYield {
        /// Strategy (conservative, balanced, aggressive)
        #[arg(short, long, default_value = "balanced")]
        strategy: String,
        
        /// Token filter (optional)
        #[arg(short, long)]
        token: Option<String>,
        
        /// Number of results to return
        #[arg(short, long, default_value_t = 5)]
        limit: usize,
    },
    /// Get ElizaOS system status
    Status,
    /// Optimize a portfolio allocation
    Optimize {
        /// Strategy (conservative, balanced, aggressive)
        #[arg(short, long, default_value = "balanced")]
        strategy: String,
        
        /// Investment amount in EGLD
        #[arg(short, long, default_value_t = 100.0)]
        amount: f64,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting AgentX Blockchain Integration Layer");

    // Parse command line arguments
    let cli = Cli::parse();

    // Load configuration
    let config = if cli.dev {
        info!("Running in development mode");
        AppConfig::development()
    } else {
        AppConfig::production()
    };
    let config = Arc::new(config);

    // Initialize database connection
    let db_pool = db::init_db(&config).await?;
    info!("Database connection established");

    // Initialize blockchain client
    let blockchain_client = blockchain::MultiversXClient::new(&config);
    info!("Blockchain client initialized");
    
    // Initialize ElizaOS
    let elizaos_config = ElizaOSConfig {
        network: config.blockchain.network.clone(),
        max_gas_price: config.blockchain.min_gas_price,
        default_gas_limit: config.blockchain.min_gas_limit,
        rebalance_threshold: 0.05,
        max_concurrent_txs: 10,
        tx_timeout_seconds: 300,
    };
    
    let elizaos = ElizaOS::new(blockchain_client.clone(), Some(elizaos_config));
    elizaos.initialize().await?;
    info!("ElizaOS initialized successfully");

    // Create service container
    let services = services::ServiceContainer::new(db_pool, blockchain_client, config.clone());
    let services = Arc::new(services);

    match cli.command {
        Some(Commands::Serve { port }) => {
            info!("Starting API server on port {}", port);
            api::start_server(services, port).await?;
        }
        Some(Commands::GenerateWallet) => {
            let wallet = wallet::Wallet::generate()?;
            println!("Generated new wallet:");
            println!("Address: {}", wallet.address());
            println!("Mnemonic: {}", wallet.mnemonic());
            println!("Private Key: {}", hex::encode(wallet.private_key()));
        }
        Some(Commands::Balance { address }) => {
            let balance = blockchain::get_account_balance(&services.blockchain_client, &address).await?;
            println!("Balance for {}: {} EGLD", address, balance);
        }
        Some(Commands::FindYield { strategy, token, limit }) => {
            // Parse strategy
            let strategy_enum = match strategy.to_lowercase().as_str() {
                "conservative" => models::position::PositionStrategy::Conservative,
                "aggressive" => models::position::PositionStrategy::Aggressive,
                _ => models::position::PositionStrategy::Balanced,
            };
            
            // Find yield opportunities
            let opportunities = elizaos.find_best_opportunities(&strategy_enum, token, limit).await?;
            
            println!("Top {} yield opportunities for {} strategy:", 
                     opportunities.len(), strategy);
            println!("{:<20} {:<20} {:<10} {:<10} {:<10}", 
                     "Protocol", "Pool", "APY", "TVL", "Risk");
            println!("{}", "-".repeat(70));
            
            for opp in opportunities {
                println!("{:<20} {:<20} {:<10.2}% ${:<10.2}M {:<10}", 
                         opp.protocol_name, 
                         opp.pool_name, 
                         opp.apy, 
                         opp.tvl / 1_000_000.0,
                         opp.risk);
            }
        }
        Some(Commands::Status) => {
            // Get ElizaOS system status
            let status = elizaos.get_system_status().await?;
            
            println!("ElizaOS System Status");
            println!("---------------------");
            println!("Network: {} (Round: {}, Epoch: {})", 
                     status.network, status.current_round, status.current_epoch);
            println!("Transactions: {} pending, {} completed, {} failed",
                     status.pending_transactions, status.completed_transactions, status.failed_transactions);
            println!("\nProtocol Status:");
            println!("{:<20} {:<10} {:<10} {:<10}", 
                     "Protocol", "TVL ($M)", "APY", "Status");
            println!("{}", "-".repeat(50));
            
            for protocol in status.protocols {
                println!("{:<20} ${:<10.2} {:<10.2}% {:<10}", 
                         protocol.name, 
                         protocol.tvl / 1_000_000.0,
                         protocol.apy,
                         protocol.status);
            }
        }
        Some(Commands::Optimize { strategy, amount }) => {
            // Parse strategy
            let strategy_enum = match strategy.to_lowercase().as_str() {
                "conservative" => models::position::PositionStrategy::Conservative,
                "aggressive" => models::position::PositionStrategy::Aggressive,
                _ => models::position::PositionStrategy::Balanced,
            };
            
            // Calculate optimal allocation
            let allocation = elizaos.calculate_optimal_allocation(&strategy_enum, amount).await?;
            
            // Calculate expected APY
            let expected_apy = elizaos.get_yield_optimizer().calculate_expected_apy(&allocation).await?;
            
            println!("Optimal portfolio allocation for {} EGLD with {} strategy:", 
                     amount, strategy);
            println!("Expected APY: {:.2}%", expected_apy);
            println!("\n{:<30} {:<15} {:<15}", 
                     "Protocol:Pool", "Amount (EGLD)", "Allocation (%)");
            println!("{}", "-".repeat(60));
            
            let total: f64 = allocation.values().sum();
            
            for (key, value) in allocation {
                let percentage = (value / total) * 100.0;
                println!("{:<30} {:<15.2} {:<15.2}%", 
                         key, value, percentage);
            }
        }
        None => {
            // Default to serving the API
            info!("Starting API server on default port 3030");
            api::start_server(services, 3030).await?;
        }
    }

    Ok(())
}