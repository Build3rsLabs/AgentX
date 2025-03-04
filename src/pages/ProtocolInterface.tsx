import React, { useState, useEffect } from 'react';
import { useParams, useNavigate } from 'react-router-dom';
import { PlusCircle, ArrowUpDown, History, Settings, Wallet as WalletIcon } from 'lucide-react';
import ProtocolSelector from '../components/ProtocolSelector';
import ProtocolOverview from '../components/ProtocolOverview';
import PositionCard from '../components/PositionCard';
import StrategySelector from '../components/StrategySelector';
import PerformanceChart from '../components/PerformanceChart';
import TransactionHistory from '../components/TransactionHistory';
import CreatePositionModal from '../components/CreatePositionModal';
import DepositModal from '../components/DepositModal';
import { useAuth } from '../context/AuthContext';
import { useProtocol } from '../context/ProtocolContext';
import { performanceData } from '../data/performance';
import ConnectWalletModal from '../components/ConnectWalletModal';

const ProtocolInterface: React.FC = () => {
  const { protocolId } = useParams();
  const navigate = useNavigate();
  const { isAuthenticated } = useAuth();
  const { 
    protocols, 
    selectedProtocol, 
    userPositions, 
    isLoading, 
    error, 
    selectProtocol 
  } = useProtocol();
  
  const [isCreateModalOpen, setIsCreateModalOpen] = useState(false);
  const [isDepositModalOpen, setIsDepositModalOpen] = useState(false);
  const [isConnectModalOpen, setIsConnectModalOpen] = useState(false);
  const [activeTab, setActiveTab] = useState('positions');
  
  // Update selected protocol when URL param changes
  useEffect(() => {
    const loadProtocol = async () => {
      if (protocols.length > 0) {
        if (protocolId) {
          // Check if the protocol exists
          const protocolExists = protocols.some(p => p.id === protocolId);
          if (protocolExists) {
            await selectProtocol(protocolId);
          } else {
            // If protocol doesn't exist, navigate to the first available protocol
            console.warn(`Protocol with ID ${protocolId} not found, redirecting to first available protocol`);
            navigate(`/earn/${protocols[0].id}`);
          }
        } else if (protocols[0]) {
          // If no protocol ID in URL, navigate to the first available protocol
          navigate(`/earn/${protocols[0].id}`);
        }
      }
    };
    
    loadProtocol();
  }, [protocolId, protocols, navigate, selectProtocol]);

  const handleCreatePosition = () => {
    if (!isAuthenticated) {
      setIsConnectModalOpen(true);
    } else {
      setIsCreateModalOpen(true);
    }
  };

  const handleDeposit = () => {
    if (!isAuthenticated) {
      setIsConnectModalOpen(true);
    } else {
      setIsDepositModalOpen(true);
    }
  };

  return (
    <div className="container mx-auto px-4 py-8">
      <ProtocolSelector />
      
      {error && (
        <div className="bg-red-900 bg-opacity-20 text-red-400 p-6 rounded-lg mb-6">
          <h2 className="text-xl font-semibold mb-2">Error</h2>
          <p>{error}</p>
          <button 
            onClick={() => window.location.reload()}
            className="mt-4 bg-red-600 hover:bg-red-700 text-white px-4 py-2 rounded-lg"
          >
            Reload Page
          </button>
        </div>
      )}
      
      {selectedProtocol && (
        <ProtocolOverview protocol={selectedProtocol} />
      )}
      
      <div className="bg-gray-800 rounded-lg p-6 mb-6">
        <div className="flex flex-col md:flex-row justify-between items-start md:items-center mb-6">
          <div>
            <h2 className="text-xl font-semibold text-white">Your Positions</h2>
            <p className="text-gray-400">Manage your active positions and strategies</p>
          </div>
          
          <div className="flex mt-4 md:mt-0">
            <button
              className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg font-medium transition-colors flex items-center"
              onClick={handleCreatePosition}
            >
              <PlusCircle className="h-5 w-5 mr-2" />
              New Position
            </button>
          </div>
        </div>
        
        <div className="flex border-b border-gray-700 mb-6">
          <button
            className={`px-4 py-2 font-medium text-sm ${
              activeTab === 'positions' 
                ? 'text-white border-b-2 border-blue-500' 
                : 'text-gray-400 hover:text-white'
            }`}
            onClick={() => setActiveTab('positions')}
          >
            Positions
          </button>
          <button
            className={`px-4 py-2 font-medium text-sm ${
              activeTab === 'strategy' 
                ? 'text-white border-b-2 border-blue-500' 
                : 'text-gray-400 hover:text-white'
            }`}
            onClick={() => setActiveTab('strategy')}
          >
            Strategy
          </button>
          <button
            className={`px-4 py-2 font-medium text-sm ${
              activeTab === 'history' 
                ? 'text-white border-b-2 border-blue-500' 
                : 'text-gray-400 hover:text-white'
            }`}
            onClick={() => setActiveTab('history')}
          >
            History
          </button>
          <button
            className={`px-4 py-2 font-medium text-sm ${
              activeTab === 'settings' 
                ? 'text-white border-b-2 border-blue-500' 
                : 'text-gray-400 hover:text-white'
            }`}
            onClick={() => setActiveTab('settings')}
          >
            Settings
          </button>
        </div>
        
        {activeTab === 'positions' && (
          <div>
            {!isAuthenticated ? (
              <div className="text-center py-12">
                <div className="bg-gray-700 inline-block p-4 rounded-full mb-4">
                  <WalletIcon className="h-8 w-8 text-gray-400" />
                </div>
                <h3 className="text-white text-lg font-semibold mb-2">Connect Wallet to View Positions</h3>
                <p className="text-gray-400 max-w-md mx-auto mb-6">
                  Connect your MultiversX wallet to view and manage your positions.
                </p>
                <button
                  className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
                  onClick={() => setIsConnectModalOpen(true)}
                >
                  Connect Wallet
                </button>
              </div>
            ) : userPositions.length > 0 ? (
              <div className="space-y-4">
                {userPositions.map((position) => (
                  <PositionCard 
                    key={position.id} 
                    position={position} 
                    onDeposit={handleDeposit}
                  />
                ))}
              </div>
            ) : (
              <div className="text-center py-12">
                <div className="bg-gray-700 inline-block p-4 rounded-full mb-4">
                  <ArrowUpDown className="h-8 w-8 text-gray-400" />
                </div>
                <h3 className="text-white text-lg font-semibold mb-2">No Active Positions</h3>
                <p className="text-gray-400 max-w-md mx-auto mb-6">
                  You don't have any active positions with this protocol. Create a new position to start earning yield.
                </p>
                <button
                  className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
                  onClick={() => setIsCreateModalOpen(true)}
                >
                  Create Position
                </button>
              </div>
            )}
          </div>
        )}
        
        {activeTab === 'strategy' && (
          <StrategySelector />
        )}
        
        {activeTab === 'history' && (
          <div>
            <PerformanceChart 
              data={performanceData} 
              title="Portfolio Performance" 
            />
            
            <TransactionHistory />
          </div>
        )}
        
        {activeTab === 'settings' && (
          <div className="space-y-6">
            <div className="bg-gray-700 rounded-lg p-6">
              <h3 className="text-white font-semibold mb-4">Rebalancing Settings</h3>
              <div className="space-y-4">
                <div>
                  <label className="block text-gray-400 text-sm mb-2">
                    Rebalance Frequency
                  </label>
                  <select className="w-full bg-gray-800 border border-gray-600 text-white rounded-lg p-3 focus:outline-none focus:ring-2 focus:ring-blue-500">
                    <option value="daily">Daily</option>
                    <option value="weekly" selected>Weekly</option>
                    <option value="monthly">Monthly</option>
                    <option value="manual">Manual Only</option>
                  </select>
                </div>
                
                <div>
                  <label className="block text-gray-400 text-sm mb-2">
                    Gas Price Threshold (for automatic rebalancing)
                  </label>
                  <select className="w-full bg-gray-800 border border-gray-600 text-white rounded-lg p-3 focus:outline-none focus:ring-2 focus:ring-blue-500">
                    <option value="low">Low (Slower)</option>
                    <option value="medium" selected>Medium</option>
                    <option value="high">High (Faster)</option>
                  </select>
                </div>
                
                <div className="flex items-center">
                  <input 
                    type="checkbox" 
                    id="slippageProtection" 
                    className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-600 rounded bg-gray-800"
                    defaultChecked
                  />
                  <label htmlFor="slippageProtection" className="ml-2 block text-white">
                    Enable slippage protection (max 1%)
                  </label>
                </div>
              </div>
            </div>
            
            <div className="bg-gray-700 rounded-lg p-6">
              <h3 className="text-white font-semibold mb-4">Notification Settings</h3>
              <div className="space-y-4">
                <div className="flex items-center">
                  <input 
                    type="checkbox" 
                    id="emailNotifications" 
                    className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-600 rounded bg-gray-800"
                    defaultChecked
                  />
                  <label htmlFor="emailNotifications" className="ml-2 block text-white">
                    Email notifications for deposits and withdrawals
                  </label>
                </div>
                
                <div className="flex items-center">
                  <input 
                    type="checkbox" 
                    id="rebalanceNotifications" 
                    className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-600 rounded bg-gray-800"
                    defaultChecked
                  />
                  <label htmlFor="rebalanceNotifications" className="ml-2 block text-white">
                    Email notifications for rebalancing events
                  </label>
                </div>
                
                <div className="flex items-center">
                  <input 
                    type="checkbox" 
                    id="yieldAlerts" 
                    className="h-4 w-4 text-blue-600 focus:ring-blue-500 border-gray-600 rounded bg-gray-800"
                    defaultChecked
                  />
                  <label htmlFor="yieldAlerts" className="ml-2 block text-white">
                    Yield opportunity alerts
                  </label>
                </div>
              </div>
            </div>
            
            <div className="flex justify-end">
              <button className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors">
                Save Settings
              </button>
            </div>
          </div>
        )}
      </div>
      
      {selectedProtocol && (
        <>
          <CreatePositionModal 
            isOpen={isCreateModalOpen} 
            onClose={() => setIsCreateModalOpen(false)}
            protocol={selectedProtocol}
          />
          
          <DepositModal
            isOpen={isDepositModalOpen}
            onClose={() => setIsDepositModalOpen(false)}
            protocolName={selectedProtocol.name}
          />
        </>
      )}
      
      <ConnectWalletModal
        isOpen={isConnectModalOpen}
        onClose={() => setIsConnectModalOpen(false)}
      />
    </div>
  );
};

export default ProtocolInterface;