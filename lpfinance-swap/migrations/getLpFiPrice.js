// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");
const TOKEN = require("@solana/spl-token");
const { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token, getAssociatedTokenAddress } = require('@solana/spl-token')
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Connection } = anchor.web3;

const idl = require("../target/idl/lpfinance_swap.json");
const programID = idl.metadata.address;

// Test Token's MINT
const LpFiPoolKey = new PublicKey("DcB2ZfvRU5ac9FgYF9doWYjHWtRXbStH7wDhciQcmF6v"); 


module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  try {
    const LiquidatePoolData = await program.account.poolInfo.fetch(LpFiPoolKey);
    const rate = LiquidatePoolData.tokenbAmount / LiquidatePoolData.tokenaAmount;
    const LpFiPrice = rate * USDC_PRICE
  } catch (err) {
    console.log("Transaction error: ", err);
  }
}



const convert_to_wei = (val) => (parseFloat(val) * 1e9).toString();
// 2022-07-01 devnet
// ProgramID 87jyVePaEbZAYAcAjtGwQTcC4LU188KxJdynUzFWJKHA
// LiquidityPool: DcB2ZfvRU5ac9FgYF9doWYjHWtRXbStH7wDhciQcmF6v