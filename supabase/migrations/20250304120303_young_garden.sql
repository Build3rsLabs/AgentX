OneDex', 'https://app.onedex.app/favicon.ico', 'Aggregator DEX providing the best rates across MultiversX.', 31500000, 7.8, 'Low', ARRAY['EGLD', 'ONE', 'USDC'], 'https://app.onedex.app', 'erd1qqqqqqqqqqqqqpgqrc4pg2xarca9z34njcxeur622qmfjp8jjlwqhxj4k8', true),
    ('jexchange', 'JEXchange', 'https://jexchange.io/favicon.ico', 'Decentralized exchange with focus on community governance and yield farming.', 28700000, 16.5, 'High', ARRAY['EGLD', 'JEX', 'USDC'], 'https://jexchange.io', 'erd1qqqqqqqqqqqqqpgq0lzzvr8c494a5uuf8fy2sdnhehfl4gk2jlwq6ht6xz', true);

-- Seed pools
INSERT INTO pools (id, protocol_id, name, tvl, apy, tokens, risk, contract_address, is_active)
VALUES
    ('maiar-egld-mex', 'maiar-exchange', 'EGLD-MEX LP', 42500000, 18.5, ARRAY['EGLD', 'MEX'], 'Medium', 'erd1qqqqqqqqqqqqqpgqd9rvv2n378e27jcts8vfwynpwm0qrjl9zzgq4y5xrh', true),
    ('maiar-egld-usdc', 'maiar-exchange', 'EGLD-USDC LP', 38700000, 12.3, ARRAY['EGLD', 'USDC'], 'Low', 'erd1qqqqqqqqqqqqqpgqmuk0q2saj0mgutxm4teywre5v5j4gk2kzzgqmj7eqy', true),
    ('hatom-egld-lending', 'hatom', 'EGLD Lending', 24600000, 5.8, ARRAY['EGLD'], 'Low', 'erd1qqqqqqqqqqqqqpgqhe8t5jewfqgz7pz5rz5dpwgkldpvmg380n4s2fgxau', true),
    ('hatom-usdc-lending', 'hatom', 'USDC Lending', 18900000, 8.7, ARRAY['USDC'], 'Low', 'erd1qqqqqqqqqqqqqpgq0lzzvr8c494a5uuf8fy2sdnhehfl4gk2jlwq6ht6xz', true),
    ('ashswap-stable-pool', 'ashswap', 'Stablecoin Pool', 32100000, 9.2, ARRAY['USDC', 'USDT', 'BUSD'], 'Low', 'erd1qqqqqqqqqqqqqpgqvc7gdl0p4s97guh498wgz75k8sav6sjfjlwqh679jy', true),
    ('xexchange-egld-xex', 'xexchange', 'EGLD-XEX Farm', 28500000, 22.4, ARRAY['EGLD', 'XEX'], 'High', 'erd1qqqqqqqqqqqqqpgqmuk0q2saj0mgutxm4teywre5v5j4gk2kzzgqmj7eqy', true),
    ('onedex-egld-one', 'onedex', 'EGLD-ONE LP', 15700000, 14.8, ARRAY['EGLD', 'ONE'], 'Medium', 'erd1qqqqqqqqqqqqqpgqd77fnev2sthnczp2lnfx0y5jdycynjfhzzgq6p3rax', true),
    ('jexchange-jex-usdc', 'jexchange', 'JEX-USDC Farm', 12300000, 28.5, ARRAY['JEX', 'USDC'], 'High', 'erd1qqqqqqqqqqqqqpgq5774jcntdqkzv62tlvvhfn2y7eevpty6rchsq7k4hp', true);