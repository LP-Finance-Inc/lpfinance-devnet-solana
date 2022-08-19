import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Uniswap } from "../target/types/uniswap";
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
import { LpFI, NETWORK, USDC } from "./config";

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

const create_uniswap = async () => {
    
  const connection = new Connection(NETWORK, "confirmed");

  const authKeypair = getCreatorKeypair(); // getKeypair("creator");
  console.log("Creator address:", authKeypair.publicKey.toBase58());

  const provider = new SignerWallet(authKeypair).createProvider(connection);
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.Uniswap as Program<Uniswap>;

  const token_a = LpFI;
  console.log("token_a : ", token_a.toBase58())

  const token_b = USDC;
  console.log("token_b : ", token_b.toBase58())

  const uniswap_pool_pda = await PublicKey.findProgramAddress(
    [
        Buffer.from("uniswap"),
        token_a.toBuffer(),
        token_b.toBuffer(),
        authKeypair.publicKey.toBuffer()
    ],
    program.programId
  ); 
  console.log("Uniswap Pool PDA address:", uniswap_pool_pda[0].toBase58());
  console.log("Uniswap Pool PDA bump:", uniswap_pool_pda[1]);
  writePublicKey(uniswap_pool_pda[0], `lpfi_usdc_pool`); 

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
    uniswap_pool_pda[0],
    token_a
  );
  console.log('pool_ata_a:', pool_ata_a.toBase58());
  
  const pool_ata_b = await findAssociatedTokenAddress(
    uniswap_pool_pda[0],
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
  const fee = 5;       //  = 0.5%

  await program.rpc.createUniswap(
    new anchor.BN(amount_a),
    new anchor.BN(amount_b),
    new anchor.BN(fee),
    {
        accounts: {
            uniswapPool: uniswap_pool_pda[0],
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

create_uniswap();