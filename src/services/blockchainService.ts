import axios from 'axios';
import { Position } from '../types';

class BlockchainService {
  private apiUrl: string;
  private token: string | null;
  private networkConfig: {
    chainId: string;
    gatewayUrl: string;
    explorerUrl: string;
  };

  constructor() {
    // API URL would typically come from environment variables in production
    this.apiUrl = import.meta.env.VITE_BLOCKCHAIN_API_URL || 'http://localhost:3030/api';
    this.token = localStorage.getItem('auth_token');
    
    // Network configuration
    this.networkConfig = {
      chainId: 'D', // Devnet
      gatewayUrl: 'https://devnet-gateway.multiversx.com',
      explorerUrl: 'https://devnet-explorer.multiversx.com',
    };
  }

  // Set authentication token
  setToken(token: string) {
    this.token = token;
    localStorage.setItem('auth_token', token);
  }

  // Clear authentication token
  clearToken() {
    this.token = null;
    localStorage.removeItem('auth_token');
  }

  // Get request headers
  private getHeaders() {
    return {
      'Content-Type': 'application/json',
      ...(this.token ? { Authorization: `Bearer ${this.token}` } : {}),
    };
  }

  // Get network configuration
  getNetworkConfig() {
    return this.networkConfig;
  }

  // Check blockchain connection status
  async checkConnection(): Promise<boolean> {
    try {
      const response = await axios.get(`${this.apiUrl}/health`);
      return response.data.status === 'ok';
    } catch (error) {
      console.error('Blockchain connection check failed:', error);
      return false;
    }
  }

  // Register a wallet address
  async registerWallet(address: string): Promise<any> {
    try {
      const response = await axios.post(
        `${this.apiUrl}/auth/register`,
        address,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error('Failed to register wallet:', error);
      throw error;
    }
  }

  // Authenticate with wallet signature
  async authenticate(address: string, signature: string): Promise<any> {
    try {
      const response = await axios.post(
        `${this.apiUrl}/auth/login`,
        { address, signature },
        { headers: this.getHeaders() }
      );
      
      if (response.data.token) {
        this.setToken(response.data.token);
      }
      
      return response.data;
    } catch (error) {
      console.error('Authentication failed:', error);
      throw error;
    }
  }

  // Get user profile
  async getUserProfile(): Promise<any> {
    try {
      const response = await axios.get(
        `${this.apiUrl}/users/me`,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error('Failed to get user profile:', error);
      throw error;
    }
  }

  // Get all protocols
  async getProtocols(): Promise<any> {
    try {
      const response = await axios.get(
        `${this.apiUrl}/protocols`,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error('Failed to get protocols:', error);
      throw error;
    }
  }

  // Get protocol by ID
  async getProtocolById(id: string): Promise<any> {
    try {
      const response = await axios.get(
        `${this.apiUrl}/protocols/${id}`,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error(`Failed to get protocol ${id}:`, error);
      throw error;
    }
  }

  // Get pools by protocol
  async getPoolsByProtocol(protocolId: string): Promise<any> {
    try {
      const response = await axios.get(
        `${this.apiUrl}/protocols/${protocolId}/pools`,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error(`Failed to get pools for protocol ${protocolId}:`, error);
      throw error;
    }
  }

  // Get all pools
  async getAllPools(): Promise<any> {
    try {
      const response = await axios.get(
        `${this.apiUrl}/pools`,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error('Failed to get all pools:', error);
      throw error;
    }
  }

  // Get user positions
  async getUserPositions(protocolId?: string): Promise<any> {
    try {
      const url = protocolId 
        ? `${this.apiUrl}/positions?protocol_id=${protocolId}`
        : `${this.apiUrl}/positions`;
        
      const response = await axios.get(
        url,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error('Failed to get user positions:', error);
      throw error;
    }
  }

  // Create a new position
  async createPosition(position: Omit<Position, 'id'>): Promise<any> {
    try {
      const response = await axios.post(
        `${this.apiUrl}/positions`,
        position,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error('Failed to create position:', error);
      throw error;
    }
  }

  // Update a position
  async updatePosition(id: string, updates: Partial<Position>): Promise<any> {
    try {
      const response = await axios.put(
        `${this.apiUrl}/positions/${id}`,
        updates,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error(`Failed to update position ${id}:`, error);
      throw error;
    }
  }

  // Delete a position
  async deletePosition(id: string): Promise<any> {
    try {
      const response = await axios.delete(
        `${this.apiUrl}/positions/${id}`,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error(`Failed to delete position ${id}:`, error);
      throw error;
    }
  }

  // Rebalance a position
  async rebalancePosition(id: string): Promise<any> {
    try {
      const response = await axios.post(
        `${this.apiUrl}/positions/${id}/rebalance`,
        {},
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error(`Failed to rebalance position ${id}:`, error);
      throw error;
    }
  }

  // Get user transactions
  async getUserTransactions(): Promise<any> {
    try {
      const response = await axios.get(
        `${this.apiUrl}/transactions`,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error('Failed to get user transactions:', error);
      throw error;
    }
  }

  // Create a new transaction
  async createTransaction(transaction: any): Promise<any> {
    try {
      const response = await axios.post(
        `${this.apiUrl}/transactions`,
        transaction,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error('Failed to create transaction:', error);
      throw error;
    }
  }

  // Get transaction status
  async getTransactionStatus(txHash: string): Promise<any> {
    try {
      const response = await axios.get(
        `${this.apiUrl}/transactions/hash/${txHash}`,
        { headers: this.getHeaders() }
      );
      return response.data;
    } catch (error) {
      console.error(`Failed to get transaction status for ${txHash}:`, error);
      throw error;
    }
  }

  // Get transaction explorer URL
  getTransactionExplorerUrl(txHash: string): string {
    return `${this.networkConfig.explorerUrl}/transactions/${txHash}`;
  }

  // Get address explorer URL
  getAddressExplorerUrl(address: string): string {
    return `${this.networkConfig.explorerUrl}/accounts/${address}`;
  }
}

export default new BlockchainService();