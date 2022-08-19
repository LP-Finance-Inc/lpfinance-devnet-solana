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