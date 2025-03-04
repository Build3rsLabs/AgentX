-- Seed protocols
INSERT INTO protocols (id, name, logo_url, description, tvl, apy, risk, tokens, website_url, contract_address, is_active)
VALUES
    ('maiar-exchange', 'Maiar Exchange', 'https://maiar.exchange/favicon.ico', 'The leading DEX on MultiversX, offering swaps and liquidity pools with attractive yields.', 124500000, 12.5, 'Low', ARRAY['EGLD', 'MEX', 'USDC'], 'https://maiar.exchange', 'erd1qqqqqqqqqqqqqpgqd77fnev2sthnczp2lnfx0y5jdycynjfhzzgq6p3rax', true),
    ('hatom', 'Hatom Protocol', 'https://hatom.com/favicon.ico', 'Lending and borrowing protocol on MultiversX with competitive interest rates.', 78300000, 8.2, 'Medium', ARRAY['EGLD', 'USDC', 'USDT', 'HTM'], 'https://hatom.com', 'erd1qqqqqqqqqqqqqpgq7ykazrzd905zvnlr88dpfw06677lxe9w0n4suz00uh', true),
    ('ashswap', 'AshSwap', 'https://app.ashswap.io/favicon.ico', 'Stable swap AMM protocol focused on capital efficiency and minimal slippage.', 45600000, 9.7, 'Medium', ARRAY['USDC', 'USDT', 'BUSD', 'ASH'], 'https://app.ashswap.io', 'erd1qqqqqqqqqqqqqpgq5774jcntdqkzv62tlvvhfn2y7eevpty6rchsq7k4hp', true),
    ('xexchange', 'xExchange', 'https://xexchange.com/favicon.ico', 'Decentralized exchange with farming opportunities and governance.', 92100000, 14.3, 'Medium', ARRAY['EGLD', 'XEX', 'USDC'], 'https://xexchange.com', 'erd1qqqqqqqqqqqqqpgqvc7gdl0p4s97guh498wgz75k8sav6sjfjlwqh679jy', true),
    ('onedex', '