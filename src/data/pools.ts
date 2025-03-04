import { PoolData } from '../types';

export const pools: PoolData[] = [
  {
    id: 'maiar-egld-mex',
    protocol: 'Maiar Exchange',
    name: 'EGLD-MEX LP',
    tvl: 42500000,
    apy: 18.5,
    tokens: ['EGLD', 'MEX'],
    risk: 'Medium'
  },
  {
    id: 'maiar-egld-usdc',
    protocol: 'Maiar Exchange',
    name: 'EGLD-USDC LP',
    tvl: 38700000,
    apy: 12.3,
    tokens: ['EGLD', 'USDC'],
    risk: 'Low'
  },
  {
    id: 'hatom-egld-lending',
    protocol: 'Hatom Protocol',
    name: 'EGLD Lending',
    tvl: 24600000,
    apy: 5.8,
    tokens: ['EGLD'],
    risk: 'Low'
  },
  {
    id: 'hatom-usdc-lending',
    protocol: 'Hatom Protocol',
    name: 'USDC Lending',
    tvl: 18900000,
    apy: 8.7,
    tokens: ['USDC'],
    risk: 'Low'
  },
  {
    id: 'ashswap-stable-pool',
    protocol: 'AshSwap',
    name: 'Stablecoin Pool',
    tvl: 32100000,
    apy: 9.2,
    tokens: ['USDC', 'USDT', 'BUSD'],
    risk: 'Low'
  },
  {
    id: 'xexchange-egld-xex',
    protocol: 'xExchange',
    name: 'EGLD-XEX Farm',
    tvl: 28500000,
    apy: 22.4,
    tokens: ['EGLD', 'XEX'],
    risk: 'High'
  },
  {
    id: 'onedex-egld-one',
    protocol: 'OneDex',
    name: 'EGLD-ONE LP',
    tvl: 15700000,
    apy: 14.8,
    tokens: ['EGLD', 'ONE'],
    risk: 'Medium'
  },
  {
    id: 'jexchange-jex-usdc',
    protocol: 'JEXchange',
    name: 'JEX-USDC Farm',
    tvl: 12300000,
    apy: 28.5,
    tokens: ['JEX', 'USDC'],
    risk: 'High'
  }
];