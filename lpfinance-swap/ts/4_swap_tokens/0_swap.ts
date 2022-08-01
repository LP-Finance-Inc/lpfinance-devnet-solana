import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpfinanceSwap } from "../../target/types/lpfinance_swap";
import { createMemoInstruction, SignerWallet, suppressConsoleErrorAsync } from "@saberhq/solana-contrib";
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
  getATAPublicKey, 
  getCreatorKeypair 
} from "../utils";
import { LpSOLMint, LPSWAP_PREFIX, LpUSDMint, NETWORK, pythUsdcAccount, USDCMint, wSOLMint } from "../config";

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

const swap_pool = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");

  const userKeypair = getCreatorKeypair(); 

  const provider = new SignerWallet(userKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpfinanceSwap as Program<LpfinanceSwap>;

  const pool_pubkey = await getPublicKey(`lpfi-usdc-pool`);
  console.log("pool pubkey : ", pool_pubkey.toBase58());

  let poolAccount = await program.account.poolInfo.fetch(pool_pubkey);


  const token_acc_a = poolAccount.tokenAccA;
  const token_acc_b = poolAccount.tokenAccB;

  const amount_swap = 1 * 1e9;
  const quote_mint = poolAccount.tokenaMint;
  const dest_mint = poolAccount.tokenbMint;

  const ata_user_a = await getATAPublicKey(quote_mint, userKeypair.publicKey) 
  console.log("user_ata_a : ", ata_user_a.toBase58());

  const ata_user_b = await getATAPublicKey(dest_mint, userKeypair.publicKey) 
  console.log("user_ata_b : ", ata_user_b.toBase58());

  const swapProgramId = program.programId;
  const PDA = await PublicKey.findProgramAddress(
    [Buffer.from(LPSWAP_PREFIX)],
    swapProgramId
  );

  await program.rpc.swapTokenToToken( 
    new anchor.BN(amount_swap), 
    {
        accounts: {
            liquidityPool: pool_pubkey,
            userAuthority: userKeypair.publicKey,
            quoteMint: quote_mint,
            destMint: dest_mint,
            userQuote: ata_user_a,
            userDest: ata_user_b,
            quotePool: token_acc_a,
            destPool: token_acc_b,
            stateAccount: PDA[0],
            pythQuoteAccount: pythUsdcAccount,
            pythDestAccount: pythUsdcAccount,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY  
        },
  });

  console.log("Token Swap Completed");

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

swap_pool();