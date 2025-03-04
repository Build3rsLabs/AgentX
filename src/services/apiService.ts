import axios from 'axios';
import { Protocol, PoolData, Asset, Position } from '../types';
import { protocols as mockProtocols } from '../data/protocols';
import { pools as mockPools } from '../data/pools';
import { assets as mockAssets } from '../data/assets';
import { positions as mockPositions } from '../data/positions';
import { v4 as uuidv4 } from 'uuid';

// In a real application, this would be an environment variable
const API_BASE_URL = 'https://api.multiversx.com';

// Create axios instance with base configuration
const api = axios.create({
  baseURL: API_BASE_URL,
  headers: {
    'Content-Type': 'application/json',
  },
});

// Add request interceptor for authentication
api.interceptors.request.use(
  (config) => {
    // In a real app, you would get this from localStorage or a state management solution
    const token = localStorage.getItem('auth_token');
    if (token) {
      config.headers.Authorization = `Bearer ${token}`;
    }
    return config;
  },
  (error) => Promise.reject(error)
);

// API service with methods for each endpoint
class ApiService {
  // Authentication
  async login(address: string): Promise<{ token: string }> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.post('/auth/login', { address });
      // return response.data;
      
      // For demo purposes, we'll simulate a successful login
      const token = `demo_token_${uuidv4()}`;
      localStorage.setItem('auth_token', token);
      return { token };
    } catch (error) {
      console.error('Login error:', error);
      throw error;
    }
  }

  async logout(): Promise<void> {
    try {
      // In a real app, this would call the actual API
      // await api.post('/auth/logout');
      
      // For demo purposes, just remove the token
      localStorage.removeItem('auth_token');
    } catch (error) {
      console.error('Logout error:', error);
      throw error;
    }
  }

  // Check if user is authenticated
  isAuthenticated(): boolean {
    return !!localStorage.getItem('auth_token');
  }

  // Protocols
  async getProtocols(): Promise<Protocol[]> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get('/protocols');
      // return response.data;
      
      // For demo purposes, return mock data
      return mockProtocols;
    } catch (error) {
      console.error('Get protocols error:', error);
      throw error;
    }
  }

  async getProtocolById(id: string): Promise<Protocol | undefined> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get(`/protocols/${id}`);
      // return response.data;
      
      // For demo purposes, return mock data
      const protocol = mockProtocols.find(protocol => protocol.id === id);
      if (!protocol) {
        console.warn(`Protocol with ID ${id} not found`);
      }
      return protocol;
    } catch (error) {
      console.error(`Get protocol ${id} error:`, error);
      throw error;
    }
  }

  // Pools
  async getPools(protocolId?: string): Promise<PoolData[]> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get('/pools', { params: { protocolId } });
      // return response.data;
      
      // For demo purposes, return mock data
      if (protocolId) {
        const protocol = mockProtocols.find(p => p.id === protocolId);
        if (protocol) {
          return mockPools.filter(pool => pool.protocol === protocol.name);
        }
        console.warn(`No protocol found with ID ${protocolId} for filtering pools`);
        return [];
      }
      return mockPools;
    } catch (error) {
      console.error('Get pools error:', error);
      throw error;
    }
  }

  async getPoolById(id: string): Promise<PoolData | undefined> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get(`/pools/${id}`);
      // return response.data;
      
      // For demo purposes, return mock data
      const pool = mockPools.find(pool => pool.id === id);
      if (!pool) {
        console.warn(`Pool with ID ${id} not found`);
      }
      return pool;
    } catch (error) {
      console.error(`Get pool ${id} error:`, error);
      throw error;
    }
  }

  // Assets
  async getAssets(): Promise<Asset[]> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get('/assets');
      // return response.data;
      
      // For demo purposes, return mock data
      return mockAssets;
    } catch (error) {
      console.error('Get assets error:', error);
      throw error;
    }
  }

  async getAssetBySymbol(symbol: string): Promise<Asset | undefined> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get(`/assets/${symbol}`);
      // return response.data;
      
      // For demo purposes, return mock data
      const asset = mockAssets.find(asset => asset.symbol === symbol);
      if (!asset) {
        console.warn(`Asset with symbol ${symbol} not found`);
      }
      return asset;
    } catch (error) {
      console.error(`Get asset ${symbol} error:`, error);
      throw error;
    }
  }

  // User positions
  async getUserPositions(protocolId?: string): Promise<Position[]> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get('/positions', { params: { protocolId } });
      // return response.data;
      
      // For demo purposes, return mock data
      if (protocolId) {
        return mockPositions.filter(position => position.protocolId === protocolId);
      }
      return mockPositions;
    } catch (error) {
      console.error('Get user positions error:', error);
      throw error;
    }
  }

  async getPositionById(id: string): Promise<Position | undefined> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get(`/positions/${id}`);
      // return response.data;
      
      // For demo purposes, return mock data
      const position = mockPositions.find(position => position.id === id);
      if (!position) {
        console.warn(`Position with ID ${id} not found`);
      }
      return position;
    } catch (error) {
      console.error(`Get position ${id} error:`, error);
      throw error;
    }
  }

  async createPosition(position: Omit<Position, 'id'>): Promise<Position> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.post('/positions', position);
      // return response.data;
      
      // For demo purposes, create a new position with a generated ID
      const newPosition: Position = {
        ...position,
        id: `pos-${uuidv4()}`,
      };
      
      // In a real app, this would be handled by the backend
      // For demo, we'll just return the new position
      return newPosition;
    } catch (error) {
      console.error('Create position error:', error);
      throw error;
    }
  }

  async updatePosition(id: string, updates: Partial<Position>): Promise<Position> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.put(`/positions/${id}`, updates);
      // return response.data;
      
      // For demo purposes, find and update the position
      const position = mockPositions.find(p => p.id === id);
      if (!position) {
        throw new Error(`Position with ID ${id} not found`);
      }
      
      const updatedPosition: Position = {
        ...position,
        ...updates,
      };
      
      // In a real app, this would be handled by the backend
      // For demo, we'll just return the updated position
      return updatedPosition;
    } catch (error) {
      console.error(`Update position ${id} error:`, error);
      throw error;
    }
  }

  async deletePosition(id: string): Promise<void> {
    try {
      // In a real app, this would call the actual API
      // await api.delete(`/positions/${id}`);
      
      // For demo purposes, we'll just log the deletion
      console.log(`Position ${id} deleted`);
    } catch (error) {
      console.error(`Delete position ${id} error:`, error);
      throw error;
    }
  }

  // User wallet
  async getUserBalance(): Promise<{ egld: number; tokens: { symbol: string; amount: number }[] }> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get('/wallet/balance');
      // return response.data;
      
      // For demo purposes, return mock data
      return {
        egld: 12.45,
        tokens: [
          { symbol: 'MEX', amount: 25000 },
          { symbol: 'USDC', amount: 5000 },
          { symbol: 'USDT', amount: 3000 },
          { symbol: 'HTM', amount: 1500 },
        ],
      };
    } catch (error) {
      console.error('Get user balance error:', error);
      throw error;
    }
  }

  // Transactions
  async getTransactionHistory(): Promise<any[]> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get('/transactions');
      // return response.data;
      
      // For demo purposes, return mock data
      return [
        {
          id: 'tx1',
          date: '2025-03-01',
          type: 'Deposit',
          amount: 2.5,
          token: 'EGLD',
          status: 'Completed',
        },
        {
          id: 'tx2',
          date: '2025-02-28',
          type: 'Rebalance',
          amount: null,
          token: null,
          status: 'Completed',
        },
        {
          id: 'tx3',
          date: '2025-02-25',
          type: 'Deposit',
          amount: 5.0,
          token: 'EGLD',
          status: 'Completed',
        },
        {
          id: 'tx4',
          date: '2025-02-20',
          type: 'Withdraw',
          amount: 1.2,
          token: 'EGLD',
          status: 'Completed',
        },
        {
          id: 'tx5',
          date: '2025-02-15',
          type: 'Deposit',
          amount: 3.0,
          token: 'EGLD',
          status: 'Completed',
        },
      ];
    } catch (error) {
      console.error('Get transaction history error:', error);
      throw error;
    }
  }

  // User settings
  async getUserSettings(): Promise<any> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.get('/settings');
      // return response.data;
      
      // For demo purposes, return mock data
      return {
        rebalanceFrequency: 'weekly',
        gasPrice: 'medium',
        slippageProtection: true,
        notifications: {
          email: true,
          rebalance: true,
          yieldAlerts: true,
        },
      };
    } catch (error) {
      console.error('Get user settings error:', error);
      throw error;
    }
  }

  async updateUserSettings(settings: any): Promise<any> {
    try {
      // In a real app, this would call the actual API
      // const response = await api.put('/settings', settings);
      // return response.data;
      
      // For demo purposes, just return the updated settings
      return settings;
    } catch (error) {
      console.error('Update user settings error:', error);
      throw error;
    }
  }
}

export default new ApiService();