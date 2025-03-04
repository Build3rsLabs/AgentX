export interface Protocol {
  id: string;
  name: string;
  logo: string;
  description: string;
  tvl: number;
  apy: number;
  risk: 'Low' | 'Medium' | 'High';
  tokens: string[];
  url: string;
}

export interface Asset {
  symbol: string;
  name: string;
  logo: string;
  price: number;
  change24h: number;
}

export interface PoolData {
  id: string;
  protocol: string;
  name: string;
  tvl: number;
  apy: number;
  tokens: string[];
  risk: 'Low' | 'Medium' | 'High';
}

export interface ChatMessage {
  type: 'user' | 'agent';
  message: string;
  timestamp?: Date;
}

export interface Position {
  id: string;
  protocolId: string;
  name: string;
  type: string;
  tokens: string[];
  deposited: number;
  currentValue: number;
  apy: number;
  strategy: string;
  entryDate: string;
  lastRebalance: string;
  rebalanceFrequency: string;
  allocation: {
    token: string;
    percentage: number;
  }[];
}

export interface PerformanceData {
  date: string;
  value: number;
}