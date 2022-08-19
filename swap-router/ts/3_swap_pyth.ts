import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SwapRouter } from "../target/types/swap_router";
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
  getProgramId,
  getATAPublicKey
} from "./utils";
import { LpFI, LpSOL, LpUSD, NETWORK, stableswap_programID, TestTokenIDL, UniswapLpfiUsdc, uniswap_programID, USDC, USDC_PYTH, WSOL, WSOL_PYTH } from "./config";

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

const swap_pyth = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");

  const authKeypair = getCreatorKeypair(); // getKeypair("creator");
  console.log("Creator address:", authKeypair.publicKey.toBase58());

  const provider = new SignerWallet(authKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.SwapRouter as Program<SwapRouter>;

  const tokenProgramID = new PublicKey(TestTokenIDL.metadata.address);
  const token_state_account_pda = await PublicKey.findProgramAddress(
    [
        Buffer.from("test-tokens")
    ], 
    tokenProgramID
  );
  console.log("token_state_account_pda : ", token_state_account_pda[0].toBase58());

  const escrow_pda = await PublicKey.findProgramAddress(
    [
      Buffer.from("swap-escrow")
    ],
    program.programId
  ); 

  const token_src = USDC;
  const token_dest = WSOL;
  const swapAtaDest = await getATAPublicKey(token_dest, escrow_pda[0]);

  const user_ata_src = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    token_src
  );
  console.log('user_ata_src:', swapAtaDest.toBase58(), user_ata_src.toBase58());
  
  const user_ata_dest = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    token_dest
  );
  console.log('user_ata_dest:', user_ata_dest.toBase58());

  const amount_src = 10 * 1e9;

  const result = await program.rpc.swapPyth(
    new anchor.BN(amount_src),
    {
        accounts: {
          user: authKeypair.publicKey,
          tokenStateAccount: token_state_account_pda[0],
          tokenSrc: token_src,
          tokenDest: token_dest,
          pythSrc: USDC_PYTH,
          pythDest: WSOL_PYTH,
          userAtaSrc: user_ata_src,
          userAtaDest: user_ata_dest,
          swapAtaDest,
          swapPda: escrow_pda[0],
          testtokensProgram: tokenProgramID,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY
        },
    });
    console.log("result : ", result);
        
};

swap_pyth();