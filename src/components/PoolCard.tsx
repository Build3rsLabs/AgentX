import React from 'react';
import { PoolData } from '../types';

interface PoolCardProps {
  pool: PoolData;
}

const PoolCard: React.FC<PoolCardProps> = ({ pool }) => {
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
          <div>
            <h3 className="text-xl font-semibold text-gray-900">{pool.name}</h3>
            <p className="text-sm text-gray-500">{pool.protocol}</p>
          </div>
          <span className={`px-3 py-1 rounded-full text-xs font-medium ${getRiskColor(pool.risk)}`}>
            {pool.risk} Risk
          </span>
        </div>
        
        <div className="grid grid-cols-2 gap-4 mb-4">
          <div>
            <p className="text-sm text-gray-500">TVL</p>
            <p className="text-lg font-semibold">{formatTVL(pool.tvl)}</p>
          </div>
          <div>
            <p className="text-sm text-gray-500">APY</p>
            <p className="text-lg font-semibold text-green-600">{pool.apy.toFixed(2)}%</p>
          </div>
        </div>
        
        <div className="mb-4">
          <p className="text-sm text-gray-500 mb-2">Assets</p>
          <div className="flex flex-wrap gap-2">
            {pool.tokens.map((token) => (
              <span key={token} className="bg-blue-50 text-blue-700 px-2 py-1 rounded-md text-xs">
                {token}
              </span>
            ))}
          </div>
        </div>
        
        <button className="w-full bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded-lg transition-colors">
          Invest Now
        </button>
      </div>
    </div>
  );
};

export default PoolCard;