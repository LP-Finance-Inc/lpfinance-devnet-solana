cp ./cbs-protocol/target/idl/cbs_protocol.json ./idls/cbs_protocol.json
cp ./apricot/target/idl/apricot.json ./idls/apricot.json
cp ./solend/target/idl/solend.json ./idls/solend.json
cp ./uniswap/target/idl/uniswap.json ./idls/uniswap.json
cp ./stable-swap/target/idl/stable_swap.json ./idls/stable_swap.json
cp ./swap-router/target/idl/swap_router.json ./idls/swap_router.json
cp ./lpfinance-tokens/target/idl/lpfinance_tokens.json ./idls/lpfinance_tokens.json
cp ./test-tokens/target/idl/test_tokens.json ./idls/test_tokens.json
cp ./lpusd-auction/target/idl/lpusd_auction.json ./idls/lpusd-auction.json

node ./program_helper/merge_keys.ts
