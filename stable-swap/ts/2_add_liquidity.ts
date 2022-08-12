import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StableSwap } from "../target/types/stable_swap";
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
import { LpUSD, NETWORK, StableswapPool, USDC } from "./config";

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

const add_liquidity = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");

  const authKeypair = getCreatorKeypair(); // getKeypair("creator");
  console.log("Creator address:", authKeypair.publicKey.toBase58());

  const provider = new SignerWallet(authKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.StableSwap as Program<StableSwap>;

  const stableswap_pool_acc = await program.account.stableswapPool.fetch(StableswapPool);
  const token_a = stableswap_pool_acc.tokenA;
  const token_b = stableswap_pool_acc.tokenB;
  const token_lp = stableswap_pool_acc.tokenLp;

  const author_ata_a = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    token_a
  );
  console.log('author_ata_a:', author_ata_a.toBase58());
  
  const author_ata_b = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    token_b
  );
  console.log('author_ata_b:', author_ata_b.toBase58());

  const pool_ata_a = await findAssociatedTokenAddress(
    StableswapPool,
    token_a
  );
  console.log('pool_ata_a:', pool_ata_a.toBase58());
  
  const pool_ata_b = await findAssociatedTokenAddress(
    StableswapPool,
    token_b
  );
  console.log('pool_ata_b:', pool_ata_b.toBase58());

  const author_ata_lp = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    token_lp
  );
  console.log('author_ata_lp:', author_ata_lp.toBase58());

  const amount_a = 100000 * 1e9;

  await program.rpc.addLiquidityStableswap(
    new anchor.BN(amount_a),
    {
        accounts: {
            stableSwapPool: StableswapPool,
            adder: authKeypair.publicKey,
            tokenA: token_a,
            tokenB: token_b,
            adderAtaA: author_ata_a,
            adderAtaB: author_ata_b,
            poolAtaA: pool_ata_a,
            poolAtaB: pool_ata_b,
            tokenLp: token_lp,
            adderAtaLp: author_ata_lp,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY
        }
    });

    console.log("Add Liquidity StabeSwap !")
};

add_liquidity();