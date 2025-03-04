import { Protocol } from '../types';

export const protocols: Protocol[] = [
  {
    id: 'maiar-exchange',
    name: 'Maiar Exchange',
    logo: 'https://maiar.exchange/favicon.ico',
    description: 'The leading DEX on MultiversX, offering swaps and liquidity pools with attractive yields.',
    tvl: 124500000,
    apy: 12.5,
    risk: 'Low',
    tokens: ['EGLD', 'MEX', 'USDC'],
    url: 'https://maiar.exchange'
  },
  {
    id: 'hatom',
    name: 'Hatom Protocol',
    logo: 'https://hatom.com/favicon.ico',
    description: 'Lending and borrowing protocol on MultiversX with competitive interest rates.',
    tvl: 78300000,
    apy: 8.2,
    risk: 'Medium',
    tokens: ['EGLD', 'USDC', 'USDT', 'HTM'],
    url: 'https://hatom.com'
  },
  {
    id: 'ashswap',
    name: 'AshSwap',
    logo: 'https://app.ashswap.io/favicon.ico',
    description: 'Stable swap AMM protocol focused on capital efficiency and minimal slippage.',
    tvl: 45600000,
    apy: 9.7,
    risk: 'Medium',
    tokens: ['USDC', 'USDT', 'BUSD', 'ASH'],
    url: 'https://app.ashswap.io'
  },
  {
    id: 'xexchange',
    name: 'xExchange',
    logo: 'https://xexchange.com/favicon.ico',
    description: 'Decentralized exchange with farming opportunities and governance.',
    tvl: 92100000,
    apy: 14.3,
    risk: 'Medium',
    tokens: ['EGLD', 'XEX', 'USDC'],
    url: 'https://xexchange.com'
  },
  {
    id: 'onedex',
    name: 'OneDex',
    logo: 'https://app.onedex.app/favicon.ico',
    description: 'Aggregator DEX providing the best rates across MultiversX.',
    tvl: 31500000,
    apy: 7.8,
    risk: 'Low',
    tokens: ['EGLD', 'ONE', 'USDC'],
    url: 'https://app.onedex.app'
  },
  {
    id: 'jexchange',
    name: 'JEXchange',
    logo: 'https://jexchange.io/favicon.ico',
    description: 'Decentralized exchange with focus on community governance and yield farming.',
    tvl: 28700000,
    apy: 16.5,
    risk: 'High',
    tokens: ['EGLD', 'JEX', 'USDC'],
    url: 'https://jexchange.io'
  }
];