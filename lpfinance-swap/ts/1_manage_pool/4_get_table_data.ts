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

const get_table_data = async () => {
    
  const creatorKeypair = getCreatorKeypair();

  const connection = new Connection(NETWORK, "confirmed");

  const provider = new SignerWallet(creatorKeypair).createProvider(connection);
  // console.log(provider)
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpfinanceSwap as Program<LpfinanceSwap>;
  
  try {
    const pool_pubkey = await getPublicKey("lpfi-usdc-pool");
    const poolAccount = await program.account.poolInfo.fetch(pool_pubkey);

    const total_lp_amount = Number(poolAccount.totalLpAmount);
    const feeRate = Number(poolAccount.fee) / 1000; // 0.5 % = return 0.005
    const lpTokenPrice = 2;
    const Liquidity =  lpTokenPrice * total_lp_amount;
    console.log("Liquidity: ", Liquidity);
  } catch(e) {
    console.log("failed");
    console.log(e);
  }


};

get_table_data();