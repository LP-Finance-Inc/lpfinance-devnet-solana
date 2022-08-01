import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpfinanceSwap } from "../../target/types/lpfinance_swap";
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
    convert_to_wei,
  getCreatorKeypair,
  getPublicKey,
  writePublicKey
} from "../utils";
import { LpFIMint, NETWORK, PoolLpFi, PoolUSDC, USDCMint } from "../config";

const create_pool = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");

  const creatorKeypair = getCreatorKeypair(); // getKeypair("creator");

  const poolKeypair = anchor.web3.Keypair.generate();
  writePublicKey(poolKeypair.publicKey, `lpfi-usdc-pool`); 
  
  const token_mint_a = LpFIMint;
  const token_mint_b = USDCMint;

  const token_mint_lp = anchor.web3.Keypair.generate();

  console.log("Creator address:", creatorKeypair.publicKey.toBase58());

  const provider = new SignerWallet(creatorKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpfinanceSwap as Program<LpfinanceSwap>;

  const fee = 5;   // real fee = (fee / 10) %
  const min_lp = 0;

  const token_acc_lp_Keypair = anchor.web3.Keypair.generate();

  const amount_a = convert_to_wei(10000000) // 10000000 * 1e9 // 
  const amount_b = convert_to_wei(10000000) // 10000000 * 1e9// 

  await program.rpc.createPair( 
    new anchor.BN(amount_a), 
    new anchor.BN(amount_b), 
    new anchor.BN(min_lp),
    fee,
    {
        accounts: {
            liquidityPool: poolKeypair.publicKey,
            creator: creatorKeypair.publicKey,
            tokenaMint: token_mint_a,
            tokenbMint: token_mint_b,
            tokenLp: token_mint_lp.publicKey,
            tokenAccLp: token_acc_lp_Keypair.publicKey,
            tokenAccA: PoolLpFi,
            tokenAccB: PoolUSDC,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY  
        },
        signers: [
          poolKeypair, 
          token_mint_lp, 
          token_acc_lp_Keypair
        ]
  });

  console.log("1.Create new Pool");
  console.log("2.Create new LP Token of Creator");
  console.log("3.Create new LP TokenAccount of Creator");
  console.log("4.Mint LP Token to Creator");
  console.log("5.Change LP Token Owner: Creator -> Pool PDA");
  console.log("6.Change LP TokenAccount Owner: Creator -> Pool PDA");

  function sleep(milliseconds) {
    const date = Date.now();
    let currentDate = null;
    do {
      currentDate = Date.now();
    } while (currentDate - date < milliseconds);
  }
  sleep(1000);
  
  const poolAccount = await program.account.poolInfo.fetch(poolKeypair.publicKey);

  let list = [];
  list.push({ "Property" : "Pool", "Value" : poolKeypair.publicKey.toBase58() });
  list.push({ "Property" : "Creator", "Value" : poolAccount.creator.toBase58() });
  list.push({ "Property" : "A token", "Value" : poolAccount.tokenaMint.toBase58() });
  list.push({ "Property" : "B token", "Value" : poolAccount.tokenbMint.toBase58() });
  list.push({ "Property" : "LP token", "Value" : poolAccount.tokenLp.toBase58() });
  list.push({ "Property" : "A tokenAccount", "Value" : poolAccount.tokenAccA.toBase58() });
  list.push({ "Property" : "B tokenAccount", "Value" : poolAccount.tokenAccB.toBase58() });
  list.push({ "Property" : "LP tokenAccount", "Value" : poolAccount.tokenAccLp.toBase58() });
  list.push({ "Property" : "Amount A", "Value" : poolAccount.tokenaAmount.toString() });
  list.push({ "Property" : "Amount B", "Value" : poolAccount.tokenbAmount.toString() });
  list.push({ "Property" : "total LP amount", "Value" : poolAccount.totalLpAmount.toString() });
  list.push({ "Property" : "min LP amount", "Value" : poolAccount.minLpAmount.toString() });
  list.push({ "Property" : "fee", "Value" : poolAccount.fee });
  list.push({ "Property" : "State", "Value" : poolAccount.state });
  
  console.table(list);

};

create_pool();
