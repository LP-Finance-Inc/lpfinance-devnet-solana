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
import { LpFI, LpSOL, LpUSD, NETWORK, StableswapLpsolWsol, StableswapLpusdUsdc, stableswap_programID, UniswapLpfiUsdc, uniswap_programID, USDC, WSOL } from "./config";

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

  const escrow_pda = await PublicKey.findProgramAddress(
    [
        Buffer.from("swap-escrow")
    ],
    program.programId
  ); 
  console.log("escrow_pda:", escrow_pda[0].toBase58());
  console.log("escrow_pda:", escrow_pda[1]);

  const stableswap_pool = StableswapLpusdUsdc;
  const uniswap_pool = UniswapLpfiUsdc;

  const user_ata_lpusd = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    LpUSD
  );
  console.log('user_ata_lpusd:', user_ata_lpusd.toBase58());
  
  const user_ata_lpfi = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    LpFI
  );
  console.log('user_ata_lpfi:', user_ata_lpfi.toBase58());

  const stableswap_pool_ata_lpusd = await findAssociatedTokenAddress(
    stableswap_pool,
    LpUSD
  );
  console.log('stableswap_pool_ata_lpusd:', stableswap_pool_ata_lpusd.toBase58());
  
  const stableswap_pool_ata_usdc = await findAssociatedTokenAddress(
    stableswap_pool,
    USDC
  );
  console.log('stableswap_pool_ata_usdc:', stableswap_pool_ata_usdc.toBase58());

  const uniswap_pool_ata_lpfi = await findAssociatedTokenAddress(
    uniswap_pool,
    LpFI
  );
  console.log('uniswap_pool_ata_lpfi:', uniswap_pool_ata_lpfi.toBase58());
  
  const uniswap_pool_ata_usdc = await findAssociatedTokenAddress(
    uniswap_pool,
    USDC
  );
  console.log('uniswap_pool_ata_usdc:', uniswap_pool_ata_usdc.toBase58());

  const escrow_ata_lpusd = await findAssociatedTokenAddress(
    escrow_pda[0],
    LpUSD
  );
  console.log('escrow_ata_lpusd:', escrow_ata_lpusd.toBase58());
  
  const escrow_ata_lpfi = await findAssociatedTokenAddress(
    escrow_pda[0],
    LpFI
  );
  console.log('escrow_ata_lpfi:', escrow_ata_lpfi.toBase58());
  
  const escrow_ata_usdc = await findAssociatedTokenAddress(
    escrow_pda[0],
    USDC
  );
  console.log('escrow_ata_usdc:', escrow_ata_usdc.toBase58());
  

  const amount_lpusd = 10 * 1e9;

  const result = await program.rpc.swapLpusdToLpfi(
    new anchor.BN(amount_lpusd),
    {
        accounts: {
          user: authKeypair.publicKey,
          swapPda: escrow_pda[0],
          stableSwapPool: stableswap_pool,
          uniswapPool: uniswap_pool,
          tokenLpusd: LpUSD,
          tokenLpfi: LpFI,
          tokenUsdc: USDC,
          userAtaLpusd: user_ata_lpusd,
          userAtaLpfi: user_ata_lpfi,
          stableswapPoolAtaLpusd: stableswap_pool_ata_lpusd,
          stableswapPoolAtaUsdc: stableswap_pool_ata_usdc,
          uniswapPoolAtaLpfi: uniswap_pool_ata_lpfi,
          uniswapPoolAtaUsdc: uniswap_pool_ata_usdc,
          escrowAtaLpusd: escrow_ata_lpusd,
          escrowAtaLpfi: escrow_ata_lpfi,
          escrowAtaUsdc: escrow_ata_usdc,
          stableswapProgram: stableswap_programID,
          uniswapProgram: uniswap_programID,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY
        },
    });
    console.log("result : ", result);
        
};

stable_swap();