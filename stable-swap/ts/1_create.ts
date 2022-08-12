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
import { LpSOL, LpUSD, NETWORK, USDC, WSOL } from "./config";

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

const create_stableswap = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");

  const authKeypair = getCreatorKeypair(); // getKeypair("creator");
  console.log("Creator address:", authKeypair.publicKey.toBase58());

  const provider = new SignerWallet(authKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.StableSwap as Program<StableSwap>;

  // const lpsol_wsol_pool_key = 'lpsol-wsol-pool'
  // const token_a = LpSOL;
  // console.log("token_a : ", token_a.toBase58())

  // const token_b = WSOL;
  // console.log("token_b : ", token_b.toBase58())

  const lpusd_usdc_pool_key = 'lpusd-usdc-pool'
  const token_a = LpUSD;
  console.log("token_a : ", token_a.toBase58())

  const token_b = USDC;
  console.log("token_b : ", token_b.toBase58())

  const stableswap_pool_pda = await PublicKey.findProgramAddress(
    [
        Buffer.from("stable-swap"),
        token_a.toBuffer(),
        token_b.toBuffer(),
        authKeypair.publicKey.toBuffer()
    ],
    program.programId
  ); 
  console.log("Swap Swap Pool PDA address:", stableswap_pool_pda[0].toBase58());
  console.log("Swap Swap Pool PDA bump:", stableswap_pool_pda[1]);
  // writePublicKey(stableswap_pool_pda[0], lpsol_wsol_pool_key); 
  writePublicKey(stableswap_pool_pda[0], lpusd_usdc_pool_key); 

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
    stableswap_pool_pda[0],
    token_a
  );
  console.log('pool_ata_a:', pool_ata_a.toBase58());
  
  const pool_ata_b = await findAssociatedTokenAddress(
    stableswap_pool_pda[0],
    token_b
  );
  console.log('pool_ata_b:', pool_ata_b.toBase58());

  const token_lp_keypair = anchor.web3.Keypair.generate();
  console.log("token_lp : ", token_lp_keypair.publicKey.toBase58())

  const author_ata_lp = await findAssociatedTokenAddress(
    authKeypair.publicKey,
    token_lp_keypair.publicKey
  );
  console.log('author_ata_lp:', author_ata_lp.toBase58());

  const amount_a = 2000000 * 1e9;
  const amount_b = 2000000 * 1e9;
  const amp = 1000;
  const fee = 5;       //  = 0.5%

  await program.rpc.createStableswap(
    new anchor.BN(amount_a),
    new anchor.BN(amount_b),
    new anchor.BN(amp),
    new anchor.BN(fee),
    {
        accounts: {
            stableSwapPool: stableswap_pool_pda[0],
            author: authKeypair.publicKey,
            tokenA: token_a,
            tokenB: token_b,
            authorAtaA: author_ata_a,
            authorAtaB: author_ata_b,
            poolAtaA: pool_ata_a,
            poolAtaB: pool_ata_b,
            tokenLp: token_lp_keypair.publicKey,
            authorAtaLp: author_ata_lp,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY
        },
        signers: [
            token_lp_keypair
        ]
    });

    console.log("Created StabeSwap !")
};

create_stableswap();