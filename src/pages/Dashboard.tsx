import React, { useState } from 'react';
import { Wallet, BarChart3, TrendingUp, DollarSign, Bot, Shield, Zap, Clock, MessageSquare } from 'lucide-react';
import TopPoolsTable from '../components/TopPoolsTable';
import AgentChat from '../components/AgentChat';
import BlockchainStatus from '../components/BlockchainStatus';
import { pools } from '../data/pools';

const Dashboard: React.FC = () => {
  const [isChatOpen, setIsChatOpen] = useState(false);
  
  // Sort pools by APY in descending order and take top 5
  const topPools = [...pools].sort((a, b) => b.apy - a.apy).slice(0, 5);

  return (
    <div className="container mx-auto px-4 py-8">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-white mb-2">AgentX Dashboard</h1>
        <p className="text-gray-400">
          Discover the best yield opportunities across the MultiversX ecosystem
        </p>
      </div>
      
      {/* Blockchain Status */}
      <div className="mb-6">
        <BlockchainStatus />
      </div>
      
      {/* Agent Explanation Section */}
      <div className="bg-gray-800 rounded-xl shadow-md p-6 mb-8">
        <h2 className="text-xl font-semibold text-white mb-4">How Your AgentX Works</h2>
        <p className="text-gray-400 mb-6">
          Our automated agents help you maximize your yield while maintaining your risk preferences. Here's how it works:
        </p>
        
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
          <div className="bg-gray-700 p-5 rounded-lg flex flex-col items-center text-center">
            <div className="bg-blue-900 p-3 rounded-full mb-4">
              <Bot className="h-6 w-6 text-blue-400" />
            </div>
            <h3 className="font-semibold text-white mb-2">Dedicated Agent</h3>
            <p className="text-sm text-gray-400">
              Upon deposit, you get your own agent operating through a dedicated wallet on the MultiversX blockchain.
            </p>
          </div>
          
          <div className="bg-gray-700 p-5 rounded-lg flex flex-col items-center text-center">
            <div className="bg-green-900 p-3 rounded-full mb-4">
              <Shield className="h-6 w-6 text-green-400" />
            </div>
            <h3 className="font-semibold text-white mb-2">Managed Supplies</h3>
            <p className="text-sm text-gray-400">
              Your agent manages supplies in MultiversX vaults following strict policies and your set preferences.
            </p>
          </div>
          
          <div className="bg-gray-700 p-5 rounded-lg flex flex-col items-center text-center">
            <div className="bg-purple-900 p-3 rounded-full mb-4">
              <Zap className="h-6 w-6 text-purple-400" />
            </div>
            <h3 className="font-semibold text-white mb-2">Automatic Optimization</h3>
            <p className="text-sm text-gray-400">
              The agent automatically seeks the best yields (APY) across vaults, rebalancing to maximize your returns.
            </p>
          </div>
          
          <div className="bg-gray-700 p-5 rounded-lg flex flex-col items-center text-center">
            <div className="bg-orange-900 p-3 rounded-full mb-4">
              <Clock className="h-6 w-6 text-orange-400" />
            </div>
            <h3 className="font-semibold text-white mb-2">Customizable Strategy</h3>
            <p className="text-sm text-gray-400">
              Customize your strategy through our chat interface and set your preferred rebalancing schedule.
            </p>
          </div>
        </div>
        
        <div className="mt-6 pt-6 border-t border-gray-700 text-center">
          <button 
            className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors flex items-center justify-center mx-auto"
            onClick={() => setIsChatOpen(true)}
          >
            <MessageSquare className="h-5 w-5 mr-2" />
            Chat with Your Agent
          </button>
        </div>
      </div>
      
      <div className="mb-8">
        <TopPoolsTable pools={topPools} />
      </div>
      
      <div className="bg-gray-800 rounded-xl shadow-md p-6 mb-8">
        <h2 className="text-xl font-semibold text-white mb-4">MultiversX Ecosystem Overview</h2>
        <p className="text-gray-400 mb-4">
          MultiversX (formerly Elrond) is a highly scalable, fast, and secure blockchain platform for distributed apps, enterprise use cases, and the new internet economy. The MultiversX DeFi ecosystem is growing rapidly with various protocols offering yield opportunities.
        </p>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="bg-gray-700 p-4 rounded-lg">
            <h3 className="font-semibold text-blue-400 mb-2">High Performance</h3>
            <p className="text-sm text-gray-300">
              MultiversX's Adaptive State Sharding enables parallel processing, resulting in 15,000+ TPS and 6-second latency.
            </p>
          </div>
          <div className="bg-gray-700 p-4 rounded-lg">
            <h3 className="font-semibold text-green-400 mb-2">Secure Proof of Stake</h3>
            <p className="text-sm text-gray-300">
              The Secure Proof of Stake consensus mechanism provides security while being environmentally friendly.
            </p>
          </div>
          <div className="bg-gray-700 p-4 rounded-lg">
            <h3 className="font-semibold text-purple-400 mb-2">Developer Friendly</h3>
            <p className="text-sm text-gray-300">
              MultiversX offers tools and frameworks that make it easy for developers to build and deploy dApps.
            </p>
          </div>
        </div>
      </div>
      
      <div className="bg-blue-600 text-white rounded-xl shadow-md p-8 text-center">
        <h2 className="text-2xl font-bold mb-4">Ready to start earning?</h2>
        <p className="mb-6 max-w-2xl mx-auto">
          Connect your MultiversX wallet to start investing in the highest yielding opportunities across the ecosystem.
        </p>
        <button className="bg-white text-blue-600 hover:bg-blue-50 px-6 py-3 rounded-lg font-medium transition-colors">
          Connect Wallet
        </button>
      </div>
      
      {/* Agent Chat Component */}
      <AgentChat isOpen={isChatOpen} onClose={() => setIsChatOpen(false)} />
      
      {/* Chat Button (Fixed) */}
      {!isChatOpen && (
        <button
          className="fixed bottom-6 right-6 bg-blue-600 text-white p-4 rounded-full shadow-lg hover:bg-blue-700 transition-colors z-50"
          onClick={() => setIsChatOpen(true)}
        >
          <MessageSquare className="h-6 w-6" />
        </button>
      )}
    </div>
  );
};

export default Dashboard;