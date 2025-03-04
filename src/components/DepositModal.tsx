import React, { useState } from 'react';
import { X, AlertCircle, ArrowRight } from 'lucide-react';

interface DepositModalProps {
  isOpen: boolean;
  onClose: () => void;
  protocolName: string;
}

const DepositModal: React.FC<DepositModalProps> = ({ isOpen, onClose, protocolName }) => {
  const [amount, setAmount] = useState<string>('');
  const [step, setStep] = useState<number>(1);
  
  if (!isOpen) return null;
  
  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (step === 1) {
      setStep(2);
    } else {
      // Process deposit
      onClose();
      setStep(1);
      setAmount('');
    }
  };
  
  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4 overflow-y-auto">
      <div className="bg-gray-800 rounded-xl w-full max-w-md my-8">
        <div className="flex justify-between items-center p-6 border-b border-gray-700">
          <h2 className="text-xl font-semibold text-white">
            {step === 1 ? `Deposit to ${protocolName}` : 'Confirm Deposit'}
          </h2>
          <button 
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <X className="h-6 w-6" />
          </button>
        </div>
        
        <form onSubmit={handleSubmit}>
          {step === 1 ? (
            <div className="p-6 max-h-[60vh] overflow-y-auto">
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
                  />
                  <button
                    type="button"
                    className="absolute right-3 top-1/2 transform -translate-y-1/2 text-blue-400 text-sm"
                    onClick={() => setAmount('12.45')} // Max amount
                  >
                    MAX
                  </button>
                </div>
                <p className="text-gray-500 text-xs mt-2">
                  Available Balance: 12.45 EGLD
                </p>
              </div>
              
              <div className="bg-gray-700 rounded-lg p-4 mb-6">
                <h3 className="text-white font-semibold mb-2">Deposit Summary</h3>
                <div className="space-y-2">
                  <div className="flex justify-between">
                    <span className="text-gray-400">Protocol</span>
                    <span className="text-white">{protocolName}</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Strategy</span>
                    <span className="text-white">Balanced</span>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-gray-400">Est. APY</span>
                    <span className="text-green-400">12.5%</span>
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
                    <span className="text-gray-400">Protocol</span>
                    <span className="text-white">{protocolName}</span>
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
                  <span className="text-xs font-semibold">{protocolName}</span>
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
              >
                Back
              </button>
            )}
            <button
              type="submit"
              className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors"
            >
              {step === 1 ? 'Continue' : 'Confirm Deposit'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default DepositModal;