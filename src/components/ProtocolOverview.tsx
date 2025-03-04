import React from 'react';
import { Shield, TrendingUp, DollarSign, Users } from 'lucide-react';
import { Protocol } from '../types';

interface ProtocolOverviewProps {
  protocol: Protocol;
}

const ProtocolOverview: React.FC<ProtocolOverviewProps> = ({ protocol }) => {
  const formatTVL = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      notation: 'compact',
      maximumFractionDigits: 2
    }).format(value);
  };

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

  return (
    <div className="bg-gray-800 rounded-lg p-6 mb-6">
      <div className="flex flex-col md:flex-row items-start md:items-center gap-6">
        <img 
          src={protocol.logo} 
          alt={protocol.name} 
          className="w-16 h-16 rounded-full"
        />
        
        <div className="flex-grow">
          <h1 className="text-2xl font-bold text-white mb-2">{protocol.name}</h1>
          <p className="text-gray-300 mb-4">{protocol.description}</p>
          
          <div className="flex flex-wrap gap-4">
            <div className="flex items-center">
              <DollarSign className="h-5 w-5 text-blue-400 mr-2" />
              <div>
                <p className="text-sm text-gray-400">TVL</p>
                <p className="font-semibold text-white">{formatTVL(protocol.tvl)}</p>
              </div>
            </div>
            
            <div className="flex items-center">
              <TrendingUp className="h-5 w-5 text-green-400 mr-2" />
              <div>
                <p className="text-sm text-gray-400">Avg. APY</p>
                <p className="font-semibold text-white">{protocol.apy.toFixed(2)}%</p>
              </div>
            </div>
            
            <div className="flex items-center">
              <Shield className="h-5 w-5 text-purple-400 mr-2" />
              <div>
                <p className="text-sm text-gray-400">Risk Level</p>
                <p className={`font-semibold ${getRiskColor(protocol.risk)}`}>{protocol.risk}</p>
              </div>
            </div>
            
            <div className="flex items-center">
              <Users className="h-5 w-5 text-yellow-400 mr-2" />
              <div>
                <p className="text-sm text-gray-400">Supported Assets</p>
                <div className="flex flex-wrap gap-1 mt-1">
                  {protocol.tokens.map(token => (
                    <span key={token} className="bg-gray-700 text-xs text-gray-300 px-2 py-1 rounded">
                      {token}
                    </span>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </div>
        
        <a 
          href={protocol.url}
          target="_blank"
          rel="noopener noreferrer"
          className="bg-blue-600 hover:bg-blue-700 text-white px-6 py-3 rounded-lg font-medium transition-colors whitespace-nowrap"
        >
          Visit Protocol
        </a>
      </div>
    </div>
  );
};

export default ProtocolOverview;