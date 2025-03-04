import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { Protocol, PoolData, Position } from '../types';
import apiService from '../services/apiService';

interface ProtocolContextType {
  protocols: Protocol[];
  selectedProtocol: Protocol | null;
  protocolPools: PoolData[];
  userPositions: Position[];
  isLoading: boolean;
  error: string | null;
  selectProtocol: (protocolId: string) => Promise<void>;
  refreshData: () => Promise<void>;
  createPosition: (position: Omit<Position, 'id'>) => Promise<Position>;
  updatePosition: (id: string, updates: Partial<Position>) => Promise<Position>;
  deletePosition: (id: string) => Promise<void>;
}

const ProtocolContext = createContext<ProtocolContextType | undefined>(undefined);

export const useProtocol = () => {
  const context = useContext(ProtocolContext);
  if (context === undefined) {
    throw new Error('useProtocol must be used within a ProtocolProvider');
  }
  return context;
};

interface ProtocolProviderProps {
  children: ReactNode;
}

export const ProtocolProvider: React.FC<ProtocolProviderProps> = ({ children }) => {
  const [protocols, setProtocols] = useState<Protocol[]>([]);
  const [selectedProtocol, setSelectedProtocol] = useState<Protocol | null>(null);
  const [protocolPools, setProtocolPools] = useState<PoolData[]>([]);
  const [userPositions, setUserPositions] = useState<Position[]>([]);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);

  // Load protocols on mount
  useEffect(() => {
    loadProtocols();
  }, []);

  const loadProtocols = async () => {
    try {
      const data = await apiService.getProtocols();
      setProtocols(data);
      
      // If no protocol is selected yet, select the first one
      if (data.length > 0 && !selectedProtocol) {
        await selectProtocol(data[0].id);
      }
    } catch (error) {
      console.error('Failed to load protocols:', error);
      setError('Failed to load protocols. Please try again later.');
    }
  };

  const selectProtocol = async (protocolId: string) => {
    setError(null);
    setIsLoading(true);
    try {
      const protocol = await apiService.getProtocolById(protocolId);
      if (!protocol) {
        throw new Error(`Protocol with ID ${protocolId} not found`);
      }
      
      setSelectedProtocol(protocol);
      
      // Load pools and positions for the selected protocol
      const [pools, positions] = await Promise.all([
        apiService.getPools(protocolId),
        apiService.getUserPositions(protocolId),
      ]);
      
      setProtocolPools(pools);
      setUserPositions(positions);
    } catch (error) {
      console.error(`Failed to select protocol ${protocolId}:`, error);
      setError(`Failed to load protocol data. Please try again later.`);
      
      // Reset selected protocol if there was an error
      setSelectedProtocol(null);
      setProtocolPools([]);
      setUserPositions([]);
    } finally {
      setIsLoading(false);
    }
  };

  const refreshData = async () => {
    if (selectedProtocol) {
      await selectProtocol(selectedProtocol.id);
    } else {
      await loadProtocols();
    }
  };

  const createPosition = async (position: Omit<Position, 'id'>): Promise<Position> => {
    try {
      const newPosition = await apiService.createPosition(position);
      setUserPositions(prev => [...prev, newPosition]);
      return newPosition;
    } catch (error) {
      console.error('Failed to create position:', error);
      throw error;
    }
  };

  const updatePosition = async (id: string, updates: Partial<Position>): Promise<Position> => {
    try {
      const updatedPosition = await apiService.updatePosition(id, updates);
      setUserPositions(prev => 
        prev.map(pos => pos.id === id ? updatedPosition : pos)
      );
      return updatedPosition;
    } catch (error) {
      console.error(`Failed to update position ${id}:`, error);
      throw error;
    }
  };

  const deletePosition = async (id: string): Promise<void> => {
    try {
      await apiService.deletePosition(id);
      setUserPositions(prev => prev.filter(pos => pos.id !== id));
    } catch (error) {
      console.error(`Failed to delete position ${id}:`, error);
      throw error;
    }
  };

  const value = {
    protocols,
    selectedProtocol,
    protocolPools,
    userPositions,
    isLoading,
    error,
    selectProtocol,
    refreshData,
    createPosition,
    updatePosition,
    deletePosition,
  };

  return <ProtocolContext.Provider value={value}>{children}</ProtocolContext.Provider>;
};