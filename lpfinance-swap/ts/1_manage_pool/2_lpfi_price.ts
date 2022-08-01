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
import { LpFIMint, NETWORK } from "../config";

const get_lpfi_price = async () => {
    
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

    const tokenaMint = poolAccount.tokenaMint;
    const tokenbMint = poolAccount.tokenbMint;
    const lpfiMint = LpFIMint;

    const USDC_PRICE = 1;
    let LpFI_PRICE;
    if (lpfiMint.toBase58().toLowerCase() === tokenaMint.toBase58().toLocaleLowerCase()) {
        LpFI_PRICE = pool_amount_b / pool_amount_a * USDC_PRICE;
    } else if (lpfiMint.toBase58().toLowerCase() === tokenbMint.toBase58().toLocaleLowerCase()) {
        LpFI_PRICE = pool_amount_a / pool_amount_b * USDC_PRICE;
    } else {
        console.log("invalid token");
        LpFI_PRICE= 0;
    }
    console.log("LpPrice: ", LpFI_PRICE);
  } catch(e) {
    console.log("failed");
    console.log(e);
  }


};

get_lpfi_price();