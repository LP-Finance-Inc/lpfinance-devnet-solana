import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StableSwap } from "../target/types/stable_swap";
import { SignerWallet } from "@saberhq/solana-contrib";
import { 
    TOKEN_PROGRAM_ID, 
    ASSOCIATED_TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import {
  PublicKey,
  Connection,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";

import {
  getKeypair,
  getCreatorKeypair,
  getPublicKey,
  writePublicKey,
  getProgramId
} from "./utils";
import { NETWORK } from "./config";

const get_router = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");

  const authKeypair = getCreatorKeypair(); // getKeypair("creator");
  console.log("Creator address:", authKeypair.publicKey.toBase58());

  const provider = new SignerWallet(authKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.StableSwap as Program<StableSwap>;

  const stableswappoolAccounts = await program.account.stableswapPool.all();

  const len = stableswappoolAccounts.length;
  console.log("Pool amount : ", len);

  let list = []
  for (let i=0; i<len; i++){
    list.push({
        "pubkey" : stableswappoolAccounts[i].publicKey.toBase58(),
        "author": stableswappoolAccounts[i].account.author.toBase58().substring(0,10) + '...',
        "token_a": stableswappoolAccounts[i].account.tokenA.toBase58().substring(0,10) + '...',
        "token_b": stableswappoolAccounts[i].account.tokenB.toBase58().substring(0,10) + '...',
    })

    console.log(i)
    console.log('author : ', stableswappoolAccounts[i].account.author.toBase58());
    console.log('token_a : ', stableswappoolAccounts[i].account.tokenA.toBase58());
    console.log('amount_a : ', stableswappoolAccounts[i].account.amountA.toNumber());
    console.log('token_b : ', stableswappoolAccounts[i].account.tokenB.toBase58());
    console.log('amount_b : ', stableswappoolAccounts[i].account.amountB.toNumber());
    console.log('token_lp : ', stableswappoolAccounts[i].account.tokenLp.toBase58());
    console.log('total_amount_lp : ', stableswappoolAccounts[i].account.totalLpAmount.toNumber());
    console.log('amp : ', stableswappoolAccounts[i].account.amp.toNumber());
    console.log('fee : ', stableswappoolAccounts[i].account.fee);
  }

  console.table(list);  
};

get_router();