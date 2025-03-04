import { Position } from '../types';

export const positions: Position[] = [
  {
    id: 'pos-1',
    protocolId: 'maiar-exchange',
    name: 'EGLD-MEX Liquidity',
    type: 'Liquidity Pool',
    tokens: ['EGLD', 'MEX'],
    deposited: 2500,
    currentValue: 2680,
    apy: 18.5,
    strategy: 'Balanced',
    entryDate: '2025-02-15',
    lastRebalance: '2025-03-01',
    rebalanceFrequency: 'Weekly',
    allocation: [
      { token: 'EGLD', percentage: 50 },
      { token: 'MEX', percentage: 50 }
    ]
  },
  {
    id: 'pos-2',
    protocolId: 'hatom',
    name: 'EGLD Lending',
    type: 'Lending',
    tokens: ['EGLD'],
    deposited: 1800,
    currentValue: 1845,
    apy: 5.8,
    strategy: 'Conservative',
    entryDate: '2025-01-20',
    lastRebalance: '2025-03-01',
    rebalanceFrequency: 'Weekly',
    allocation: [
      { token: 'EGLD', percentage: 100 }
    ]
  },
  {
    id: 'pos-3',
    protocolId: 'ashswap',
    name: 'Stablecoin Pool',
    type: 'Stable Swap',
    tokens: ['USDC', 'USDT', 'BUSD'],
    deposited: 5000,
    currentValue: 5115,
    apy: 9.2,
    strategy: 'Conservative',
    entryDate: '2025-02-01',
    lastRebalance: '2025-03-01',
    rebalanceFrequency: 'Weekly',
    allocation: [
      { token: 'USDC', percentage: 33 },
      { token: 'USDT', percentage: 33 },
      { token: 'BUSD', percentage: 34 }
    ]
  },
  {
    id: 'pos-4',
    protocolId: 'xexchange',
    name: 'EGLD-XEX Farm',
    type: 'Yield Farm',
    tokens: ['EGLD', 'XEX'],
    deposited: 3200,
    currentValue: 3580,
    apy: 22.4,
    strategy: 'Aggressive',
    entryDate: '2025-02-10',
    lastRebalance: '2025-03-01',
    rebalanceFrequency: 'Weekly',
    allocation: [
      { token: 'EGLD', percentage: 50 },
      { token: 'XEX', percentage: 50 }
    ]
  }
];