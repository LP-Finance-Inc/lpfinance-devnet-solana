import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpfinanceSwap } from "../../target/types/lpfinance_swap";
import { createMemoInstruction, SignerWallet, suppressConsoleErrorAsync } from "@saberhq/solana-contrib";
import { 
    TOKEN_PROGRAM_ID, 
    ASSOCIATED_TOKEN_PROGRAM_ID,
} from '@solana/spl-token';
import {
  PublicKey,
  Connection,
} from "@solana/web3.js";

import {
  getPublicKey,
  getCreatorKeypair,
  writePublicKey,
} from "../utils";
import { NETWORK } from "../config";

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

const getUserLptokenBalance = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");
  const userKeypair = getCreatorKeypair();

  const provider = new SignerWallet(userKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpfinanceSwap as Program<LpfinanceSwap>;

  const pool_pubkey = await getPublicKey(`lpfi-usdc-pool`);
  console.log("pool pubkey : ", pool_pubkey.toBase58());

  let poolAccount = await program.account.poolInfo.fetch(pool_pubkey);

  const token_lp = poolAccount.tokenLp;

  const ata_user_lp = await findAssociatedTokenAddress(
    userKeypair.publicKey,
    token_lp
  );
  console.log('ata User LP:', ata_user_lp.toBase58());
  let userLptokenBalance = await connection.getTokenAccountBalance(ata_user_lp);
  console.log(userLptokenBalance.value.uiAmount)
};

getUserLptokenBalance();