import { Connection, Keypair, PublicKey } from "@solana/web3.js";

export { default as SolendIDL } from "../../../solend/target/idl/solend.json"
export { default as  ApricotIDL } from "../../../apricot/target/idl/apricot.json"
export { default as  LpfinanceTokenIDL } from "../../../lpfinance-tokens/target/idl/lpfinance_tokens.json";
export { default as SwapRouterIDL } from "../../../swap-router/target/idl/swap_router.json"
export { default as StableSwapIDL } from "../../../stable-swap/target/idl/stable_swap.json"
export { default as AuctionIDL } from "../../../lpusd-auction/target/idl/lpusd_auction.json"
export { default as TestTokenIDL } from "../../../test-tokens/target/idl/test_tokens.json";
export { default as UniswapIDL } from "../../../uniswap/target/idl/uniswap.json";

export const NETWORK = "https://api.devnet.solana.com";

export const PREFIX = "cbs-pda";
export const SOLEND_PREFIX = "solend-pool";
export const APRICOT_PREFIX = "apricot-pool";

export const LpfinanceTokenPDA = new PublicKey("5vPTNsA4LQJUdouZ46t8m5iPE8Lhxo5PktQprT4TK7uU");
export const LpfinanceTokenConfig = new PublicKey("F8imm9aY2viQJufr2HFh6anTnfd2LHo1yULx6mpbQhtm");
export const TestTokenConfig = new PublicKey("1j1o1d1V45a97kf9o3ZYfGMJFCXhdSNbDAf3S59TRn1");

export const SolendConfig= new PublicKey("777w85N2QZg8zBng1aNq8FnjC2JLhbV6R6En5WgJkw4A")
export const ApricotConfig= new PublicKey("HbA66JJa6TojT8dbW9WKHET68j7BvUFT5p6o4TvbEzy7")
export const SolendStateAccount= new PublicKey("6Eq8QRR2X8qwXSAvNSiMDDxSRRAeUToXkipyT25GWQPe")
export const ApricotStateAccount= new PublicKey("DdJsiBW9RSj6KhwN1qt8dRbAiTG3pAHjhYy9uZGDcsTu")
export const tokenStateAccount = new PublicKey("3kbA8tGp5ayZPbKVetchs633deYkwjnd95bPpuq8bypq");
export const StableLpusdPool = new PublicKey("BFteZ5EXKa4myspKtvKcD7DNkQaLFrwEVpvMDaGwbeTZ");
export const StableLpsolPool = new PublicKey("6VBUBPA2Bev3dZTEJwfSVBJpCWv6sw9eoyywTS3cXmu3");

// TOKEN
export const wSOL_Mint = new PublicKey("3TufxjnyAiHH9cdMkdstCTMTjtH54p3Mnv9prDJ4eLTb")
export const MSOL_Mint = new PublicKey("9ct5yURTeKhET3wGqfThjYpftW1Z4QWR5C2U7Rh5fYGH")
export const stsol_Mint = new PublicKey("DRXKnTLC9ypQ7NbaubgapdM2VTpfo3qNKddnKeRtw6Np")
export const scnSOL_Mint = new PublicKey("DhgcE8JaU2nvysme1s51oBg55JRdpAGKyRgGqyX1bSzL")
export const USDC_Mint = new PublicKey("F79eXaUWMH6BXWKbtE6woxtvHpbT7EVPSxsFEwFh62py")
export const Btc_Mint = new PublicKey("8ZjLiFQ3j9a4H9FmAwPcFJsej6bNxCZCfcQkWxcf8DSg")
export const ETH_Mint = new PublicKey("DSq2PjVbBGvHFDGKxhUZJHEyudG3bNLhvMtJiX7ZnEh2")
export const Ray_Mint = new PublicKey("3oZHawbMxXLPKkphvsfprt5tbaTqcLxd9DB46Ptusi8C")
export const SRM_Mint = new PublicKey("BqKSUy8Q6V7GmWXWV4fNcdoaEgbt7bsmwzqTLAJiuBQa")
export const Avax_Mint = new PublicKey("DK3Rk1m1kxwzCWgg2DkMz7QEvhXDo7JaoRjzuQPjTQhH")
export const fida_Mint = new PublicKey("FsvS6djyMBjGZCE5JToHKuwpXcipccXVPjatYEryHV4R")
export const ftt_Mint = new PublicKey("EHSoZqF5EsRRBrBo26etRKwRwuowp2a7nVXt3pun3yLs")
export const ftm_Mint = new PublicKey("BqQnbAqosF4AYJJPPZDQhFmRvQcS9rmsX3mkWVhTFJZ6")
export const gmt_Mint = new PublicKey("mhHXAsp2p67ucpxFLvNi1TqrauVR2wioWKG3RqEgt6p")
export const luna_Mint = new PublicKey("6YCDbh9im1jaJFRdqNnw2PfBpy6xHuL9dTczhT4C5R5G")
export const matic_Mint = new PublicKey("EdAGouCwpTJwwGHbXGVvfidBjWsSA7qGbAug3xtBZxsu")
export const USDT_Mint = new PublicKey("DubveCXcPuJNPdjwL4cRbAueQtwnDQHUXQNHw3YfQWJ5")

export const EscrowUSDC = new PublicKey("CTGUjzaPEhCJ8pSHmSM67KPYPHBQc3Pk1buathMxQWGa");

// SOLEND token account
export const solendRAYPool = new PublicKey("CGYRG9HJ1oKXEBcaf8hBp3cvCpuAXqr3azcT3mKyUVzQ")
export const solendwSOLPool = new PublicKey("4nU1ycCrpQFGUVNUkPNH5uBrnYbkevBbF4bevYztZaSy")
export const solendMSOLPool = new PublicKey("HtY2TXZtXYVATnrKSU2XTVGxHPKZTwc6pS5PqbMd2nzT")
export const solendSRMPool = new PublicKey("GvANkMuafhMD2h6HsiEdF96XNyf8YKMHy4a9GbUTJE71")
export const solendSCNSOLPool = new PublicKey("57DLxxcFR1a2m7QaqgF7ARdh3j71SimzUVZGGkUR75uJ")
export const solendSTSOLPool = new PublicKey("7joyah4n3P5tn2psL1XveGhJWPjdbPvWyM9QmKDVcWic")

// APRICOT token account
export const apricotRAYPool = new PublicKey("7CrChAbBcgyETrzAKBzWLXrsfPcc9uKvBLKLo7fF8t4t")
export const apricotwSOLPool = new PublicKey("4s8KkDLCceqK23qHVMQ3YGDDAemYK81uuq34hqHt7XVM")
export const apricotMSOLPool = new PublicKey("A2G3nS4TyPJQiqMvS7eMUcEnbB4W2dp4uAaUzDoec6hn")
export const apricotSRMPool = new PublicKey("6ch55C8j2QQMbLdvN7YjdWH7ffSMChr2eq3EhHhYFsF1")
export const apricotSCNSOLPool = new PublicKey("ATi33pTpoMnYBUamMhZD5AKyhS67yGXq3q3RtNzrXAV3")
export const apricotSTSOLPool = new PublicKey("EnZRffpnJPbYGeeS87QioBLRb6AwVwJAGGJ8WSVWNtyw")

export const LiquidityPool = new PublicKey("2rp27TLQGkohZS2RcpVuX4s1kmktmoM7QLdUKgVMTom8")

// ======> PYTH
export const pythRayAccount = new PublicKey("EhgAdTrgxi4ZoVZLQx1n93vULucPpiFi2BQtz9RJr1y6"); // 3m1y5h2uv7EQL3KaJZehvAJa4yDNvgc5yAdL9KPMKwvk
export const pythUsdcAccount = new PublicKey("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7"); // 6NpdXrQEpmDZ3jZKmM2rhdmkd3H6QAk23j2x8bkXcHKA
export const pythSolAccount = new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E
export const pythMsolAccount = new PublicKey("9a6RNx3tCu1TSs6TBSfV2XRXEPEZXQ6WB7jRojZRvyeZ"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E

export const pythSrmAccount = new PublicKey("992moaMQKs32GKZ9dxi8keyM2bUmbrwBZpK4p2K6X5Vs"); // 6NpdXrQEpmDZ3jZKmM2rhdmkd3H6QAk23j2x8bkXcHKA
export const pythScnsolAccount = new PublicKey("HoDAYYYhFvCNQNFPui51H8qvpcdz6KuVtq77ZGtHND2T"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E
                                            
export const pythStsolAccount = new PublicKey("2LwhbcswZekofMNRtDRMukZJNSRUiKYMFbqtBwqjDfke"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E
// ======> PYTH