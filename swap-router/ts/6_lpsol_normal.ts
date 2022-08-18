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
        Buffer.from("swap-escrow")
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

  const user_ata_lpsol = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    LpSOL
  );
  console.log('user_ata_lpsol:', user_ata_lpsol.toBase58());
  
  const user_ata_usdc = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    USDC
  );
  console.log('user_ata_usdc:', user_ata_usdc.toBase58());

  const stableswap_pool_ata_lpsol = await findAssociatedTokenAddress(
    StableswapLpsolWsol,
    LpSOL
  );
  console.log('stableswap_pool_ata_lpsol:', stableswap_pool_ata_lpsol.toBase58());
  
  const stableswap_pool_ata_wsol = await findAssociatedTokenAddress(
    StableswapLpsolWsol,
    WSOL
  );
  console.log('stableswap_pool_ata_wsol:', stableswap_pool_ata_wsol.toBase58());

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
  

  const amount_lpsol = 10 * 1e9;

  const result = await program.rpc.swapLpsolToNormal(
    new anchor.BN(amount_lpsol),
    {
        accounts: {
          user: authKeypair.publicKey,
          swapPda: escrow_pda[0],
          stableSwapPool: StableswapLpsolWsol,
          tokenStateAccount: token_state_account_pda[0],
          tokenLpsol: LpSOL,
          tokenWsol: WSOL,
          tokenDest: USDC,
          pythWsol: WSOL_PYTH,
          pythDest: USDC_PYTH,
          userAtaLpsol: user_ata_lpsol,
          userAtaDest: user_ata_usdc,
          stableswapPoolAtaLpsol: stableswap_pool_ata_lpsol,
          stableswapPoolAtaWsol: stableswap_pool_ata_wsol,
          escrowAtaLpsol: escrow_ata_lpsol,
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
        
};

stable_swap();