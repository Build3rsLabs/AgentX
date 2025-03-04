import React, { useState, useEffect } from 'react';
import { format } from 'date-fns';
import { ArrowUpRight, ArrowDownRight, RefreshCw, ExternalLink } from 'lucide-react';
import apiService from '../services/apiService';

interface Transaction {
  id: string;
  date: string;
  type: string;
  amount: number | null;
  token: string | null;
  status: string;
}

const TransactionHistory: React.FC = () => {
  const [transactions, setTransactions] = useState<Transaction[]>([]);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    loadTransactions();
  }, []);

  const loadTransactions = async () => {
    setError(null);
    try {
      const data = await apiService.getTransactionHistory();
      setTransactions(data);
    } catch (error) {
      console.error('Failed to load transactions:', error);
      setError('Failed to load transaction history. Please try again later.');
    }
  };

  const getTransactionIcon = (type: string) => {
    switch (type) {
      case 'Deposit':
        return <ArrowDownRight className="h-4 w-4 text-green-400" />;
      case 'Withdraw':
        return <ArrowUpRight className="h-4 w-4 text-red-400" />;
      case 'Rebalance':
        return <RefreshCw className="h-4 w-4 text-blue-400" />;
      default:
        return <ExternalLink className="h-4 w-4 text-gray-400" />;
    }
  };

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'Completed':
        return 'bg-green-900 text-green-300';
      case 'Pending':
        return 'bg-yellow-900 text-yellow-300';
      case 'Failed':
        return 'bg-red-900 text-red-300';
      default:
        return 'bg-gray-700 text-gray-300';
    }
  };

  return (
    <div className="bg-gray-800 rounded-lg overflow-hidden">
      <div className="p-4 border-b border-gray-700 flex justify-between items-center">
        <h3 className="text-white font-semibold">Transaction History</h3>
        <button 
          onClick={loadTransactions}
          className="text-gray-400 hover:text-white transition-colors"
        >
          <RefreshCw className="h-4 w-4" />
        </button>
      </div>
      
      {error ? (
        <div className="p-8 text-center">
          <p className="text-red-400">{error}</p>
          <button 
            onClick={loadTransactions}
            className="mt-2 text-sm underline hover:text-red-300"
          >
            Try again
          </button>
        </div>
      ) : transactions.length === 0 ? (
        <div className="p-8 text-center">
          <p className="text-gray-400">No transactions found</p>
        </div>
      ) : (
        <div className="overflow-x-auto">
          <table className="min-w-full divide-y divide-gray-700">
            <thead className="bg-gray-700">
              <tr>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Date</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Type</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Amount</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Status</th>
                <th className="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">Details</th>
              </tr>
            </thead>
            <tbody className="bg-gray-800 divide-y divide-gray-700">
              {transactions.map((tx) => (
                <tr key={tx.id} className="hover:bg-gray-750">
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-white">
                    {format(new Date(tx.date), 'MMM dd, yyyy')}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-white">
                    <div className="flex items-center">
                      {getTransactionIcon(tx.type)}
                      <span className="ml-2">{tx.type}</span>
                    </div>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-white">
                    {tx.amount ? `${tx.amount} ${tx.token}` : '-'}
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm">
                    <span className={`px-2 py-1 text-xs font-medium rounded-full ${getStatusColor(tx.status)}`}>
                      {tx.status}
                    </span>
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-blue-400">
                    <a href="#" className="hover:underline flex items-center">
                      View <ExternalLink className="h-3 w-3 ml-1" />
                    </a>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}
    </div>
  );
};

export default TransactionHistory;