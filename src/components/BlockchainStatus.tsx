import React, { useState, useEffect } from 'react';
import { Server, CheckCircle, AlertCircle, Loader2 } from 'lucide-react';

const BlockchainStatus: React.FC = () => {
  const [status, setStatus] = useState<'connecting' | 'connected' | 'error'>('connecting');
  const [message, setMessage] = useState<string>('Connecting to blockchain...');
  const [networkDetails, setNetworkDetails] = useState<{
    name: string;
    chainId: string;
    latestBlock: number;
  } | null>(null);

  useEffect(() => {
    const checkBlockchainStatus = async () => {
      try {
        // Simulate blockchain connection and data fetching
        await new Promise(resolve => setTimeout(resolve, 1500));
        
        setStatus('connected');
        setMessage('Connected to MultiversX Devnet');
        setNetworkDetails({
          name: 'MultiversX Devnet',
          chainId: 'D',
          latestBlock: 15384621,
        });
      } catch (error) {
        console.error('Blockchain connection error:', error);
        setStatus('error');
        setMessage('Failed to connect to blockchain');
      }
    };

    checkBlockchainStatus();
    
    // Set up periodic status check
    const intervalId = setInterval(checkBlockchainStatus, 30000);
    
    return () => clearInterval(intervalId);
  }, []);

  const handleRetry = async () => {
    setStatus('connecting');
    setMessage('Reconnecting to blockchain...');
    
    try {
      // Simulate reconnection
      await new Promise(resolve => setTimeout(resolve, 1500));
      
      setStatus('connected');
      setMessage('Connected to MultiversX Devnet');
      setNetworkDetails({
        name: 'MultiversX Devnet',
        chainId: 'D',
        latestBlock: 15384621,
      });
    } catch (error) {
      console.error('Blockchain reconnection error:', error);
      setStatus('error');
      setMessage('Failed to connect to blockchain');
    }
  };

  return (
    <div className="bg-gray-800 rounded-lg p-4">
      <div className="flex items-center justify-between">
        <div className="flex items-center">
          {status === 'connecting' && (
            <>
              <Loader2 className="h-5 w-5 text-blue-400 mr-2 animate-spin" />
              <span className="text-gray-300 text-sm">{message}</span>
            </>
          )}
          
          {status === 'connected' && (
            <>
              <CheckCircle className="h-5 w-5 text-green-400 mr-2" />
              <span className="text-gray-300 text-sm">{message}</span>
            </>
          )}
          
          {status === 'error' && (
            <>
              <AlertCircle className="h-5 w-5 text-red-400 mr-2" />
              <span className="text-gray-300 text-sm">{message}</span>
              <button 
                className="ml-2 text-blue-400 text-xs hover:text-blue-300 transition-colors"
                onClick={handleRetry}
              >
                Retry
              </button>
            </>
          )}
        </div>
        
        {status === 'connected' && networkDetails && (
          <div className="text-xs text-gray-400">
            <span className="mr-3">Chain ID: {networkDetails.chainId}</span>
            <span>Block: #{networkDetails.latestBlock.toLocaleString()}</span>
          </div>
        )}
      </div>
    </div>
  );
};

export default BlockchainStatus;