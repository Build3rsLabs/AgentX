import React, { useState, useEffect } from 'react';
import { X, AlertCircle, ArrowRight, Info } from 'lucide-react';
import { useProtocol } from '../context/ProtocolContext';
import { Protocol, PoolData } from '../types';
import apiService from '../services/apiService';

interface CreatePositionModalProps {
  isOpen: boolean;
  onClose: () => void;
  protocol: Protocol;
}

const CreatePositionModal: React.FC<CreatePositionModalProps> = ({ isOpen, onClose, protocol }) => {
  const { createPosition } = useProtocol();
  const [step, setStep] = useState<number>(1);
  const [amount, setAmount] = useState<string>('');
  const [selectedPool, setSelectedPool] = useState<string>('');
  const [selectedStrategy, setSelectedStrategy] = useState<string>('balanced');
  const [pools, setPools] = useState<PoolData[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  const [userBalance, setUserBalance] = useState<number>(0);
  
  useEffect(() => {
    if (isOpen) {
      loadData();
    }
  }, [isOpen, protocol]);
  
  const loadData = async () => {
    setIsLoading(true);
    try {
      // Load pools for the protocol
      const protocolPools = await apiService.getPools(protocol.id);
      setPools(protocolPools);
      
      // Set default selected pool if available
      if (protocolPools.length > 0) {
        setSelectedPool(protocolPools[0].id);
      }
      
      // Get user balance
      const balance = await apiService.getUserBalance();
      setUserBalance(balance.egld);
    } catch (error) {
      console.error('Failed to load data:', error);
      setError('Failed to load data. Please try again.');
    } finally {
      setIsLoading(false);
    }
  };
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (step === 1) {
      // Validate inputs
      if (!amount || parseFloat(amount) <= 0) {
        setError('Please enter a valid amount');
        return;
      }
      
      if (!selectedPool) {
        setError('Please select a pool');
        return;
      }
      
      // Move to confirmation step
      setError(null);
      setStep(2);
    } else {
      // Create position
      setIsLoading(true);
      setError(null);
      
      try {
        const pool = pools.find(p => p.id === selectedPool);
        if (!pool) {
          throw new Error('Selected pool not found');
        }
        
        // Create new position
        const newPosition = {
          protocolId: protocol.id,
          name: pool.name,
          type: 'Liquidity Pool',
          tokens: pool.tokens,
          deposited: parseFloat(amount),
          currentValue: parseFloat(amount),
          apy: pool.apy,
          strategy: selectedStrategy === 'conservative' ? 'Conservative' : 
                   selectedStrategy === 'balanced' ? 'Balanced' : 'Aggressive',
          entryDate: new Date().toISOString().split('T')[0],
          lastRebalance: new Date().toISOString().split('T')[0],
          rebalanceFrequency: 'Weekly',
          allocation: pool.tokens.map(token => ({
            token,
            percentage: 100 / pool.tokens.length
          }))
        };
        
        await createPosition(newPosition);
        
        // Reset and close modal
        resetForm();
        onClose();
      } catch (error) {
        console.error('Failed to create position:', error);
        setError('Failed to create position. Please try again.');
      } finally {
        setIsLoading(false);
      }
    }
  };
  
  const resetForm = () => {
    setStep(1);
    setAmount('');
    setSelectedPool('');
    setSelectedStrategy('balanced');
    setError(null);
  };
  
  const handleClose = () => {
    resetForm();
    onClose();
  };
  
  if (!isOpen) return null;
  
  const selectedPoolData = pools.find(p => p.id === selectedPool);
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4 overflow-y-auto">
      <div className="bg-gray-800 rounded-xl w-full max-w-md my-8">
        <div className="flex justify-between items-center p-6 border-b border-gray-700">
          <h2 className="text-xl font-semibold text-white">
            {step === 1 ? `Create Position on ${protocol.name}` : 'Confirm Position'}
          </h2>
          <button 
            onClick={handleClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <X className="h-6 w-6" />
          </button>
        </div>
        
        <form onSubmit={handleSubmit}>
          {error && (
            <div className="mx-6 mt-6 flex items-start gap-2 text-red-400 bg-red-900 bg-opacity-20 p-3 rounded-lg">
              <AlertCircle className="h-5 w-5 flex-shrink-0 mt-0.5" />
              <p className="text-sm">{error}</p>
            </div>
          )}
          
          {step === 1 ? (
            <div className="p-6 max-h-[60vh] overflow-y-auto">
              <div className="mb-6">
                <label className="block text-gray-400 text-sm mb-2">
                  Select Pool
                </label>
                <select
                  className="w-full bg-gray-700 border border-gray-600 text-white rounded-lg p-3 focus:outline-none focus:ring-2 focus:ring-blue-500"
                  value={selectedPool}
                  onChange={(e) => setSelectedPool(e.target.value)}
                  required
                  disabled={isLoading || pools.length === 0}
                >
                  {pools.length === 0 ? (
                    <option value="">No pools available</option>
                  ) : (
                    <>
                      <option value="">Select a pool</option>
                      {pools.map((pool) => (
                        <option key={pool.id} value={pool.id}>
                          {pool.name} - {pool.apy.toFixed(2)}% APY
                        </option>
                      ))}
                    </>
                  )}
                </select>
              </div>
              
              {selectedPoolData && (
                <div className="bg-gray-700 rounded-lg p-4 mb-6">
                  <div className="flex justify-between items-center mb-2">
                    <h3 className="text-white font-semibold">Pool Details</h3>
                    <span className={`px-2 py-1 text-xs font-medium rounded-full ${
                      selectedPoolData.risk === 'Low' ? 'bg-green-900 text-green-300' :
                      selectedPoolData.risk === 'Medium' ? 'bg-yellow-900 text-yellow-300' :
                      'bg-red-900 text-red-300'
                    }`}>
                      {selectedPoolData.risk} Risk
                    </span>
                  </div>
                  <div className="space-y-2">
                    <div className="flex justify-between">
                      <span className="text-gray-400">Protocol</span>
                      <span className="text-white">{selectedPoolData.protocol}</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">APY</span>
                      <span className="text-green-400">{selectedPoolData.apy.toFixed(2)}%</span>
                    </div>
                    <div className="flex justify-between">
                      <span className="text-gray-400">Assets</span>
                      <div className="flex space-x-1">
                        {selectedPoolData.tokens.map(token => (
                          <span key={token} className="bg-gray-600 text-xs text-white px-2 py-1 rounded">
                            {token}
                          </span>
                        ))}
                      </div>
                    </div>
                  </div>
                </div>
              )}
              
              <div className="mb-6">
                <label className="block text-gray-400 text-sm mb-2">
                  Amount to Deposit (EGLD)
                </label>
                <div className="relative">
                  <input
                    type="number"
                    className="w-full bg-gray-700 border border-gray-600 text-white rounded-lg p-3 focus:outline-none focus:ring-2 focus:ring-blue-500"
                    placeholder="0.00"
                    value={amount}
                    onChange={(e) => setAmount(e.target.value)}
                    required
                    min="0.01"
                    step="0.01"
                    disabled={isLoading}
                  />
                  <button
                    type="button"
                    className="absolute right-3 top-1/2 transform -translate-y-1/2 text-blue-400 text-sm"
                    onClick={() => setAmount(userBalance.toString())}
                    disabled={isLoading}
                  >
                    MAX
                  </button>
                </div>
                <p className="text-gray-500 text-xs mt-2">
                  Available Balance: {userBalance} EGLD
                </p>
              </div>
              
              <div className="mb-6">
                <label className="block text-gray-400 text-sm mb-2">
                  Strategy
                </label>
                <div className="space-y-3">
                  <div 
                    className={`border rounded-lg p-4 cursor-pointer transition-all ${
                      selectedStrategy === 'conservative' 
                        ? 'border-green-500 bg-gray-700' 
                        : 'border-gray-700 bg-gray-800 hover:border-gray-600'
                    }`}
                    onClick={() => setSelectedStrategy('conservative')}
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center">
                        <div className="mr-3 text-green-400">
                          <AlertCircle className="h-5 w-5" />
                        </div>
                        <div>
                          <h3 className="text-white font-semibold">Conservative</h3>
                          <p className="text-xs text-green-400">Low Risk</p>
                        </div>
                      </div>
                      <span className="text-green-400 text-sm">5-8% APY</span>
                    </div>
                  </div>
                  
                  <div 
                    className={`border rounded-lg p-4 cursor-pointer transition-all ${
                      selectedStrategy === 'balanced' 
                        ? 'border-blue-500 bg-gray-700' 
                        : 'border-gray-700 bg-gray-800 hover:border-gray-600'
                    }`}
                    onClick={() => setSelectedStrategy('balanced')}
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center">
                        <div className="mr-3 text-blue-400">
                          <Info className="h-5 w-5" />
                        </div>
                        <div>
                          <h3 className="text-white font-semibold">Balanced</h3>
                          <p className="text-xs text-yellow-400">Medium Risk</p>
                        </div>
                      </div>
                      <span className="text-blue-400 text-sm">10-15% APY</span>
                    </div>
                  </div>
                  
                  <div 
                    className={`border rounded-lg p-4 cursor-pointer transition-all ${
                      selectedStrategy === 'aggressive' 
                        ? 'border-red-500 bg-gray-700' 
                        : 'border-gray-700 bg-gray-800 hover:border-gray-600'
                    }`}
                    onClick={() => setSelectedStrategy('aggressive')}
                  >
                    <div className="flex items-center justify-between">
                      <div className="flex items-center">
                        <div className="mr-3 text-red-400">
                          <AlertCircle className="h-5 w-5" />
                        </div>
                        <div>
                          <h3 className="text-white font-semibold">Aggressive</h3>
                          <p className="text-xs text-red-400">High Risk</p>
                        </div>
                      </div>
                      <span className="text-red-400 text-sm">18-25% APY</span>
                    </div>
                  </div>
                </div>
              </div>
              
              <div className="flex items-start gap-2 text-yellow-400 bg-gray-900 p-3 rounded-lg mb-6">
                <AlertCircle className="h-5 w-5 flex-shrink-0 mt-0.5" />
                <p className="text-sm">
                  Your funds will be managed by an automated agent that will optimize for the best yields based on your selected strategy.
                </p>
              </div>
            </div>
          ) : (
            <div className="p-6 max-h-[60vh] overflow-y-auto">
              <div className="bg-gray-700 rounded-lg p-4 mb-6">
                <h3 className="text-white font-semibold mb-3">Confirm Transaction</h3>
                <div className="space-y-3">
                  <div className="flex justify-between">
                    <span className="text-gray-400">Amount</span>
                    <span className="text-white">{amount} EGLD</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Pool</span>
                    <span className="text-white">{selectedPoolData?.name}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Protocol</span>
                    <span className="text-white">{protocol.name}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Strategy</span>
                    <span className="text-white capitalize">{selectedStrategy}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Gas Fee</span>
                    <span className="text-white">~0.0005 EGLD</span>
                  </div>
                  <div className="pt-2 border-t border-gray-600">
                    <div className="flex justify-between">
                      <span className="text-gray-400">Total</span>
                      <span className="text-white font-semibold">
                        {(parseFloat(amount) + 0.0005).toFixed(4)} EGLD
                      </span>
                    </div>
                  </div>
                </div>
              </div>
              
              <div className="flex items-center justify-center gap-3 mb-6">
                <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center">
                  <span className="text-xs font-semibold">EGLD</span>
                </div>
                <ArrowRight className="h-5 w-5 text-gray-400" />
                <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center">
                  <span className="text-xs font-semibold">{selectedPoolData?.tokens.join('-')}</span>
                </div>
              </div>
              
              <div className="flex items-start gap-2 text-yellow-400 bg-gray-900 p-3 rounded-lg mb-6">
                <AlertCircle className="h-5 w-5 flex-shrink-0 mt-0.5" />
                <p className="text-sm">
                  Please review the transaction details carefully. Once confirmed, your agent will begin managing these funds.
                </p>
              </div>
            </div>
          )}
          
          <div className="border-t border-gray-700 p-6 flex justify-end gap-3">
            {step === 2 && (
              <button
                type="button"
                className="bg-gray-700 hover:bg-gray-600 text-white px-6 py-3 rounded-lg font-medium transition-colors"
                onClick={() => setStep(1)}
                disabled={isLoading}
              >
                Back
              </button>
            )}
            <button
              type="submit"
              className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              disabled={isLoading}
            >
              {isLoading ? 'Processing...' : step === 1 ? 'Continue' : 'Confirm Position'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default CreatePositionModal;