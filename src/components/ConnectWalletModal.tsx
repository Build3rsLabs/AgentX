import React, { useState } from 'react';
import { X, Wallet, AlertCircle } from 'lucide-react';
import { useAuth } from '../context/AuthContext';

interface ConnectWalletModalProps {
  isOpen: boolean;
  onClose: () => void;
}

const ConnectWalletModal: React.FC<ConnectWalletModalProps> = ({ isOpen, onClose }) => {
  const { login } = useAuth();
  const [isConnecting, setIsConnecting] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!isOpen) return null;

  const handleConnect = async (walletType: string) => {
    setIsConnecting(true);
    setError(null);
    
    try {
      // In a real app, this would connect to the actual wallet
      // For demo purposes, we'll simulate a connection with a mock address
      let address;
      
      if (walletType === 'maiar') {
        address = '0x7a69d2Ef3BA777e3f4d7138B0D5f3f4d';
      } else if (walletType === 'ledger') {
        address = '0x8b45c23F9B4d7138B0D5f3f4d7a69d2E';
      } else if (walletType === 'webwallet') {
        address = '0x3f4d7a69d2Ef3BA777e3f4d7138B0D5f';
      } else {
        throw new Error('Unsupported wallet type');
      }
      
      await login(address);
      onClose();
    } catch (error) {
      console.error('Wallet connection error:', error);
      setError('Failed to connect wallet. Please try again.');
    } finally {
      setIsConnecting(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-gray-800 rounded-xl w-full max-w-md">
        <div className="flex justify-between items-center p-6 border-b border-gray-700">
          <h2 className="text-xl font-semibold text-white">Connect Wallet</h2>
          <button 
            onClick={onClose}
            className="text-gray-400 hover:text-white transition-colors"
          >
            <X className="h-6 w-6" />
          </button>
        </div>
        
        <div className="p-6">
          {error && (
            <div className="flex items-start gap-2 text-red-400 bg-red-900 bg-opacity-20 p-3 rounded-lg mb-6">
              <AlertCircle className="h-5 w-5 flex-shrink-0 mt-0.5" />
              <p className="text-sm">{error}</p>
            </div>
          )}
          
          <p className="text-gray-300 mb-6">
            Connect your MultiversX wallet to access the platform and manage your yield positions.
          </p>
          
          <div className="space-y-4">
            <button
              className="w-full bg-blue-600 hover:bg-blue-700 text-white p-4 rounded-lg flex items-center justify-between transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => handleConnect('maiar')}
              disabled={isConnecting}
            >
              <div className="flex items-center">
                <div className="bg-white p-2 rounded-full mr-3">
                  <Wallet className="h-5 w-5 text-blue-600" />
                </div>
                <span className="font-medium">xPortal App</span>
              </div>
              <span className="text-sm text-blue-300">Recommended</span>
            </button>
            
            <button
              className="w-full bg-gray-700 hover:bg-gray-600 text-white p-4 rounded-lg flex items-center transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => handleConnect('ledger')}
              disabled={isConnecting}
            >
              <div className="bg-white p-2 rounded-full mr-3">
                <Wallet className="h-5 w-5 text-gray-800" />
              </div>
              <span className="font-medium">Ledger Hardware Wallet</span>
            </button>
            
            <button
              className="w-full bg-gray-700 hover:bg-gray-600 text-white p-4 rounded-lg flex items-center transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              onClick={() => handleConnect('webwallet')}
              disabled={isConnecting}
            >
              <div className="bg-white p-2 rounded-full mr-3">
                <Wallet className="h-5 w-5 text-gray-800" />
              </div>
              <span className="font-medium">MultiversX Web Wallet</span>
            </button>
          </div>
          
          <div className="mt-6 pt-6 border-t border-gray-700">
            <p className="text-gray-400 text-sm text-center">
              By connecting your wallet, you agree to our Terms of Service and Privacy Policy.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ConnectWalletModal;