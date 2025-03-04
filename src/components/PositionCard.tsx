import React, { useState } from 'react';
import { ArrowRight, ChevronDown, ChevronUp, AlertCircle } from 'lucide-react';
import { Position } from '../types';
import WithdrawModal from './WithdrawModal';

interface PositionCardProps {
  position: Position;
  onDeposit: () => void;
}

const PositionCard: React.FC<PositionCardProps> = ({ position, onDeposit }) => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isWithdrawModalOpen, setIsWithdrawModalOpen] = useState(false);
  
  const formatValue = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      maximumFractionDigits: 2
    }).format(value);
  };

  return (
    <>
      <div className="bg-gray-800 rounded-lg overflow-hidden mb-4">
        <div className="p-5">
          <div className="flex flex-col md:flex-row md:items-center justify-between gap-4">
            <div className="flex items-center">
              <div className="flex -space-x-2 mr-4">
                {position.tokens.map((token, index) => (
                  <div key={index} className="w-10 h-10 rounded-full bg-gray-700 border-2 border-gray-800 flex items-center justify-center">
                    <span className="text-xs font-semibold">{token}</span>
                  </div>
                ))}
              </div>
              <div>
                <h3 className="text-lg font-semibold text-white">{position.name}</h3>
                <p className="text-sm text-gray-400">{position.type}</p>
              </div>
            </div>
            
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div>
                <p className="text-xs text-gray-400">Deposited</p>
                <p className="text-white font-semibold">{formatValue(position.deposited)}</p>
              </div>
              <div>
                <p className="text-xs text-gray-400">Current Value</p>
                <p className="text-white font-semibold">{formatValue(position.currentValue)}</p>
              </div>
              <div>
                <p className="text-xs text-gray-400">APY</p>
                <p className="text-green-400 font-semibold">{position.apy.toFixed(2)}%</p>
              </div>
              <div>
                <p className="text-xs text-gray-400">Earned</p>
                <p className="text-green-400 font-semibold">
                  {formatValue(position.currentValue - position.deposited)}
                </p>
              </div>
            </div>
            
            <div className="flex space-x-2">
              <button 
                className="bg-blue-600 hover:bg-blue-700 text-white px-4 py-2 rounded-lg transition-colors text-sm"
                onClick={onDeposit}
              >
                Deposit
              </button>
              <button 
                className="bg-gray-700 hover:bg-gray-600 text-white px-4 py-2 rounded-lg transition-colors text-sm"
                onClick={() => setIsWithdrawModalOpen(true)}
              >
                Withdraw
              </button>
              <button
                className="bg-gray-700 hover:bg-gray-600 text-white p-2 rounded-lg transition-colors"
                onClick={() => setIsExpanded(!isExpanded)}
              >
                {isExpanded ? <ChevronUp className="h-5 w-5" /> : <ChevronDown className="h-5 w-5" />}
              </button>
            </div>
          </div>
        </div>
        
        {isExpanded && (
          <div className="bg-gray-700 p-5 border-t border-gray-600">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div>
                <h4 className="text-white font-semibold mb-3">Position Details</h4>
                <div className="space-y-2">
                  <div className="flex justify-between">
                    <span className="text-gray-400">Strategy</span>
                    <span className="text-white">{position.strategy}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Entry Date</span>
                    <span className="text-white">{position.entryDate}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Last Rebalance</span>
                    <span className="text-white">{position.lastRebalance}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Rebalance Frequency</span>
                    <span className="text-white">{position.rebalanceFrequency}</span>
                  </div>
                </div>
              </div>
              
              <div>
                <h4 className="text-white font-semibold mb-3">Asset Allocation</h4>
                <div className="space-y-3">
                  {position.allocation.map((item, index) => (
                    <div key={index}>
                      <div className="flex justify-between text-sm mb-1">
                        <span className="text-gray-400">{item.token}</span>
                        <span className="text-white">{item.percentage}%</span>
                      </div>
                      <div className="w-full bg-gray-600 rounded-full h-2">
                        <div 
                          className="bg-blue-500 h-2 rounded-full" 
                          style={{ width: `${item.percentage}%` }}
                        ></div>
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            </div>
            
            <div className="mt-6 pt-4 border-t border-gray-600">
              <div className="flex items-start gap-2 text-yellow-400 bg-gray-800 p-3 rounded-lg">
                <AlertCircle className="h-5 w-5 flex-shrink-0 mt-0.5" />
                <div>
                  <p className="text-sm">
                    This position is managed by your yield agent. The agent will automatically rebalance your assets to maximize returns based on your risk preferences.
                  </p>
                </div>
              </div>
            </div>
          </div>
        )}
      </div>
      
      <WithdrawModal 
        isOpen={isWithdrawModalOpen}
        onClose={() => setIsWithdrawModalOpen(false)}
        position={position}
      />
    </>
  );
};

export default PositionCard;