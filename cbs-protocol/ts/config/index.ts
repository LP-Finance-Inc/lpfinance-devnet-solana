import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as fs from "fs";

export { default as SolendIDL } from "../../../solend/target/idl/solend.json"
export { default as  ApricotIDL } from "../../../apricot/target/idl/apricot.json"
export { default as  LpfinanceTokenIDL } from "../../../lpfinance-tokens/target/idl/lpfinance_tokens.json";

export const NETWORK = "https://api.devnet.solana.com";

export const PREFIX = "cbs-pda";
export const SOLEND_PREFIX = "solend0";
export const APRICOT_PREFIX = "apricot0";


export const LpfinanceTokenPDA = new PublicKey("64iaARaRU9sXwLmAVy1a5NkYVM82GJ9Lvk2VfJ8PMChk");
export const LpfinanceTokenConfig = new PublicKey("3Lpjwy6tGj4XQVJBMcr8ESRpLDgdat3ozedQD5AjSf5a");
export const TestTokenConfig = new PublicKey("3Pguudq3L6AHwnSKaPVngwuu9JfFNM2x7sv5WeMSrsw8");

export const SolendConfig= new PublicKey("68SQXmcLmJzEUUm5MxudGZfJiPHsMEu3rQboTuNEabUT")
export const ApricotConfig= new PublicKey("1MA4Cp4wkLipThnYB1M6QpJ12mJvdH2ESbAvQv8bjYK")
export const SolendStateAccount= new PublicKey("76XJ35ToUi7ivAc9p62t8t4ukvE9BQPsNRSbUKiZmmuW")
export const ApricotStateAccount= new PublicKey("7bAetJoCfBiUmg8kZmQxUjnwCy7C7fPWUeYzAg1edFDM")

export const StableLpusdPool = new PublicKey("B51GTPYfj8FvVLq71wStjAkkc4mSDgXbfDQgLygDpczc");
export const StableLpsolPool = new PublicKey("CVsmW8n6Wm8YfF6ssMgpvaURdFCY2je55WBnHTUjX7hz");

export const solendPool = new PublicKey("AjU3jz8zc7vksB42VhvH3D1Rx5M5s2Bfh94WbJdYS79Y") // RAY ata
export const apricotPool = new PublicKey("3iHXjSSpVwg8rtUFDt6LNQRu8ttdYvBLnAj6cmW2zkqs") // RAY ata

export const LiquidityPool = new PublicKey("C4rkcFbPi2E9jUcuLxfFakJQZKaRRuKgjnCdLSYWBSeq")

// ======> PYTH
export const pythRayAccount = new PublicKey("EhgAdTrgxi4ZoVZLQx1n93vULucPpiFi2BQtz9RJr1y6"); // 3m1y5h2uv7EQL3KaJZehvAJa4yDNvgc5yAdL9KPMKwvk
export const pythUsdcAccount = new PublicKey("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7"); // 6NpdXrQEpmDZ3jZKmM2rhdmkd3H6QAk23j2x8bkXcHKA
export const pythSolAccount = new PublicKey("J83w4HKfqxwcq3BEMMkPFSppX3gqekLyLJBexebFVkix"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E
export const pythMsolAccount = new PublicKey("9a6RNx3tCu1TSs6TBSfV2XRXEPEZXQ6WB7jRojZRvyeZ"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E

export const pythSrmAccount = new PublicKey("992moaMQKs32GKZ9dxi8keyM2bUmbrwBZpK4p2K6X5Vs"); // 6NpdXrQEpmDZ3jZKmM2rhdmkd3H6QAk23j2x8bkXcHKA
export const pythScnsolAccount = new PublicKey("HoDAYYYhFvCNQNFPui51H8qvpcdz6KuVtq77ZGtHND2T"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E
                                            
export const pythStsolAccount = new PublicKey("2LwhbcswZekofMNRtDRMukZJNSRUiKYMFbqtBwqjDfke"); // 3Mnn2fX6rQyUsyELYms1sBJyChWofzSNRoqYzvgMVz5E
// ======> PYTH