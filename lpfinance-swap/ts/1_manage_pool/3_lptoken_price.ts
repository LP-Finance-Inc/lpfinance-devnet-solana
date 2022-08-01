import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpfinanceSwap } from "../../target/types/lpfinance_swap";
import { SignerWallet } from "@saberhq/solana-contrib";
import {
  Connection,
} from "@solana/web3.js";
import {
  getCreatorKeypair, getPublicKey,
} from "../utils";
import { NETWORK } from "../config";

const get_lptoken_price = async () => {
    
  const creatorKeypair = getCreatorKeypair();

  const connection = new Connection(NETWORK, "confirmed");

  const provider = new SignerWallet(creatorKeypair).createProvider(connection);
  // console.log(provider)
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpfinanceSwap as Program<LpfinanceSwap>;
  
  try {
    const pool_pubkey = await getPublicKey("lpfi-usdc-pool");
    const poolAccount = await program.account.poolInfo.fetch(pool_pubkey);

    const pool_amount_a = Number(poolAccount.tokenaAmount);
    const pool_amount_b = Number(poolAccount.tokenbAmount);
    const total_lp_amount = Number(poolAccount.totalLpAmount);

    const price_a = 1;
    const price_b = 1;
    const LpPrice = (pool_amount_a * price_a + pool_amount_b * price_b) / total_lp_amount;
    console.log("LpPrice: ", LpPrice);
  } catch(e) {
    console.log("failed");
    console.log(e);
  }


};

get_lptoken_price();