import { PublicKey } from "@solana/web3.js";

export const NETWORK = "https://api.devnet.solana.com";
// export const NETWORK = "http://localhost:8899";
export { default as SolendIDL } from "../../solend/target/idl/solend.json"
export { default as ApricotIDL } from "../../apricot/target/idl/apricot.json"
export { default as LpfinanceTokenIDL } from "../../lpfinance-tokens/target/idl/lpfinance_tokens.json";
export { default as CBSProtocolIDL } from "../../cbs-protocol/target/idl/cbs_protocol.json";
export { default as SwapRouterIDL } from "../../swap-router/target/idl/swap_router.json";
export { default as StableSwapIDL } from "../../stable-swap/target/idl/stable_swap.json";
export { default as TestTokenIDL } from "../../test-tokens/target/idl/test_tokens.json";
export { default as UniswapIDL } from "../../uniswap/target/idl/uniswap.json";

export const tokenStateAccount = new PublicKey("3kbA8tGp5ayZPbKVetchs633deYkwjnd95bPpuq8bypq");
export const StableswapLpusdUsdc = new PublicKey("BFteZ5EXKa4myspKtvKcD7DNkQaLFrwEVpvMDaGwbeTZ")
export const StableswapLpsolWsol = new PublicKey("6VBUBPA2Bev3dZTEJwfSVBJpCWv6sw9eoyywTS3cXmu3")
export const UniswapLpfiUsdc = new PublicKey("C4rkcFbPi2E9jUcuLxfFakJQZKaRRuKgjnCdLSYWBSeq")

export const LpUSD = new PublicKey("3GB97goPSqywzcXybmVurYW7jSxRdGuS28nj74W8fAtL")
export const LpSOL = new PublicKey("5jmsfTrYxWSKgrZp4Y8cziTWvt7rqmTCiJ75FbLqFTVZ")
export const LpFI = new PublicKey("3x96fk94Pp4Jn2PWUexAXYN4eLK8TVYXHUippdYCHK1p");

export const WSOL = new PublicKey("3TufxjnyAiHH9cdMkdstCTMTjtH54p3Mnv9prDJ4eLTb")
export const WSOL_PYTH = new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix")

export const MSOL = new PublicKey("9ct5yURTeKhET3wGqfThjYpftW1Z4QWR5C2U7Rh5fYGH")
export const MSOL_PYTH = new PublicKey("9a6RNx3tCu1TSs6TBSfV2XRXEPEZXQ6WB7jRojZRvyeZ")

export const STSOL = new PublicKey("DRXKnTLC9ypQ7NbaubgapdM2VTpfo3qNKddnKeRtw6Np")
export const STSOL_PYTH = new PublicKey("2LwhbcswZekofMNRtDRMukZJNSRUiKYMFbqtBwqjDfke")

export const SCNSOL = new PublicKey("8BUAjLqr2UKMtUuVRo5M4JzZ5UXS6dpKE97SFxHcBSer")
export const SCNSOL_PYTH = new PublicKey("HoDAYYYhFvCNQNFPui51H8qvpcdz6KuVtq77ZGtHND2T")

export const USDC = new PublicKey("F79eXaUWMH6BXWKbtE6woxtvHpbT7EVPSxsFEwFh62py")
export const USDC_PYTH = new PublicKey("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7")

export const RAY = new PublicKey("Hzzzh4U29e2UoHsnVpLRCBRjgNtyK6D9sHc4BBzq1pdc")
export const RAY_PYTH = new PublicKey("EhgAdTrgxi4ZoVZLQx1n93vULucPpiFi2BQtz9RJr1y6")

export const SRM = new PublicKey("8iTWpvvTFjB4KviP5h9jojqzwEjHo5bEFHb4pJECXM97")
export const SRM_PYTH = new PublicKey("992moaMQKs32GKZ9dxi8keyM2bUmbrwBZpK4p2K6X5Vs")


export const LpSOLRouterPool = new PublicKey("4CxJohmwhWvNC6Sf2F3cWpo2PJfYiL21jYtuB1qfZsoV");
export const LpUSDRouterPool = new PublicKey("FnLniqHbT6Kx9S1ytyJceoFp8TrzaEpmsrwDXdcvuCML");
export const LpFIRouterPool = new PublicKey("97DHxQmaU8Fi8jcX2bkEBdiDCV6BKZNXADVKDvnK95Tv");
export const wSOLRouterPool = new PublicKey("EWBKUANgPLN3wLUBE6yoiAH8HS46EPcKYHuk3YuBcS9i");
export const MSOLRouterPool = new PublicKey("QJ6AnnwybfrSozsavsFBRvVkT5uehKoAwJzuknfr4fE");
export const stsolRouterPool = new PublicKey("53i8zF7XfESfevMtrCYLDHGjHykgFtuXcay1SSWsnwwq");
export const scnSOLRouterPool = new PublicKey("DrcCtd4iq2yPAHCnhQYrpW2Mk4CY39kYzALAnQbNd3G9");
export const USDCRouterPool = new PublicKey("9oEizmVh7LeJ79h4HsBAZqTmpHcfRTXLrrsL9ckgWM8X");
export const BtcRouterPool = new PublicKey("14AUoPMeCwuq8nCuKWNyNNWz4aofSNYXTTo1UJVe5qts");
export const ETHRouterPool = new PublicKey("CDPTPTWpe968EWWeu13pDowGuD6RKNAp8YnnHaQNHpGe");
export const RayRouterPool = new PublicKey("FU2LoQSAVk21UME4qMUVz48AVqZfiGwz3EEUG3dubDtw");
export const SRMRouterPool = new PublicKey("CUbQGvWptgXPj8Ud4YQk2Hu61q8BCAKqWY5BL1canPGM");
export const AvaxRouterPool = new PublicKey("3bQ12X2vZsYMSvCzMjPvAj2cmpW7J3ySW9x9Ao8HTu9i");
export const fidaRouterPool = new PublicKey("9QVpG7ZoEUaxtQsr1JshfkB9h1KwqTjvaMEUYiqAyjPf");
export const fttRouterPool = new PublicKey("DdCPUX78VnTUfSntxKgmfHLfMFzzeNrCeEytqpwdSa3x");
export const ftmRouterPool = new PublicKey("2c4Aqx3SgATLynd3CYCCaq23CWYEozCYSuX6kbWn1KWg");
export const gmtRouterPool = new PublicKey("fJ2RpaBSz3r36C6WtHiPvRcn9EiLN8N3DUVevfDrb3C");
export const lunaRouterPool = new PublicKey("74U2vVzcJT5yLBrwkwjuTwyCWK2UZap7ZYFEdfeQbKZK");
export const maticRouterPool = new PublicKey("E22SKgd7nnxvxweoTM8nNXRWp42s6YuoVh12vNVA27iB");
export const USDTRouterPool = new PublicKey("3Pnu8bcBhqWrcc7A4UEzQGDgvFSf737cJzw3aW6zT9nf");

