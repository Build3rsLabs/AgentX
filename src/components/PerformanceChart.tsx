import React, { useState } from 'react';
import { format } from 'date-fns';
import { 
  LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, 
  ResponsiveContainer, Legend 
} from 'recharts';
import { PerformanceData } from '../types';

interface PerformanceChartProps {
  data: PerformanceData[];
  title: string;
}

const PerformanceChart: React.FC<PerformanceChartProps> = ({ data, title }) => {
  const [timeRange, setTimeRange] = useState<string>('all');
  
  const formatValue = (value: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      maximumFractionDigits: 0
    }).format(value);
  };

  const CustomTooltip = ({ active, payload, label }: any) => {
    if (active && payload && payload.length) {
      return (
        <div className="bg-gray-800 border border-gray-700 p-3 rounded-lg shadow-lg">
          <p className="text-gray-400 text-xs">{label}</p>
          <p className="text-white font-semibold">
            {formatValue(payload[0].value)}
          </p>
          
          {payload[0].payload.previousValue && (
            <>
              <div className="flex justify-between text-xs mt-1">
                <span className="text-gray-400">Change</span>
                <span className={payload[0].value > payload[0].payload.previousValue ? 'text-green-400' : 'text-red-400'}>
                  {((payload[0].value / payload[0].payload.previousValue - 1) * 100).toFixed(2)}%
                </span>
              </div>
            </>
          )}
        </div>
      );
    }
    return null;
  };
  
  // Add previous values to data for calculating changes
  const enhancedData = data.map((item, index) => ({
    ...item,
    previousValue: index > 0 ? data[index - 1].value : null
  }));
  
  // Filter data based on selected time range
  const filteredData = (() => {
    if (timeRange === 'all') return enhancedData;
    
    const now = new Date();
    let cutoffDate;
    
    switch (timeRange) {
      case '1m':
        cutoffDate = new Date(now.setMonth(now.getMonth() - 1));
        break;
      case '3m':
        cutoffDate = new Date(now.setMonth(now.getMonth() - 3));
        break;
      case '6m':
        cutoffDate = new Date(now.setMonth(now.getMonth() - 6));
        break;
      case '1y':
        cutoffDate = new Date(now.setFullYear(now.getFullYear() - 1));
        break;
      default:
        return enhancedData;
    }
    
    return enhancedData.filter(item => new Date(item.date) >= cutoffDate);
  })();
  
  // Calculate performance metrics
  const calculatePerformance = () => {
    if (filteredData.length < 2) return { change: 0, annualizedReturn: 0 };
    
    const firstValue = filteredData[0].value;
    const lastValue = filteredData[filteredData.length - 1].value;
    const totalChange = (lastValue / firstValue - 1) * 100;
    
    // Calculate time period in years
    const firstDate = new Date(filteredData[0].date);
    const lastDate = new Date(filteredData[filteredData.length - 1].date);
    const yearDiff = (lastDate.getTime() - firstDate.getTime()) / (1000 * 60 * 60 * 24 * 365);
    
    // Annualized return using CAGR formula
    const annualizedReturn = (Math.pow(lastValue / firstValue, 1 / yearDiff) - 1) * 100;
    
    return {
      change: totalChange,
      annualizedReturn
    };
  };
  
  const performance = calculatePerformance();

  return (
    <div className="bg-gray-800 rounded-lg p-6 mb-6">
      <div className="flex flex-col md:flex-row justify-between items-start md:items-center mb-6">
        <h2 className="text-xl font-semibold text-white mb-2 md:mb-0">{title}</h2>
        
        <div className="flex space-x-2">
          <button 
            className={`px-3 py-1 text-sm rounded-md ${timeRange === '1m' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}`}
            onClick={() => setTimeRange('1m')}
          >
            1M
          </button>
          <button 
            className={`px-3 py-1 text-sm rounded-md ${timeRange === '3m' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}`}
            onClick={() => setTimeRange('3m')}
          >
            3M
          </button>
          <button 
            className={`px-3 py-1 text-sm rounded-md ${timeRange === '6m' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}`}
            onClick={() => setTimeRange('6m')}
          >
            6M
          </button>
          <button 
            className={`px-3 py-1 text-sm rounded-md ${timeRange === '1y' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}`}
            onClick={() => setTimeRange('1y')}
          >
            1Y
          </button>
          <button 
            className={`px-3 py-1 text-sm rounded-md ${timeRange === 'all' ? 'bg-blue-600 text-white' : 'bg-gray-700 text-gray-300 hover:bg-gray-600'}`}
            onClick={() => setTimeRange('all')}
          >
            All
          </button>
        </div>
      </div>
      
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-6">
        <div className="bg-gray-700 p-4 rounded-lg">
          <p className="text-gray-400 text-sm mb-1">Total Return</p>
          <p className={`text-xl font-semibold ${performance.change >= 0 ? 'text-green-400' : 'text-red-400'}`}>
            {performance.change.toFixed(2)}%
          </p>
        </div>
        <div className="bg-gray-700 p-4 rounded-lg">
          <p className="text-gray-400 text-sm mb-1">Annualized Return</p>
          <p className={`text-xl font-semibold ${performance.annualizedReturn >= 0 ? 'text-green-400' : 'text-red-400'}`}>
            {performance.annualizedReturn.toFixed(2)}%
          </p>
        </div>
      </div>
      
      <div className="h-64">
        <ResponsiveContainer width="100%" height="100%">
          <LineChart
            data={filteredData}
            margin={{ top: 5, right: 5, left: 5, bottom: 5 }}
          >
            <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
            <XAxis 
              dataKey="date" 
              tick={{ fill: '#9CA3AF' }} 
              axisLine={{ stroke: '#4B5563' }}
              tickFormatter={(value) => format(new Date(value), 'MMM dd')}
            />
            <YAxis 
              tickFormatter={formatValue} 
              tick={{ fill: '#9CA3AF' }} 
              axisLine={{ stroke: '#4B5563' }}
              domain={['dataMin - 500', 'dataMax + 500']}
            />
            <Tooltip content={<CustomTooltip />} />
            <Line 
              type="monotone" 
              dataKey="value" 
              stroke="#3B82F6" 
              strokeWidth={2}
              dot={{ r: 3, fill: '#3B82F6', stroke: '#3B82F6' }}
              activeDot={{ r: 5, fill: '#3B82F6', stroke: '#fff' }}
            />
          </LineChart>
        </ResponsiveContainer>
      </div>
    </div>
  );
};

export default PerformanceChart;