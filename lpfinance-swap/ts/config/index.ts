import { Connection, Keypair, PublicKey } from "@solana/web3.js";

export const NETWORK = "https://api.devnet.solana.com";

export const PREFIX = "lpusd-auction";
export const CBS_PREFIX = "cbs-pda"
export const SOLEND_PREFIX = "solend0";
export const APRICOT_PREFIX = "apricot0";
export const LPSWAP_PREFIX = "lpfiswap0";

export const LpSOLMint = new PublicKey("5jmsfTrYxWSKgrZp4Y8cziTWvt7rqmTCiJ75FbLqFTVZ")
export const LpUSDMint = new PublicKey("3GB97goPSqywzcXybmVurYW7jSxRdGuS28nj74W8fAtL")
export const LpFIMint = new PublicKey("3x96fk94Pp4Jn2PWUexAXYN4eLK8TVYXHUippdYCHK1p")

export const wSOLMint = new PublicKey("6hPAQy93EbDzwHyU843zcWKATy8NrJ1ZsKCRi2JkuXcT");
export const MSOLMint = new PublicKey("AzRQUJPKxv8L9xfHPeGgKcsXXrjbYekW5mVvbMdw11Mp");
export const stsolMint = new PublicKey("3gb5MH7VF6o6mWbuBX7V8d1KtWX1pCSYMAwFa296rPuP");
export const scnSOLMint = new PublicKey("8eijEjgBCSk8vJcjwV1geZQp8tzvXTXgc7Xgg8qthKyJ");
export const USDCMint = new PublicKey("6ybV587PY2z6DX4Pf1tTh8oEhnuR6wwXLE8LHinKQKYV");
export const BtcMint = new PublicKey("4NAbav42C1BZdKASxuiKbzTFQKSqcZXG7ZZLDwfiZCGe");
export const ETHMint = new PublicKey("49ZEVDFHe18DDcyAe4fuRrhuf3DQpTDAAUodkaDsCcco");
export const RayMint = new PublicKey("CAtFbjnodtzt1mpxyJwPKfWP6MkTisckMk9KHUgSxX7v");
export const SRMMint = new PublicKey("2F988bKHUgPaw6mHwuPfdQhiRg1XtCJuDh4hrvVpT3wD");
export const AvaxMint = new PublicKey("FzUkBfKMr8YULR2cNiVHoUF9zH3rA5Zv99BzFohgqQxo");
export const fidaMint = new PublicKey("BdY3ZJSd66ADaoLVnCiZWLEX4XANxj8a9vXFBGedqtP6");
export const fttMint = new PublicKey("EZvZWjRHqHSf3ge1T13Y1GmTgW2oNWdsaeErxu8fDpBo");
export const ftmMint = new PublicKey("FtdjvSFvRHAVcebM2zxfyFJXfDGdGQL1pXtMnAd9AQRG");
export const gmtMint = new PublicKey("Hn2UGJ1jM9Tw9oidCJwLdhWpcczS4MrMdb48XvCDMmnP");
export const lunaMint = new PublicKey("8sLT5gE4YgcdDgnL6gxy2a9NZ79t46jQgrX87q7iqFPN");
export const maticMint = new PublicKey("6sxP334TsRHEznCMaUNKSzv8xmpTQZXY11fqszF5vYMJ");
export const USDTMint = new PublicKey("4ohBE15Y2L3rPF6T6TXcHwLv7Dtkd9hwHRMBS7UDaw3V");


export const PoolUSDC = new  PublicKey("26UWs5QHCgNPeLBpQvjQpZ9TmonKLJFmx21SHXBZ7w9V");
export const PoolRAY = new  PublicKey("5gS9zobDfjzdGwfM4mFubusq5Uqkr2WwyhrntXoigR9S");
export const PoolMSOL = new  PublicKey("APgMT9TK3Th1XcfHGAgGuyVZGyn12YHJD6HYPG8Zj5Tf");
export const PoolwSOL = new  PublicKey("BTSKPGQPERh3QjnaBfZmDKKwVjvv4jPYQDNVUJ2mik3d");
export const PoolSRM = new  PublicKey("BVrTba4EVWpeRkZ3JJUqDcfuP11bfzeTiC5jhczkdGym");
export const PoolSCNSOL = new  PublicKey("95QWqKXjFvcsfcpFh6Uo8pJ9JHcN5wJcPCgzLb7hVAeX");
export const PoolSTSOL = new  PublicKey("EUZJuR3Ws8VaCypcFHCeg9mHbrtmX9t6ZH4ERAh857MP");
export const PoolLpUSD = new  PublicKey("EigZFwsHC5uUiPtBn7xMD19uv4Rb8ajZQKZWpisagis4");
export const PoolLpSOL = new  PublicKey("E8JD5qTobkasrxLFyv8AZCw8aY1mP5AgzVfGedztCWm");
export const PoolLpFi = new  PublicKey("GnFDvH41YPW3DtQcaHEk4xZhG3Xeh9XXgwsC2EWHkoSb");

export const LpfinanceTokenPDA = new PublicKey("64iaARaRU9sXwLmAVy1a5NkYVM82GJ9Lvk2VfJ8PMChk");
export const LpfinanceTokenConfig = new PublicKey("3Lpjwy6tGj4XQVJBMcr8ESRpLDgdat3ozedQD5AjSf5a");
export const TestTokenConfig = new PublicKey("3Pguudq3L6AHwnSKaPVngwuu9JfFNM2x7sv5WeMSrsw8");

export const SolendConfig= new PublicKey("68SQXmcLmJzEUUm5MxudGZfJiPHsMEu3rQboTuNEabUT")
export const ApricotConfig= new PublicKey("1MA4Cp4wkLipThnYB1M6QpJ12mJvdH2ESbAvQv8bjYK")
export const SolendStateAccount= new PublicKey("76XJ35ToUi7ivAc9p62t8t4ukvE9BQPsNRSbUKiZmmuW")
export const ApricotStateAccount= new PublicKey("7bAetJoCfBiUmg8kZmQxUjnwCy7C7fPWUeYzAg1edFDM")

export const StableLpusdPool = new PublicKey("4Mcu9CJj8EPtsijxtwqo5kpPJSmzyeTZKE7Won5Q6iyL");
export const StableLpsolPool = new PublicKey("Hi9bZiEgdto5gHdMNfcZxHe1SZh63ifnD3HJEyWhcgKF");

export const solendPool = new PublicKey("AjU3jz8zc7vksB42VhvH3D1Rx5M5s2Bfh94WbJdYS79Y") // RAY ata
export const apricotPool = new PublicKey("3iHXjSSpVwg8rtUFDt6LNQRu8ttdYvBLnAj6cmW2zkqs") // RAY ata

export const LiquidityPool = new PublicKey("DcB2ZfvRU5ac9FgYF9doWYjHWtRXbStH7wDhciQcmF6v")

// ======> PYTH
export const pythRayAccount = new PublicKey("EhgAdTrgxi4ZoVZLQx1n93vULucPpiFi2BQtz9RJr1y6"); // 3m1y5h2uv7EQL3KaJZehvAJa4yDNvgc5yAdL9KPMKwvk
export const pythUsdcAccount = new PublicKey("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7"); // 6NpdXrQEpmDZ3jZKmM2rhdmkd3H6QAk23j2x8bkXcHKA
export const pythSolAccount = new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E
export const pythMsolAccount = new PublicKey("9a6RNx3tCu1TSs6TBSfV2XRXEPEZXQ6WB7jRojZRvyeZ"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E

export const pythSrmAccount = new PublicKey("992moaMQKs32GKZ9dxi8keyM2bUmbrwBZpK4p2K6X5Vs"); // 6NpdXrQEpmDZ3jZKmM2rhdmkd3H6QAk23j2x8bkXcHKA
export const pythScnsolAccount = new PublicKey("HoDAYYYhFvCNQNFPui51H8qvpcdz6KuVtq77ZGtHND2T"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E
export const pythStsolAccount = new PublicKey("2LwhbcswZekofMNRtDRMukZJNSRUiKYMFbqtBwqjDfke"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E
// ======> PYTH