import React, { useState } from 'react';
import { Shield, TrendingUp, BarChart3, AlertCircle } from 'lucide-react';
import { useAuth } from '../context/AuthContext';
import ConnectWalletModal from './ConnectWalletModal';

interface StrategyOption {
  id: string;
  name: string;
  description: string;
  risk: 'Low' | 'Medium' | 'High';
  expectedApy: string;
  icon: React.ReactNode;
}

const strategies: StrategyOption[] = [
  {
    id: 'conservative',
    name: 'Conservative',
    description: 'Focus on capital preservation with stable, lower yields. Primarily uses lending protocols and stablecoin pools.',
    risk: 'Low',
    expectedApy: '5-8%',
    icon: <Shield className="h-6 w-6 text-green-400" />
  },
  {
    id: 'balanced',
    name: 'Balanced',
    description: 'Moderate risk approach balancing growth and stability. Mix of lending, liquidity pools, and some farming.',
    risk: 'Medium',
    expectedApy: '10-15%',
    icon: <BarChart3 className="h-6 w-6 text-blue-400" />
  },
  {
    id: 'aggressive',
    name: 'Aggressive',
    description: 'Maximize yield through higher risk strategies. Heavy focus on farming, new protocols, and leveraged positions.',
    risk: 'High',
    expectedApy: '18-25%',
    icon: <TrendingUp className="h-6 w-6 text-red-400" />
  }
];

const StrategySelector: React.FC = () => {
  const { isAuthenticated } = useAuth();
  const [selectedStrategy, setSelectedStrategy] = useState<string>('balanced');
  const [isConnectModalOpen, setIsConnectModalOpen] = useState(false);
  const [isApplying, setIsApplying] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);
  
  const getRiskColor = (risk: string) => {
    switch (risk) {
      case 'Low':
        return 'text-green-400';
      case 'Medium':
        return 'text-yellow-400';
      case 'High':
        return 'text-red-400';
      default:
        return 'text-gray-400';
    }
  };

  const handleApplyStrategy = () => {
    if (!isAuthenticated) {
      setIsConnectModalOpen(true);
      return;
    }
    
    setIsApplying(true);
    
    // Simulate API call
    setTimeout(() => {
      setIsApplying(false);
      setIsSuccess(true);
      
      // Reset success message after 3 seconds
      setTimeout(() => {
        setIsSuccess(false);
      }, 3000);
    }, 1500);
  };

  return (
    <>
      <div className="bg-gray-800 rounded-lg p-6 mb-6">
        <h2 className="text-xl font-semibold text-white mb-4">Select Your Strategy</h2>
        <p className="text-gray-400 mb-6">
          Choose how your agent should manage your assets. This will determine the risk level and potential returns.
        </p>
        
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          {strategies.map(strategy => (
            <div 
              key={strategy.id}
              className={`border rounded-lg p-5 cursor-pointer transition-all ${
                selectedStrategy === strategy.id 
                  ? 'border-blue-500 bg-gray-700' 
                  : 'border-gray-700 bg-gray-800 hover:border-gray-600'
              }`}
              onClick={() => setSelectedStrategy(strategy.id)}
            >
              <div className="flex items-center mb-3">
                <div className="mr-3">
                  {strategy.icon}
                </div>
                <div>
                  <h3 className="text-white font-semibold">{strategy.name}</h3>
                  <p className={`text-sm ${getRiskColor(strategy.risk)}`}>{strategy.risk} Risk</p>
                </div>
              </div>
              <p className="text-gray-400 text-sm mb-3">{strategy.description}</p>
              <div className="flex justify-between items-center">
                <span className="text-sm text-gray-400">Expected APY</span>
                <span className="text-green-400 font-semibold">{strategy.expectedApy}</span>
              </div>
            </div>
          ))}
        </div>
        
        <div className="mt-6 flex flex-col md:flex-row justify-between items-start md:items-center">
          <div className="flex items-start gap-2 text-yellow-400 bg-gray-900 p-3 rounded-lg mb-4 md:mb-0 md:max-w-md">
            <AlertCircle className="h-5 w-5 flex-shrink-0 mt-0.5" />
            <p className="text-sm">
              Changing your strategy will trigger a rebalancing of your portfolio. This may incur gas fees.
            </p>
          </div>
          
          <button 
            className={`${
              isApplying ? 'bg-gray-600 cursor-not-allowed' : 
              isSuccess ? 'bg-green-600 hover:bg-green-700' : 
              'bg-blue-600 hover:bg-blue-700'
            } text-white px-6 py-3 rounded-lg font-medium transition-colors`}
            onClick={handleApplyStrategy}
            disabled={isApplying}
          >
            {isApplying ? 'Applying...' : isSuccess ? 'Strategy Applied!' : 'Apply Strategy'}
          </button>
        </div>
      </div>
      
      <ConnectWalletModal
        isOpen={isConnectModalOpen}
        onClose={() => setIsConnectModalOpen(false)}
      />
    </>
  );
};

export default StrategySelector;