import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpfinanceSwap } from "../../target/types/lpfinance_swap";
import { SignerWallet } from "@saberhq/solana-contrib";
import BN = require("bn.js");
import { 
    TOKEN_PROGRAM_ID, 
    ASSOCIATED_TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import {
  PublicKey,
  Connection,
  TransactionInstruction,
  SystemProgram,
  SYSVAR_RENT_PUBKEY,
  Transaction,
  Keypair,
  Signer
} from "@solana/web3.js";

import {
  getPublicKey,
  writePublicKey,
  getCreatorKeypair,
  getATAPublicKey,
  convert_to_wei
} from "../utils";
import { LpFIMint, LPSWAP_PREFIX, NETWORK, USDCMint } from "../config";

async function findAssociatedTokenAddress(
  walletAddress: PublicKey,
  tokenMintAddress: PublicKey
): Promise<PublicKey> {
  return (await PublicKey.findProgramAddress(
      [
          walletAddress.toBuffer(),
          TOKEN_PROGRAM_ID.toBuffer(),
          tokenMintAddress.toBuffer(),
      ],
      ASSOCIATED_TOKEN_PROGRAM_ID
  ))[0];
}

const init_liquidity = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");
  const creatorKeypair = getCreatorKeypair(); 

  const provider = new SignerWallet(creatorKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpfinanceSwap as Program<LpfinanceSwap>;

  const pool_pubkey = await getPublicKey("lpfi-usdc-pool");


  let poolAccount = await program.account.poolInfo.fetch(pool_pubkey);

  const amount_a = poolAccount.tokenaAmount.toString() // 10000000 * 1e9 // 
  const amount_b = poolAccount.tokenbAmount.toString() // 10000000 * 1e9// 

  const creator_pubkey = poolAccount.creator;
  const token_acc_a = poolAccount.tokenAccA;
  const token_acc_b = poolAccount.tokenAccB;
  const token_mint_lp = poolAccount.tokenLp;
  const token_acc_lp = poolAccount.tokenAccLp;


  const ata_creator_a = await getATAPublicKey(poolAccount.tokenaMint, creatorKeypair.publicKey);
  console.log("creator_ata_a : ", ata_creator_a.toBase58());

  const ata_creator_b = await getATAPublicKey(poolAccount.tokenbMint, creatorKeypair.publicKey) // getPublicKey("ata_creator_b");
  console.log("creator_ata_b : ", ata_creator_b.toBase58());

  const ata_creator_lp = await findAssociatedTokenAddress(
    creator_pubkey,
    token_mint_lp
  );

  const PDA = await PublicKey.findProgramAddress(
    [Buffer.from(LPSWAP_PREFIX)],
    program.programId
  );
  
  console.log(PDA[0].toBase58());

  await program.rpc.initLiquidity( 
    new anchor.BN(amount_a), 
    new anchor.BN(amount_b), 
    {
        accounts: {
            pool: pool_pubkey,
            creator: creator_pubkey,
            creatorAccA: ata_creator_a,
            creatorAccB: ata_creator_b,
            tokenAccA: token_acc_a,
            tokenAccB: token_acc_b,
            tokenLp: token_mint_lp,
            ataCreatorLp: ata_creator_lp,
            tokenAccLp: token_acc_lp,
            poolPda: PDA[0],
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY  
        },
  });

  console.log("1.Transfer A Token: Creator -> Pool PDA");
  console.log("2.Transfer B Token: Creator -> Pool PDA");

  function sleep(milliseconds) {
    const date = Date.now();
    let currentDate = null;
    do {
      currentDate = Date.now();
    } while (currentDate - date < milliseconds);
  }
  sleep(1000);
  
  poolAccount = await program.account.poolInfo.fetch(pool_pubkey);

  let list = [];
  list.push({ "Property" : "Pool", "Value" : pool_pubkey.toBase58() });
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

init_liquidity();
