import React from 'react';
import { ExternalLink } from 'lucide-react';
import { Protocol } from '../types';

interface ProtocolCardProps {
  protocol: Protocol;
}

const ProtocolCard: React.FC<ProtocolCardProps> = ({ protocol }) => {
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
        return 'bg-green-100 text-green-800';
      case 'Medium':
        return 'bg-yellow-100 text-yellow-800';
      case 'High':
        return 'bg-red-100 text-red-800';
      default:
        return 'bg-gray-100 text-gray-800';
    }
  };

  return (
    <div className="bg-white rounded-xl shadow-md overflow-hidden hover:shadow-lg transition-shadow duration-300">
      <div className="p-6">
        <div className="flex items-center justify-between mb-4">
          <div className="flex items-center">
            <img src={protocol.logo} alt={protocol.name} className="w-10 h-10 rounded-full mr-3" />
            <h3 className="text-xl font-semibold text-gray-900">{protocol.name}</h3>
          </div>
          <span className={`px-3 py-1 rounded-full text-xs font-medium ${getRiskColor(protocol.risk)}`}>
            {protocol.risk} Risk
          </span>
        </div>
        
        <p className="text-gray-600 mb-4 h-12 line-clamp-2">{protocol.description}</p>
        
        <div className="grid grid-cols-2 gap-4 mb-4">
          <div>
            <p className="text-sm text-gray-500">TVL</p>
            <p className="text-lg font-semibold">{formatTVL(protocol.tvl)}</p>
          </div>
          <div>
            <p className="text-sm text-gray-500">APY</p>
            <p className="text-lg font-semibold text-green-600">{protocol.apy.toFixed(2)}%</p>
          </div>
        </div>
        
        <div className="mb-4">
          <p className="text-sm text-gray-500 mb-2">Supported Assets</p>
          <div className="flex flex-wrap gap-2">
            {protocol.tokens.map((token) => (
              <span key={token} className="bg-blue-50 text-blue-700 px-2 py-1 rounded-md text-xs">
                {token}
              </span>
            ))}
          </div>
        </div>
        
        <a 
          href={protocol.url} 
          target="_blank" 
          rel="noopener noreferrer"
          className="flex items-center justify-center w-full bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded-lg transition-colors"
        >
          Visit Protocol <ExternalLink className="ml-2 h-4 w-4" />
        </a>
      </div>
    </div>
  );
};

export default ProtocolCard;