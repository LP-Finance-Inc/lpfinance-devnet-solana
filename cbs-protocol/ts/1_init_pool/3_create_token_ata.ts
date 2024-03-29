import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CbsProtocol } from "../../target/types/cbs_protocol";
import TestTokenIDL from "../../../test-tokens/target/idl/test_tokens.json";

import { 
  ASSOCIATED_TOKEN_PROGRAM_ID,
    TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import {
  PublicKey,
  Connection,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { TestTokenConfig, NETWORK } from "../config";
import { getATAPublicKey, getCreatorKeypair, getPublicKey, writePublicKeys } from "../utils";

const { Wallet } = anchor;

const PREFIX = "cbs-pda";

const create_token_ata = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;
    
    // Config
    const config = getPublicKey('cbs_config');

    let pubkeys = "";
    const testTokenProgram = new anchor.Program(TestTokenIDL as anchor.Idl, TestTokenIDL.metadata.address);
    const testTokenConfigData = await testTokenProgram.account.config.fetch(TestTokenConfig);

    const wsolMint = testTokenConfigData.wsolMint as PublicKey;
    const rayMint = testTokenConfigData.rayMint as PublicKey;
    const msolMint = testTokenConfigData.msolMint as PublicKey;
    const srmMint = testTokenConfigData.srmMint as PublicKey;
    const scnsolMint = testTokenConfigData.scnsolMint as PublicKey;
    const stsolMint = testTokenConfigData.stsolMint as PublicKey;
    const usdcMint = testTokenConfigData.usdcMint as PublicKey;
    
    const PDA = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX)],
      program.programId
    );    

    // Find PDA for `Ray pool`
    const poolRayKeypair = await getATAPublicKey(rayMint, PDA[0]); // anchor.web3.Keypair.generate();  
    const poolRayKeyString = `export const RAY_CBS_ATA = new PublicKey("${poolRayKeypair.toString()}");\n`
    pubkeys += poolRayKeyString;

      // Find PDA for `Wsol pool`
    const poolWsolKeypair = await getATAPublicKey(wsolMint, PDA[0]); // anchor.web3.Keypair.generate();  
    const poolWsolKeyString = `export const WSOL_CBS_ATA = new PublicKey("${poolWsolKeypair.toString()}");\n`
    pubkeys += poolWsolKeyString;

    // Find PDA for `Msol pool`
    const poolMsolKeypair = await getATAPublicKey(msolMint, PDA[0]); // anchor.web3.Keypair.generate();    
    const poolMsolKeyString = `export const MSOL_CBS_ATA = new PublicKey("${poolMsolKeypair.toString()}");\n`
    pubkeys += poolMsolKeyString;

    // Find PDA for `Srm pool`
    const poolSrmKeypair = await getATAPublicKey(srmMint, PDA[0]); // anchor.web3.Keypair.generate();  
    const poolSrmKeyString = `export const SRM_CBS_ATA = new PublicKey("${poolSrmKeypair.toString()}");\n`
    pubkeys += poolSrmKeyString;

    // Find PDA for `Scnsol pool`
    const poolScnsolKeypair = await getATAPublicKey(scnsolMint, PDA[0]); // anchor.web3.Keypair.generate();  
    const poolScnsolKeyString = `export const SCNSOL_CBS_ATA = new PublicKey("${poolScnsolKeypair.toString()}");\n`
    pubkeys += poolScnsolKeyString;

    // Find PDA for `Stsol pool`
    const poolStsolKeypair = await getATAPublicKey(stsolMint, PDA[0]); // anchor.web3.Keypair.generate();    
    const poolStsolKeyString = `export const STSOL_CBS_ATA = new PublicKey("${poolStsolKeypair.toString()}");\n`
    pubkeys += poolStsolKeyString;

    // Find PDA for `Usdc pool`
    const poolUsdcKeypair = await getATAPublicKey(usdcMint, PDA[0]); // anchor.web3.Keypair.generate();  
    const poolUsdcKeyString = `export const USDC_CBS_ATA = new PublicKey("${poolUsdcKeypair.toString()}");\n`
    pubkeys += poolUsdcKeyString;


    
    writePublicKeys(pubkeys, "cbs_tokens_ata");

    // create Escrow for Liquidate
    await program.rpc.createUsdcEscrow({
      accounts: {
        authority: creatorKeypair.publicKey,
        usdcMint,
        cbsPda: PDA[0],
        poolUsdc: poolUsdcKeypair,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },      
    });
    
    // create ATA
    await program.rpc.createTokenAta1({
      accounts: {
        authority: creatorKeypair.publicKey,
        config,
        wsolMint,
        rayMint,
        msolMint,
        cbsPda: PDA[0],
        poolRay: poolRayKeypair,
        poolWsol: poolWsolKeypair,
        poolMsol: poolMsolKeypair,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },      
    });

    await program.rpc.createTokenAta2({
      accounts: {
        authority: creatorKeypair.publicKey,
        config,
        srmMint,
        scnsolMint,
        stsolMint,
        cbsPda: PDA[0],
        poolSrm: poolSrmKeypair,
        poolScnsol: poolScnsolKeypair,
        poolStsol: poolStsolKeypair,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },      
    });
}

create_token_ata();