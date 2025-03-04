import React, { useState } from 'react';
import { X, AlertCircle, ArrowRight } from 'lucide-react';
import { Position } from '../types';
import { useProtocol } from '../context/ProtocolContext';

interface WithdrawModalProps {
  isOpen: boolean;
  onClose: () => void;
  position: Position;
}

const WithdrawModal: React.FC<WithdrawModalProps> = ({ isOpen, onClose, position }) => {
  const { updatePosition } = useProtocol();
  const [amount, setAmount] = useState<string>('');
  const [step, setStep] = useState<number>(1);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  
  if (!isOpen) return null;
  
  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    
    if (step === 1) {
      // Validate amount
      const withdrawAmount = parseFloat(amount);
      if (isNaN(withdrawAmount) || withdrawAmount <= 0) {
        setError('Please enter a valid amount');
        return;
      }
      
      if (withdrawAmount > position.currentValue) {
        setError(`You cannot withdraw more than your current balance (${position.currentValue} EGLD)`);
        return;
      }
      
      // Move to confirmation step
      setError(null);
      setStep(2);
    } else {
      // Process withdrawal
      setIsLoading(true);
      setError(null);
      
      try {
        const withdrawAmount = parseFloat(amount);
        
        // If withdrawing all, delete the position
        if (withdrawAmount >= position.currentValue) {
          // In a real app, this would call an API to withdraw all funds
          // For demo purposes, we'll just update the position
          await updatePosition(position.id, {
            currentValue: 0,
            deposited: 0
          });
        } else {
          // Partial withdrawal
          const newValue = position.currentValue - withdrawAmount;
          const newDeposited = position.deposited * (newValue / position.currentValue);
          
          await updatePosition(position.id, {
            currentValue: newValue,
            deposited: newDeposited
          });
        }
        
        // Reset and close modal
        setAmount('');
        setStep(1);
        onClose();
      } catch (error) {
        console.error('Withdrawal error:', error);
        setError('Failed to process withdrawal. Please try again.');
      } finally {
        setIsLoading(false);
      }
    }
  };
  
  const handleClose = () => {
    setAmount('');
    setStep(1);
    setError(null);
    onClose();
  };
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4 overflow-y-auto">
      <div className="bg-gray-800 rounded-xl w-full max-w-md my-8">
        <div className="flex justify-between items-center p-6 border-b border-gray-700">
          <h2 className="text-xl font-semibold text-white">
            {step === 1 ? `Withdraw from ${position.name}` : 'Confirm Withdrawal'}
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
                  Amount to Withdraw (EGLD)
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
                    max={position.currentValue}
                    disabled={isLoading}
                  />
                  <button
                    type="button"
                    className="absolute right-3 top-1/2 transform -translate-y-1/2 text-blue-400 text-sm"
                    onClick={() => setAmount(position.currentValue.toString())}
                    disabled={isLoading}
                  >
                    MAX
                  </button>
                </div>
                <p className="text-gray-500 text-xs mt-2">
                  Available Balance: {position.currentValue.toFixed(4)} EGLD
                </p>
              </div>
              
              <div className="bg-gray-700 rounded-lg p-4 mb-6">
                <h3 className="text-white font-semibold mb-2">Position Summary</h3>
                <div className="space-y-2">
                  <div className="flex justify-between">
                    <span className="text-gray-400">Position</span>
                    <span className="text-white">{position.name}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Deposited</span>
                    <span className="text-white">{position.deposited.toFixed(4)} EGLD</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Current Value</span>
                    <span className="text-white">{position.currentValue.toFixed(4)} EGLD</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Profit/Loss</span>
                    <span className={`${position.currentValue > position.deposited ? 'text-green-400' : 'text-red-400'}`}>
                      {(position.currentValue - position.deposited).toFixed(4)} EGLD
                    </span>
                  </div>
                </div>
              </div>
              
              <div className="flex items-start gap-2 text-yellow-400 bg-gray-900 p-3 rounded-lg mb-6">
                <AlertCircle className="h-5 w-5 flex-shrink-0 mt-0.5" />
                <p className="text-sm">
                  Withdrawing funds may affect your overall yield strategy. Your agent will rebalance your remaining funds accordingly.
                </p>
              </div>
            </div>
          ) : (
            <div className="p-6 max-h-[60vh] overflow-y-auto">
              <div className="bg-gray-700 rounded-lg p-4 mb-6">
                <h3 className="text-white font-semibold mb-3">Confirm Withdrawal</h3>
                <div className="space-y-3">
                  <div className="flex justify-between">
                    <span className="text-gray-400">Amount</span>
                    <span className="text-white">{amount} EGLD</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Position</span>
                    <span className="text-white">{position.name}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Gas Fee</span>
                    <span className="text-white">~0.0005 EGLD</span>
                  </div>
                  <div className="pt-2 border-t border-gray-600">
                    <div className="flex justify-between">
                      <span className="text-gray-400">You will receive</span>
                      <span className="text-white font-semibold">
                        {(parseFloat(amount) - 0.0005).toFixed(4)} EGLD
                      </span>
                    </div>
                  </div>
                </div>
              </div>
              
              <div className="flex items-center justify-center gap-3 mb-6">
                <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center">
                  <span className="text-xs font-semibold">{position.tokens.join('-')}</span>
                </div>
                <ArrowRight className="h-5 w-5 text-gray-400" />
                <div className="w-10 h-10 rounded-full bg-gray-700 flex items-center justify-center">
                  <span className="text-xs font-semibold">EGLD</span>
                </div>
              </div>
              
              <div className="flex items-start gap-2 text-yellow-400 bg-gray-900 p-3 rounded-lg mb-6">
                <AlertCircle className="h-5 w-5 flex-shrink-0 mt-0.5" />
                <p className="text-sm">
                  Please review the transaction details carefully. Once confirmed, the withdrawal cannot be undone.
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
              {isLoading ? 'Processing...' : step === 1 ? 'Continue' : 'Confirm Withdrawal'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default WithdrawModal;