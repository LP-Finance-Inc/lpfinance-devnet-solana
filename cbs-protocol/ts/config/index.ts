import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import * as fs from "fs";

export { default as SolendIDL } from "../../../solend/target/idl/solend.json"
export { default as  ApricotIDL } from "../../../apricot/target/idl/apricot.json"
export { default as  LpfinanceTokenIDL } from "../../../lpfinance-tokens/target/idl/lpfinance_tokens.json";
export { default as SwapRouterIDL } from "../../../swap-router/target/idl/swap_router.json"
export { default as StableSwapIDL } from "../../../stable-swap/target/idl/stable_swap.json"

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
export const tokenStateAccount = new PublicKey("FEL9EygF1C3d5cwD2ZXkpmaQMBtdxKd1mvYRrD81KNVY");
export const StableLpusdPool = new PublicKey("B51GTPYfj8FvVLq71wStjAkkc4mSDgXbfDQgLygDpczc");
export const StableLpsolPool = new PublicKey("CVsmW8n6Wm8YfF6ssMgpvaURdFCY2je55WBnHTUjX7hz");

export const solendPool = new PublicKey("AjU3jz8zc7vksB42VhvH3D1Rx5M5s2Bfh94WbJdYS79Y") // RAY ata
export const apricotPool = new PublicKey("3iHXjSSpVwg8rtUFDt6LNQRu8ttdYvBLnAj6cmW2zkqs") // RAY ata

// TOKEN
export const USDCMint = new PublicKey("6ybV587PY2z6DX4Pf1tTh8oEhnuR6wwXLE8LHinKQKYV");

// SOLEND token account
export const solendRAYPool = new PublicKey("AjU3jz8zc7vksB42VhvH3D1Rx5M5s2Bfh94WbJdYS79Y")
export const solendwSOLPool = new PublicKey("BuTudvKNcgdUQ6r8zsiUVjviupbvP4X1obxFwzE797yK")
export const solendMSOLPool = new PublicKey("8yY8cKyP1sQNEBummXs2joVhijDSdzQwcBU2VcVG3z9w")
export const solendSRMPool = new PublicKey("3azEFYaMnyRBZ79uGpiEFvt4JHB3BjESTj99gAqKvi4z")
export const solendSCNSOLPool = new PublicKey("4aEuDicF4H4J8PWL3YGLVf9c38pbkfTKHQMRhiuzc4z2")
export const solendSTSOLPool = new PublicKey("3zLmj7hcrUwGdd6EQKeD5k3Lags8eG9jCwreAsjG7YwD")

// APRICOT token account
export const apricotRAYPool = new PublicKey("3iHXjSSpVwg8rtUFDt6LNQRu8ttdYvBLnAj6cmW2zkqs")
export const apricotwSOLPool = new PublicKey("98YTuYc4VeQELtoBCMic8BRfDgMi58BqZebXBNWkaDj7")
export const apricotMSOLPool = new PublicKey("EpV4kEfrkaVoKF2SDYbg7QCFxA9xhYsuJ2teupBbhJAp")
export const apricotSRMPool = new PublicKey("E6FnoyfaCcz3P5MHHVGdBCg9jRekMSNQyCH27pJLBRkB")
export const apricotSCNSOLPool = new PublicKey("ALu7e7r1XMngq6Tbam37PpczVjs3RAudkHFNNwLJciLH")
export const apricotSTSOLPool = new PublicKey("HxuyoKKX7nEzuJGdaU5jjkXFPYRZxeaT4Exnuobrb3bz")

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