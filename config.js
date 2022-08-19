import { PublicKey } from "@solana/web3.js";

export { default as SolendIDL } from "../../../solend/target/idl/solend.json"
export { default as ApricotIDL } from "../../../apricot/target/idl/apricot.json"
export { default as LpfinanceTokenIDL } from "../../../lpfinance-tokens/target/idl/lpfinance_tokens.json";
export { default as CBSProtocolIDL } from "../../../cbs-protocol/target/idl/cbs_protocol.json";
export { default as SwapRouterIDL } from "../../../swap-router/target/idl/swap_router.json";
export { default as StableSwapIDL } from "../../../stable-swap/target/idl/stable_swap.json";
export { default as TestTokenIDL } from "../../../test-tokens/target/idl/test_tokens.json";
export { default as UniswapIDL } from "../../../uniswap/target/idl/uniswap.json";

export const NETWORK = "https://api.devnet.solana.com";

export const AUCTION_PREFIX = "lpusd-auction";
export const CBS_PREFIX = "cbs-pda"
export const SOLEND_PREFIX = "solend-pool";
export const APRICOT_PREFIX = "apricot-pool";
export const UNISWAP_PREFIX = "uniswap";
export const STABLESWAP_PREFIX = "stable-swap";
export const SWAPROUTER_PREFIX = "swap-escrow";


export const APRICOT_CONFIG = new PublicKey("HbA66JJa6TojT8dbW9WKHET68j7BvUFT5p6o4TvbEzy7"); 
export const AUCTION_CONFIG = new PublicKey("CCuZALUXjqXwx7bkUsei8T9rAkogaSvc99Sw3dB3rwyf"); 
export const CBS_CONFIG = new PublicKey("2vPf7XSASm8cRystdx9pbEfTkH5spa1o29yRdUvB9dXG"); 
export const LPFINANCE_TOKENS_CONFIG = new PublicKey("F8imm9aY2viQJufr2HFh6anTnfd2LHo1yULx6mpbQhtm"); 
export const SOLEND_CONFIG = new PublicKey("777w85N2QZg8zBng1aNq8FnjC2JLhbV6R6En5WgJkw4A"); 
export const TEST_TOKENS_CONFIG = new PublicKey("1j1o1d1V45a97kf9o3ZYfGMJFCXhdSNbDAf3S59TRn1"); 

export const APRICOT_PDA = new PublicKey("DdJsiBW9RSj6KhwN1qt8dRbAiTG3pAHjhYy9uZGDcsTu"); 
export const AUCTION_PDA = new PublicKey("6nqS5uUaXYUncgn81iEntgQafkQwMagSYZXUCQeqZHJd"); 
export const CBS_PDA = new PublicKey("9SYSA3RPEakev2i9GNDBLD5NGNELnVYKchWkWRrK1J6B"); 
export const LPFINANCE_TOKENS_PDA = new PublicKey("5vPTNsA4LQJUdouZ46t8m5iPE8Lhxo5PktQprT4TK7uU"); 
export const SOLEND_PDA = new PublicKey("6Eq8QRR2X8qwXSAvNSiMDDxSRRAeUToXkipyT25GWQPe"); 
export const SWAP_ROUTER_PDA = new PublicKey("4Y2vLmpLtfo5gxvGhAK68RtKruQdm8vEvAWJVLicQhmf"); 
export const TEST_TOKENS_PDA = new PublicKey("3kbA8tGp5ayZPbKVetchs633deYkwjnd95bPpuq8bypq"); 

export const LPFI_USDC_POOL = new PublicKey("2rp27TLQGkohZS2RcpVuX4s1kmktmoM7QLdUKgVMTom8"); 
export const LPSOL_WSOL_POOL = new PublicKey("6VBUBPA2Bev3dZTEJwfSVBJpCWv6sw9eoyywTS3cXmu3"); 
export const LPUSD_USDC_POOL = new PublicKey("BFteZ5EXKa4myspKtvKcD7DNkQaLFrwEVpvMDaGwbeTZ"); 

export const CBS_APRICOT_ACCOUNT = new PublicKey("2W6mfHgn6GfoYYeFpjEXLEAgZmKh7516A8GYditbkXvS"); 
export const CBS_SOLEND_ACCOUNT = new PublicKey("Aw5eo7kVx1x11AYuN3GwdjdvsfRstdYQPamFQcWtGvEH"); 

export const LpSOL_MINT = new PublicKey("G3dmoWruPPE8Utz4ZpU6rajmbfUoawxdanFemw13ihH5")
export const LpUSD_MINT = new PublicKey("BJao7J4inYLbd143jnkuF11dSiMGQunqUrmdGkfaxngu")
export const LpFI_MINT = new PublicKey("7ocGjpxGz3D77tSWA4TGpLN1brCnfp7REXNdEgZf6SCN")
 

export const WSOL_MINT = new PublicKey("3TufxjnyAiHH9cdMkdstCTMTjtH54p3Mnv9prDJ4eLTb")
export const MSOL_MINT = new PublicKey("9ct5yURTeKhET3wGqfThjYpftW1Z4QWR5C2U7Rh5fYGH")
export const STSOL_MINT = new PublicKey("DRXKnTLC9ypQ7NbaubgapdM2VTpfo3qNKddnKeRtw6Np")
export const SCNSOL_MINT = new PublicKey("DhgcE8JaU2nvysme1s51oBg55JRdpAGKyRgGqyX1bSzL")
export const USDC_MINT = new PublicKey("F79eXaUWMH6BXWKbtE6woxtvHpbT7EVPSxsFEwFh62py")
export const BTC_MINT = new PublicKey("8ZjLiFQ3j9a4H9FmAwPcFJsej6bNxCZCfcQkWxcf8DSg")
export const ETH_MINT = new PublicKey("DSq2PjVbBGvHFDGKxhUZJHEyudG3bNLhvMtJiX7ZnEh2")
export const RAY_MINT = new PublicKey("3oZHawbMxXLPKkphvsfprt5tbaTqcLxd9DB46Ptusi8C")
export const SRM_MINT = new PublicKey("BqKSUy8Q6V7GmWXWV4fNcdoaEgbt7bsmwzqTLAJiuBQa")
export const AVAX_MINT = new PublicKey("DK3Rk1m1kxwzCWgg2DkMz7QEvhXDo7JaoRjzuQPjTQhH")
export const FIDA_MINT = new PublicKey("FsvS6djyMBjGZCE5JToHKuwpXcipccXVPjatYEryHV4R")
export const FTT_MINT = new PublicKey("EHSoZqF5EsRRBrBo26etRKwRwuowp2a7nVXt3pun3yLs")
export const FTM_MINT = new PublicKey("BqQnbAqosF4AYJJPPZDQhFmRvQcS9rmsX3mkWVhTFJZ6")
export const GMT_MINT = new PublicKey("mhHXAsp2p67ucpxFLvNi1TqrauVR2wioWKG3RqEgt6p")
export const LUNA_MINT = new PublicKey("6YCDbh9im1jaJFRdqNnw2PfBpy6xHuL9dTczhT4C5R5G")
export const MATIC_MINT = new PublicKey("EdAGouCwpTJwwGHbXGVvfidBjWsSA7qGbAug3xtBZxsu")
export const USDT_MINT = new PublicKey("DubveCXcPuJNPdjwL4cRbAueQtwnDQHUXQNHw3YfQWJ5")
 

export const RAY_Apricot_ATA = new PublicKey("7CrChAbBcgyETrzAKBzWLXrsfPcc9uKvBLKLo7fF8t4t")
export const WSOL_Apricot_ATA = new PublicKey("4s8KkDLCceqK23qHVMQ3YGDDAemYK81uuq34hqHt7XVM")
export const MSOL_Apricot_ATA = new PublicKey("A2G3nS4TyPJQiqMvS7eMUcEnbB4W2dp4uAaUzDoec6hn")
export const SRM_Apricot_ATA = new PublicKey("6ch55C8j2QQMbLdvN7YjdWH7ffSMChr2eq3EhHhYFsF1")
export const SCNSOL_Apricot_ATA = new PublicKey("ATi33pTpoMnYBUamMhZD5AKyhS67yGXq3q3RtNzrXAV3")
export const STSOL_Apricot_ATA = new PublicKey("EnZRffpnJPbYGeeS87QioBLRb6AwVwJAGGJ8WSVWNtyw")
 

export const RAY_Solend_ATA = new PublicKey("CGYRG9HJ1oKXEBcaf8hBp3cvCpuAXqr3azcT3mKyUVzQ")
export const WSOL_Solend_ATA = new PublicKey("4nU1ycCrpQFGUVNUkPNH5uBrnYbkevBbF4bevYztZaSy")
export const MSOL_Solend_ATA = new PublicKey("HtY2TXZtXYVATnrKSU2XTVGxHPKZTwc6pS5PqbMd2nzT")
export const SRM_Solend_ATA = new PublicKey("GvANkMuafhMD2h6HsiEdF96XNyf8YKMHy4a9GbUTJE71")
export const SCNSOL_Solend_ATA = new PublicKey("57DLxxcFR1a2m7QaqgF7ARdh3j71SimzUVZGGkUR75uJ")
export const STSOL_Solend_ATA = new PublicKey("7joyah4n3P5tn2psL1XveGhJWPjdbPvWyM9QmKDVcWic")
 

export const LpSOL_SwapRouter_ATA = new PublicKey("4CxJohmwhWvNC6Sf2F3cWpo2PJfYiL21jYtuB1qfZsoV");
export const LpUSD_SwapRouter_ATA = new PublicKey("FnLniqHbT6Kx9S1ytyJceoFp8TrzaEpmsrwDXdcvuCML");
export const LpFI_SwapRouter_ATA = new PublicKey("97DHxQmaU8Fi8jcX2bkEBdiDCV6BKZNXADVKDvnK95Tv");
export const WSOL_SwapRouter_ATA = new PublicKey("EWBKUANgPLN3wLUBE6yoiAH8HS46EPcKYHuk3YuBcS9i");
export const MSOL_SwapRouter_ATA = new PublicKey("QJ6AnnwybfrSozsavsFBRvVkT5uehKoAwJzuknfr4fE");
export const STSOL_SwapRouter_ATA = new PublicKey("53i8zF7XfESfevMtrCYLDHGjHykgFtuXcay1SSWsnwwq");
export const SCNSOL_SwapRouter_ATA = new PublicKey("DrcCtd4iq2yPAHCnhQYrpW2Mk4CY39kYzALAnQbNd3G9");
export const USDC_SwapRouter_ATA = new PublicKey("9oEizmVh7LeJ79h4HsBAZqTmpHcfRTXLrrsL9ckgWM8X");
export const BTC_SwapRouter_ATA = new PublicKey("14AUoPMeCwuq8nCuKWNyNNWz4aofSNYXTTo1UJVe5qts");
export const ETH_SwapRouter_ATA = new PublicKey("CDPTPTWpe968EWWeu13pDowGuD6RKNAp8YnnHaQNHpGe");
export const RAY_SwapRouter_ATA = new PublicKey("FU2LoQSAVk21UME4qMUVz48AVqZfiGwz3EEUG3dubDtw");
export const SRM_SwapRouter_ATA = new PublicKey("CUbQGvWptgXPj8Ud4YQk2Hu61q8BCAKqWY5BL1canPGM");
export const AVAX_SwapRouter_ATA = new PublicKey("3bQ12X2vZsYMSvCzMjPvAj2cmpW7J3ySW9x9Ao8HTu9i");
export const FIDA_SwapRouter_ATA = new PublicKey("9QVpG7ZoEUaxtQsr1JshfkB9h1KwqTjvaMEUYiqAyjPf");
export const FTT_SwapRouter_ATA = new PublicKey("DdCPUX78VnTUfSntxKgmfHLfMFzzeNrCeEytqpwdSa3x");
export const FTM_SwapRouter_ATA = new PublicKey("2c4Aqx3SgATLynd3CYCCaq23CWYEozCYSuX6kbWn1KWg");
export const GMT_SwapRouter_ATA = new PublicKey("fJ2RpaBSz3r36C6WtHiPvRcn9EiLN8N3DUVevfDrb3C");
export const LUNA_SwapRouter_ATA = new PublicKey("74U2vVzcJT5yLBrwkwjuTwyCWK2UZap7ZYFEdfeQbKZK");
export const MATIC_SwapRouter_ATA = new PublicKey("E22SKgd7nnxvxweoTM8nNXRWp42s6YuoVh12vNVA27iB");
export const USDT_SwapRouter_ATA = new PublicKey("3Pnu8bcBhqWrcc7A4UEzQGDgvFSf737cJzw3aW6zT9nf");
 

export const LpSOL_AUCTION_ATA = new PublicKey("EERadkf396iVuGD7ykMPhGihUi2qBYZBu6es52H166uS");
export const LpUSD_AUCTION_ATA = new PublicKey("EbwMqfLxjGQ6bb7jf5Zm47bBhtzKcwB3G2znTSe2f9x3"); 

export const WSOL_AUCTION_ATA = new PublicKey("BTYsjyFQY9ezwEYAeYk7YggN43YyMHHQtPyCPVwihY4U");
export const USDC_AUCTION_ATA = new PublicKey("2hnW3RwomN1ddtBgknqfNcfhzUM611hJSL1r3nq7weno");
 

export const LpSOL_CBS_ATA = new PublicKey("E1wcBGcGjbdFe4LwiFd3W3VuCiobMx2fp3Yy5bvAV57n");
export const LpUSD_CBS_ATA = new PublicKey("5ZAFTKf5ceXZwE6oCizPShccBk4dya7yoBvHzTUpU2bv");
export const LpFI_CBS_ATA= new PublicKey("D24GvHsYJSrCcwgei8pP79zT6wp8AZ5eJEnNYCLd5WCB"); 

export const RAY_CBS_ATA = new PublicKey("7LkSLcLuYv8hksmst58k1vMLxtyvDHmise6j39gWjTyV");
export const WSOL_CBS_ATA = new PublicKey("BEfFVTBsqbMgvwtPX65io29AZ2YWkbH4kzVgyaWyHJJz");
export const MSOL_CBS_ATA = new PublicKey("GjJ9cEQWWqPEmp1oqrjkUYK3XpocXu4DZjSdiVYxAYed");
export const SRM_CBS_ATA = new PublicKey("5fV9NehjmcRKqvP8RKu4jxw377jca1RPG3RBx5iFkh3j");
export const SCNSOL_CBS_ATA = new PublicKey("4eUj2xpGtVYS29YTBShtcmjXJcqe2gghAiGg8wTyASH1");
export const STSOL_CBS_ATA = new PublicKey("2y3LRqcYqVQan95PREACdokvkFtqRNyuG8P4k7WS6Jtf");
export const USDC_CBS_ATA = new PublicKey("CTGUjzaPEhCJ8pSHmSM67KPYPHBQc3Pk1buathMxQWGa");
 
