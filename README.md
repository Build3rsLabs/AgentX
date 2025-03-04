# AgentX - AI Yield Optimization Protocol

## TL,DR

AgentX is an AI yield optimization platform for the MultiversX ecosystem. It was built by us to help users maximize their DeFi returns while managing risk with automated strategies and chat recommendations.

## 🚀 Features

### AI Yield Assistant
- Natural language interface for personalized recommendations
- Context conversations about yield opportunities
- Risk-based portfolio suggestions
- Educational content on DeFi concepts

### Protocol Explorer
- Compare TVL, APY and risk levels across protocols
- Filter by risk preference or token type
- Detailed protocol information and analytics
- Direct access to protocol websites

### Intelligent Position Management
- Create and manage yield positions across protocols
- Automated rebalancing based on risk preferences
- Performance tracking and analytics
- Deposit and withdrawal management

### Strategy Selection
- Choose from Conservative, Balanced, or Aggressive strategies
- Customizable rebalancing frequency
- Risk appropriate yield targets
- Strategy performance monitoring

### Live Analytics
- Portfolio performance tracking
- Historical yield comparisons
- Risk assessment metrics
- Transaction history

## 🛠️ Technology Stack

### 1️⃣ Smart Contracts 

Language: Rust

Framework: MultiversX Smart Contract Rust framework

SDKs: multiversx-sc (for smart contracts)

Build Tool: multiversx-sc-meta (for contract metadata)

Testing: Rust unit tests + MultiversX CLI

### 2️⃣ Backend

Languages: TypeScript (Node.js), Go, Python

SDKs: MultiversX SDK (sdk-core, sdk-network-providers)

### 3️⃣ Frontend

Framework: React + TypeScript

State Management: React Context API

Styling: Tailwind CSS

Routing: React Router

Blockchain Integration: sdk-core, sdk-network-providers (for fetching smart contract data)

Data Visualization: Recharts / Chart.js (for analytics dashboards)

Icons: Lucide React

Date Handling: date-fns (for formatting timestamps from blockchain transactions)

## 📊 Supported Protocols

AgentX currently supports the following MultiversX protocols:

- Maiar Exchange (DEX)
- Hatom Protocol (Lending)
- AshSwap (Stable Swap AMM)
- xExchange (DEX)
- OneDex (Aggregator DEX)
- JEXchange (DEX)

## 🔮 Future Roadmap

### Phase 1: Enhanced AI Capabilities (Q3 2025)
- Advanced portfolio optimization algorithms
- Predictive analytics for yield forecasting
- Sentiment analysis of MultiversX ecosystem

### Phase 2: Expanded Protocol Support (Q4 2025)
- Integration with all MultiversX DeFi protocols
- Cross-protocol yield strategies
- Custom strategy builder

### Phase 3: Advanced Features (Q1 2026)
- Leveraged yield strategies for experienced users
- Institutional-grade risk management tools
- DAO governance for community-driven development

### Phase 4: Ecosystem Expansion (Q2 2026)
- Multi-chain support (starting with Ethereum and Solana)
- Fiat on/off ramps for seamless entry
- Mobile application with push notifications

## 💼 Business Model

AgentX employs a multi-tiered business model:

- **Free Tier**: Basic features with limited positions
- **Premium Subscription** ($10/month): Unlimited positions and advanced features
- **Performance Fee** (5% of yields above benchmark): For managed strategies
- **Enterprise Solutions**: For institutional clients

## 🔒 Security

Security is our top priority:

- Non-custodial architecture (users maintain control of funds)
- Smart contract audits by leading security firms
- Comprehensive risk management framework
- Regular security assessments

## 🚀 Getting Started

### Prerequisites

- Node.js 18+
- npm or yarn
- MultiversX wallet (xPortal, Ledger, or Web Wallet)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/agentx.git

# Navigate to the project directory
cd agentx

# Install dependencies
npm install

# Start the development server
npm run dev
```

### Building for Production

```bash
# Build the application
npm run build

# Preview the production build
npm run preview
```

## 📝 Demo Script

For a comprehensive demonstration of AgentX, follow our [demo script](./demo-script.md) which covers:

1. Dashboard Introduction
2. AI Assistant Demonstration
3. Earn Interface
4. Protocol Explorer
5. Pool Explorer
6. Asset Overview

## 📚 Documentation

- [Technical Architecture](./technical-architecture.md)
- [Business Model](./business-model.md)
- [Future Roadmap](./future-roadmap.md)
- [Demo Walkthrough Guide](./demo-walkthrough.md)

## 🔧 Blockchain Integration with ElizaOS

AgentX leverages ElizaOS, a specialized Rust-based operating system designed for secure blockchain operations on MultiversX. This integration provides several key advantages:

### ElizaOS Features

- **Transaction Execution**: ElizaOS provides a sandboxed environment for transaction creation and signing, ensuring user funds remain secure.
- **Protocol Adapters**: Custom-built adapters for each supported DeFi protocol enable optimized interactions with smart contracts.
- **Yield Optimization Engine**: Advanced algorithms continuously monitor and calculate optimal yield strategies across the MultiversX ecosystem.
- **Gas Optimization**: Intelligent gas price management ensures transactions are processed efficiently without overpaying fees.
- **Rebalancing Logic**: Sophisticated rebalancing algorithms determine the optimal time to rebalance positions based on gas costs, impermanent loss risk, and potential yield improvements.

### Architecture

The ElizaOS integration consists of several key components:

1. **Protocol Registry**: A central registry of all supported protocols with standardized interfaces for interaction.
2. **Smart Contract Clients**: Protocol-specific clients that handle the complexities of each protocol's smart contract interactions.
3. **Transaction Manager**: Manages the lifecycle of blockchain transactions, including creation, signing, submission, and confirmation.
4. **Position Manager**: Tracks and manages user positions across different protocols.
5. **Yield Optimizer**: Continuously analyzes yield opportunities and recommends optimal allocations.

### Security Measures

- **Non Custodial Design**: ElizaOS never takes custody of user funds; all operations are performed through user-authorized transactions.
- **Enclave Integration**: For hardware wallet users, ElizaOS integrates with secure enclaves for transaction signing.
- **Audit Trail**: Comprehensive logging of all operations for transparency and accountability.
- **Rate Limiting**: Protection against potential attack vectors through intelligent rate limiting.

### Performance

ElizaOS's Rust implementation provides exceptional performance benefits:

- **Low Latency**: Response times under 100ms for most operations
- **High Throughput**: Capable of handling thousands of positions simultaneously
- **Memory Efficiency**: Minimal memory footprint due to Rust's ownership model
- **Cross-Platform**: Works seamlessly across Linux, macOS, and Windows environments

## 🤝 Contributing

We welcome contributions to AgentX! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 📞 Contact

For questions or support, please reach out to us at:

- Email: support@agentx.finance
- Twitter: [@AgentXFinance](https://twitter.com/AgentXFinance)
- Discord: [AgentX Community](https://discord.gg/agentx)

---

Built with ❤️ for the MultiversX ecosystem
