import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpusdAuction } from "../../target/types/lpusd_auction";
import TestTokenIDL from "../../../test-tokens/target/idl/test_tokens.json";

import { 
    TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import {
  PublicKey,
  Connection,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { TestTokenConfig, NETWORK, PREFIX } from "../config";
import { getCreatorKeypair, getPublicKey, writePublicKeys } from "../utils";

const { Wallet } = anchor;

const create_token_ata = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.LpusdAuction as Program<LpusdAuction>;
    
    // Config
    const config = getPublicKey('auction_config');

    let pubkeys = "";
    const lptokenProgram = new anchor.Program(TestTokenIDL as anchor.Idl, TestTokenIDL.metadata.address);
    const lptokenConfigData = await lptokenProgram.account.config.fetch(TestTokenConfig);

    const wsolMint = lptokenConfigData.wsolMint as PublicKey;
    const rayMint = lptokenConfigData.rayMint as PublicKey;
    const msolMint = lptokenConfigData.msolMint as PublicKey;
    const srmMint = lptokenConfigData.srmMint as PublicKey;
    const scnsolMint = lptokenConfigData.scnsolMint as PublicKey;
    const stsolMint = lptokenConfigData.stsolMint as PublicKey;
    
    // Find PDA for `Ray pool`
    const poolRayKeypair = anchor.web3.Keypair.generate();  
    const poolRayKeyString = `const poolRay = new PublicKey("${poolRayKeypair.publicKey.toString()}");\n`
    pubkeys += poolRayKeyString;

      // Find PDA for `Wsol pool`
    const poolWsolKeypair = anchor.web3.Keypair.generate();  
    const poolWsolKeyString = `const poolWsol = new PublicKey("${poolWsolKeypair.publicKey.toString()}");\n`
    pubkeys += poolWsolKeyString;

    // Find PDA for `Msol pool`
    const poolMsolKeypair = anchor.web3.Keypair.generate();    
    const poolMsolKeyString = `const poolMsol = new PublicKey("${poolMsolKeypair.publicKey.toString()}");\n`
    pubkeys += poolMsolKeyString;

    // Find PDA for `Srm pool`
    const poolSrmKeypair = anchor.web3.Keypair.generate();  
    const poolSrmKeyString = `const poolSrm = new PublicKey("${poolSrmKeypair.publicKey.toString()}");\n`
    pubkeys += poolSrmKeyString;

    // Find PDA for `Scnsol pool`
    const poolScnsolKeypair = anchor.web3.Keypair.generate();  
    const poolScnsolKeyString = `const poolScnsol = new PublicKey("${poolScnsolKeypair.publicKey.toString()}");\n`
    pubkeys += poolScnsolKeyString;

    // Find PDA for `Stsol pool`
    const poolStsolKeypair = anchor.web3.Keypair.generate();    
    const poolStsolKeyString = `const poolStsol = new PublicKey("${poolStsolKeypair.publicKey.toString()}");\n`
    pubkeys += poolStsolKeyString;

    const PDA = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX)],
        program.programId
    );    
    
    writePublicKeys(pubkeys, "auction_tokens_ata");

    // initialize
    await program.rpc.createTokenAta({
      accounts: {
        authority: creatorKeypair.publicKey,
        config,
        wsolMint,
        rayMint,
        msolMint,
        srmMint,
        scnsolMint,
        stsolMint,
        auctionPda: PDA[0],
        poolRay: poolRayKeypair.publicKey,
        poolWsol: poolWsolKeypair.publicKey,
        poolMsol: poolMsolKeypair.publicKey,
        poolSrm: poolSrmKeypair.publicKey,
        poolScnsol: poolScnsolKeypair.publicKey,
        poolStsol: poolStsolKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [poolRayKeypair, poolWsolKeypair, poolMsolKeypair, poolSrmKeypair, poolScnsolKeypair, poolStsolKeypair]
    });
}

create_token_ata();