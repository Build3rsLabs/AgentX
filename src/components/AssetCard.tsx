import React from 'react';
import { TrendingUp, TrendingDown } from 'lucide-react';
import { Asset } from '../types';

interface AssetCardProps {
  asset: Asset;
}

const AssetCard: React.FC<AssetCardProps> = ({ asset }) => {
  const formatPrice = (price: number) => {
    if (price < 0.01) {
      return price.toFixed(6);
    } else if (price < 1) {
      return price.toFixed(4);
    } else {
      return price.toFixed(2);
    }
  };

  return (
    <div className="bg-white rounded-xl shadow-md overflow-hidden hover:shadow-lg transition-shadow duration-300">
      <div className="p-6">
        <div className="flex items-center mb-4">
          <img src={asset.logo} alt={asset.name} className="w-10 h-10 rounded-full mr-3" />
          <div>
            <h3 className="text-xl font-semibold text-gray-900">{asset.symbol}</h3>
            <p className="text-sm text-gray-500">{asset.name}</p>
          </div>
        </div>
        
        <div className="flex justify-between items-center">
          <div>
            <p className="text-sm text-gray-500">Price</p>
            <p className="text-lg font-semibold">
              ${formatPrice(asset.price)}
            </p>
          </div>
          
          <div>
            <p className="text-sm text-gray-500">24h Change</p>
            <div className={`flex items-center ${asset.change24h >= 0 ? 'text-green-600' : 'text-red-600'}`}>
              {asset.change24h >= 0 ? (
                <TrendingUp className="h-4 w-4 mr-1" />
              ) : (
                <TrendingDown className="h-4 w-4 mr-1" />
              )}
              <p className="font-semibold">{Math.abs(asset.change24h).toFixed(2)}%</p>
            </div>
          </div>
        </div>
        
        <div className="mt-4 pt-4 border-t border-gray-100">
          <button className="w-full bg-blue-600 hover:bg-blue-700 text-white py-2 px-4 rounded-lg transition-colors">
            Trade {asset.symbol}
          </button>
        </div>
      </div>
    </div>
  );
};

export default AssetCard;