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
  getProgramId
} from "./utils";
import { LpSOL, LpUSD, NETWORK, StableSwapIDL, USDC, WSOL } from "./config";

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

const stable_swap = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");

  const authKeypair = getCreatorKeypair(); // getKeypair("creator");
  console.log("Creator address:", authKeypair.publicKey.toBase58());

  const provider = new SignerWallet(authKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.SwapRouter as Program<SwapRouter>;

  const swap_escrow_pool_pda = await PublicKey.findProgramAddress(
    [
        Buffer.from("swap-escrow"),
        authKeypair.publicKey.toBuffer()
    ],
    program.programId
  ); 
  console.log("Swap Escrow Pool PDA address:", swap_escrow_pool_pda[0].toBase58());
  console.log("Swap Escrow Pool PDA bump:", swap_escrow_pool_pda[1]);

  // const stableswap_pool = StableswapLpusdUsdc

  // const token_src = LpUSD;
  // const token_dest = USDC;

  // const token_src = USDC;
  // const token_dest = LpUSD;

  const stableswap_pool = getPublicKey('lpsol-wsol-pool');

  // const token_src = LpSOL;
  // const token_dest = WSOL;

  const token_src = WSOL;
  const token_dest = LpSOL;

  const user_ata_src = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    token_src
  );
  console.log('user_ata_src:', user_ata_src.toBase58());
  
  const user_ata_dest = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    token_dest
  );
  console.log('user_ata_dest:', user_ata_dest.toBase58());

  const pool_ata_src = await findAssociatedTokenAddress(
    stableswap_pool,
    token_src
  );
  console.log('pool_ata_src:', pool_ata_src.toBase58());
  
  const pool_ata_dest = await findAssociatedTokenAddress(
    stableswap_pool,
    token_dest
  );
  console.log('pool_ata_dest:', pool_ata_dest.toBase58());

  const escrow_ata_src = await findAssociatedTokenAddress(
    swap_escrow_pool_pda[0],
    token_src
  );
  console.log('escrow_ata_src:', escrow_ata_src.toBase58());
  
  const escrow_ata_dest = await findAssociatedTokenAddress(
    swap_escrow_pool_pda[0],
    token_dest
  );
  console.log('escrow_ata_dest:', escrow_ata_dest.toBase58());

  const amount_src = 10 * 1e9;
  
  const stableswap_programID = new PublicKey(StableSwapIDL.metadata.address);
  const result = await program.rpc.swapStableswap(
    new anchor.BN(amount_src),
    {
        accounts: {
          user: authKeypair.publicKey,
          swapEscrow: swap_escrow_pool_pda[0],
          stableSwapPool: stableswap_pool,
          tokenSrc: token_src,
          tokenDest: token_dest,
          userAtaSrc: user_ata_src,
          userAtaDest: user_ata_dest,
          poolAtaSrc: pool_ata_src,
          poolAtaDest: pool_ata_dest,
          escrowAtaSrc: escrow_ata_src,
          escrowAtaDest: escrow_ata_dest,
          stableswapProgram: stableswap_programID,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY
        },
    });
    console.log("result : ", result);
        
};

stable_swap();