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

const stable_swap = async () => {
    
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
  const amount_a = stableswap_pool_acc.amountA.toNumber();
  const amount_b = stableswap_pool_acc.amountB.toNumber();

  const token_src = token_a;
  const token_dest = token_b;

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
    StableswapPool,
    token_src
  );
  console.log('pool_ata_src:', pool_ata_src.toBase58());
  
  const pool_ata_dest = await findAssociatedTokenAddress(
    StableswapPool,
    token_dest
  );
  console.log('pool_ata_dest:', pool_ata_dest.toBase58());

  const amount_src = 10 * 1e9;

  const amount_dest = await program.rpc.stableswapTokens(
    new anchor.BN(amount_src),
    {
        accounts: {
            stableSwapPool: StableswapPool,
            user: authKeypair.publicKey,
            tokenSrc: token_src,
            tokenDest: token_dest,
            userAtaSrc: user_ata_src,
            userAtaDest: user_ata_dest,
            poolAtaSrc: pool_ata_src,
            poolAtaDest: pool_ata_dest,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY
        },
    });
    console.log("returned : ", amount_dest);

    console.log("Swap tokens !")

    function sleep(milliseconds) {
        const date = Date.now();
        let currentDate = null;
        do {
          currentDate = Date.now();
        } while (currentDate - date < milliseconds);
      }
    sleep(1000);
      
    const stableswap_pool_acc_new = await program.account.stableswapPool.fetch(StableswapPool);
    const amount_a_new = stableswap_pool_acc_new.amountA.toNumber();
    const amount_b_new = stableswap_pool_acc_new.amountB.toNumber();

    console.log("token_a : ", amount_a, ' -> ', amount_a_new, ' : ', amount_a_new - amount_a);
    console.log("token_b : ", amount_b, ' -> ', amount_b_new, ' : ', amount_b_new - amount_b);
        
};

stable_swap();