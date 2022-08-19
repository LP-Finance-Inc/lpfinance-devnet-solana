import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpusdAuction } from "../../target/types/lpusd_auction";
import LpfinanceTokenIDL from "../../../lpfinance-tokens/target/idl/lpfinance_tokens.json";

import { 
  ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import {
  PublicKey,
  Connection,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { LpfinanceTokenConfig, NETWORK, AUCTION_PREFIX, TestTokenConfig, TestTokenIDL } from "../config";
import { getATAPublicKey, getCreatorKeypair, getPublicKey, writePublicKeys } from "../utils";

const { Wallet } = anchor;

const create_normaltoken_ata = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.LpusdAuction as Program<LpusdAuction>;
    
    let pubkeys = "";
    // Config
    const config = getPublicKey('auction_config');
    // const configData = await program.account.config.fetch(config);
    // console.log(configData.poolLpsol.toBase58());
    // return;
    const testTokenProgram = new anchor.Program(TestTokenIDL as anchor.Idl, TestTokenIDL.metadata.address);
    const testTokenConfigData = await testTokenProgram.account.config.fetch(TestTokenConfig);
    
    const wsolMint = testTokenConfigData.wsolMint as PublicKey;
    const usdcMint = testTokenConfigData.usdcMint as PublicKey;

    const PDA = await PublicKey.findProgramAddress(
      [Buffer.from(AUCTION_PREFIX)],
      program.programId
  );    
    // Find PDA for `wSOL pool`
    const poolWsolKeypair = await getATAPublicKey(wsolMint, PDA[0]) // anchor.web3.Keypair.generate();  
    const poolWsolKeyString = `export const auctionWsolPool = new PublicKey("${poolWsolKeypair.toString()}");\n`
    pubkeys += poolWsolKeyString;

      // Find PDA for `usdc pool`
    const poolUsdcKeypair = await getATAPublicKey(usdcMint, PDA[0]) // anchor.web3.Keypair.generate();  
    const poolUsdcKeyString = `export const auctionUsdcPool = new PublicKey("${poolUsdcKeypair.toString()}");\n`
    pubkeys += poolUsdcKeyString;

    writePublicKeys(pubkeys, "auction_tokens_ata");

    // initialize
    await program.rpc.createNormaltokenAta({
      accounts: {
        authority: creatorKeypair.publicKey,
        config,
        wsolMint,
        usdcMint,
        auctionPda: PDA[0],
        poolWsol: poolWsolKeypair,
        poolUsdc: poolUsdcKeypair,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });
}

create_normaltoken_ata();