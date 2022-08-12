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
import { LpFI, LpSOL, LpUSD, NETWORK, StableswapLpsolWsol, StableswapLpusdUsdc, stableswap_programID, testToken_programID, UniswapLpfiUsdc, uniswap_programID, USDC, USDC_PYTH, WSOL, WSOL_PYTH } from "./config";

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
        Buffer.from("swap-escrow"),
        authKeypair.publicKey.toBuffer()
    ],
    program.programId
  ); 
  console.log("escrow_pda:", escrow_pda[0].toBase58());
  console.log("escrow_pda:", escrow_pda[1]);

  const token_state_account_pda = await PublicKey.findProgramAddress(
    [
        Buffer.from("test-tokens")
    ], 
    testToken_programID
  );
  console.log("token_state_account_pda : ", token_state_account_pda[0].toBase58());

  const user_ata_lpusd = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    LpUSD
  );
  console.log('user_ata_lpusd:', user_ata_lpusd.toBase58());
  
  const user_ata_lpsol = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    LpSOL
  );
  console.log('user_ata_lpsol:', user_ata_lpsol.toBase58());

  const stableswap_pool_lpusd_usdc_ata_lpusd = await findAssociatedTokenAddress(
    StableswapLpusdUsdc,
    LpUSD
  );
  console.log('stableswap_pool_lpusd_usdc_ata_lpusd:', stableswap_pool_lpusd_usdc_ata_lpusd.toBase58());
  
  const stableswap_pool_lpusd_usdc_ata_usdc = await findAssociatedTokenAddress(
    StableswapLpusdUsdc,
    USDC
  );
  console.log('stableswap_pool_lpusd_usdc_ata_usdc:', stableswap_pool_lpusd_usdc_ata_usdc.toBase58());

  const stableswap_pool_lpsol_wsol_ata_lpsol = await findAssociatedTokenAddress(
    StableswapLpsolWsol,
    LpSOL
  );
  console.log('stableswap_pool_lpsol_wsol_ata_lpsol:', stableswap_pool_lpsol_wsol_ata_lpsol.toBase58());
  
  const stableswap_pool_lpsol_wsol_ata_wsol = await findAssociatedTokenAddress(
    StableswapLpsolWsol,
    WSOL
  );
  console.log('stableswap_pool_lpsol_wsol_ata_wsol:', stableswap_pool_lpsol_wsol_ata_wsol.toBase58());

  const escrow_ata_lpusd = await findAssociatedTokenAddress(
    escrow_pda[0],
    LpUSD
  );
  console.log('escrow_ata_lpusd:', escrow_ata_lpusd.toBase58());
  
  const escrow_ata_usdc = await findAssociatedTokenAddress(
    escrow_pda[0],
    USDC
  );
  console.log('escrow_ata_usdc:', escrow_ata_usdc.toBase58());

  const escrow_ata_lpsol = await findAssociatedTokenAddress(
    escrow_pda[0],
    LpSOL
  );
  console.log('escrow_ata_lpsol:', escrow_ata_lpsol.toBase58());
  
  const escrow_ata_wsol = await findAssociatedTokenAddress(
    escrow_pda[0],
    WSOL
  );
  console.log('escrow_ata_wsol:', escrow_ata_wsol.toBase58());

  const amount_lpusd = 10 * 1e9;

  const result = await program.rpc.swapLpusdToLpsolStep1(
    new anchor.BN(amount_lpusd),
    {
        accounts: {
          user: authKeypair.publicKey,
          swapEscrow: escrow_pda[0],
          stableSwapPool: StableswapLpusdUsdc,
          tokenStateAccount: token_state_account_pda[0],
          tokenLpusd: LpUSD,
          tokenUsdc: USDC,
          tokenWsol: WSOL,
          pythUsdc: USDC_PYTH,
          pythWsol: WSOL_PYTH,
          userAtaLpusd: user_ata_lpusd,
          stableswapPoolAtaLpusd: stableswap_pool_lpusd_usdc_ata_lpusd,
          stableswapPoolAtaUsdc: stableswap_pool_lpusd_usdc_ata_usdc,
          escrowAtaLpusd: escrow_ata_lpusd,
          escrowAtaUsdc: escrow_ata_usdc,
          escrowAtaWsol: escrow_ata_wsol,
          stableswapProgram: stableswap_programID,
          testtokensProgram: testToken_programID,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY                              
        },
    });
    console.log("result : ", result);

    const result1 = await program.rpc.swapLpusdToLpsolStep2(
      {
          accounts: {
            user: authKeypair.publicKey,
            swapEscrow: escrow_pda[0],
            stableSwapPool: StableswapLpsolWsol,
            tokenLpsol: LpSOL,
            tokenWsol: WSOL,
            userAtaLpsol: user_ata_lpsol,
            stableswapPoolAtaLpsol: stableswap_pool_lpsol_wsol_ata_lpsol,
            stableswapPoolAtaWsol: stableswap_pool_lpsol_wsol_ata_wsol,
            escrowAtaLpsol: escrow_ata_lpsol,
            escrowAtaWsol: escrow_ata_wsol,
            stableswapProgram: stableswap_programID,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY                              
          },
      });
      console.log("result1 : ", result1);
      
};

stable_swap();