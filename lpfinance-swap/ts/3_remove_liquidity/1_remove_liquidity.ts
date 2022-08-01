import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpfinanceSwap } from "../../target/types/lpfinance_swap";
import { createMemoInstruction, SignerWallet, suppressConsoleErrorAsync } from "@saberhq/solana-contrib";
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
    getATAPublicKey,
    getCreatorKeypair,
    getPublicKey,
} from "../utils";
import { LPSWAP_PREFIX, NETWORK } from "../config";

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

const remove_liquidity = async () => {
    
  // const connection = new Connection("http://localhost:8899", "confirmed");
  const connection = new Connection(NETWORK, "confirmed");

  const userKeypair = getCreatorKeypair();

  const provider = new SignerWallet(userKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpfinanceSwap as Program<LpfinanceSwap>;

  const pool_pubkey = await getPublicKey(`lpfi-usdc-pool`);
  console.log("pool pubkey : ", pool_pubkey.toBase58());

  let poolAccount = await program.account.poolInfo.fetch(pool_pubkey);

  const ata_user_a = await getATAPublicKey(poolAccount.tokenaMint, userKeypair.publicKey)
  console.log("user_ata_a : ", ata_user_a.toBase58());

  const ata_user_b = await getATAPublicKey(poolAccount.tokenbMint, userKeypair.publicKey)
  console.log("user_ata_b : ", ata_user_b.toBase58());

  const amount_lp = "100000000000";

  const token_acc_a = poolAccount.tokenAccA;
  const token_acc_b = poolAccount.tokenAccB;
  const token_acc_lp = poolAccount.tokenAccLp;
  const token_lp = poolAccount.tokenLp;

  const ata_user_lp = await findAssociatedTokenAddress(
    userKeypair.publicKey,
    token_lp
  );
  console.log('ata User LP:', ata_user_lp.toBase58());

  const swapProgramId = program.programId;
  const PDA = await PublicKey.findProgramAddress(
    [Buffer.from(LPSWAP_PREFIX)],
    swapProgramId
  );

  await program.rpc.removeLiquidity( 
    new anchor.BN(amount_lp),
    {
        accounts: {
            pool: pool_pubkey,
            remover: userKeypair.publicKey,
            removerAccA: ata_user_a,
            removerAccB: ata_user_b,
            tokenAccA: token_acc_a,
            tokenAccB: token_acc_b,
            ataRemoverLp: ata_user_lp,
            tokenAccLp: token_acc_lp,
            poolPda: PDA[0],
            tokenProgram: TOKEN_PROGRAM_ID,
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
  list.push({ "Property" : "State", "Value" : poolAccount.state });
  
  console.table(list);

};

remove_liquidity();