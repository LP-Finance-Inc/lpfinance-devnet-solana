import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Uniswap } from "../target/types/uniswap";
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

const get_uniswap_pool = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");

  const authKeypair = getCreatorKeypair(); // getKeypair("creator");
  console.log("Creator address:", authKeypair.publicKey.toBase58());

  const provider = new SignerWallet(authKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.Uniswap as Program<Uniswap>;

  const uniswappoolAccounts = await program.account.uniswapPool.all();

  const len = uniswappoolAccounts.length;
  console.log("Pool amount : ", len);

  let list = []
  for (let i=0; i<len; i++){
    list.push({
        "pubkey" : uniswappoolAccounts[i].publicKey.toBase58(),
        "author": uniswappoolAccounts[i].account.author.toBase58().substring(0,10) + '...',
        "token_a": uniswappoolAccounts[i].account.tokenA.toBase58().substring(0,10) + '...',
        "token_b": uniswappoolAccounts[i].account.tokenB.toBase58().substring(0,10) + '...',
    })

    console.log(i)
    console.log('author : ', uniswappoolAccounts[i].account.author.toBase58());
    console.log('token_a : ', uniswappoolAccounts[i].account.tokenA.toBase58());
    console.log('amount_a : ', uniswappoolAccounts[i].account.amountA.toNumber());
    console.log('token_b : ', uniswappoolAccounts[i].account.tokenB.toBase58());
    console.log('amount_b : ', uniswappoolAccounts[i].account.amountB.toNumber());
    console.log('token_lp : ', uniswappoolAccounts[i].account.tokenLp.toBase58());
    console.log('total_amount_lp : ', uniswappoolAccounts[i].account.totalLpAmount.toNumber());
    console.log('fee : ', uniswappoolAccounts[i].account.fee);
  }

  console.table(list);  
};

get_uniswap_pool();