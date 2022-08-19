import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CbsProtocol } from "../../target/types/cbs_protocol";
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
import { LpfinanceTokenConfig, NETWORK } from "../config";
import { getATAPublicKey, getCreatorKeypair, getPublicKey, writePublicKeys } from "../utils";

const { Wallet } = anchor;

const PREFIX = "cbs-pda";

const create_lptoken_ata = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;
    
    let pubkeys = "";
    // Config
    const config = getPublicKey('cbs_config');

    const lptokenProgram = new anchor.Program(LpfinanceTokenIDL as anchor.Idl, LpfinanceTokenIDL.metadata.address);
    const lptokenConfigData = await lptokenProgram.account.config.fetch(LpfinanceTokenConfig);

    const lpsolMint = lptokenConfigData.lpsolMint as PublicKey;
    const lpusdMint = lptokenConfigData.lpusdMint as PublicKey;
    const lpfiMint = lptokenConfigData.lpdaoMint as PublicKey;
    // console.log(lpfiMint.toBase58())

    const PDA = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX)],
      program.programId
    );    

    // Find PDA for `lpsol pool`
    const poolLpsolKeypair = await getATAPublicKey(lpsolMint, PDA[0]); // anchor.web3.Keypair.generate();  
    const poolLpsolKeyString = `export const LpSOL_CBS_ATA = new PublicKey("${poolLpsolKeypair.toString()}");\n`
    pubkeys += poolLpsolKeyString;

      // Find PDA for `lpusd pool`
    const poolLpusdKeypair = await getATAPublicKey(lpusdMint, PDA[0]); //anchor.web3.Keypair.generate();  
    const poolLpusdKeyString = `export const LpUSD_CBS_ATA = new PublicKey("${poolLpusdKeypair.toString()}");\n`
    pubkeys += poolLpusdKeyString;

    // Find PDA for `lpfi pool`
    const poolLpfiKeypair = await getATAPublicKey(lpfiMint, PDA[0]); // anchor.web3.Keypair.generate();    
    const poolLpfiKeyString = `export const LpFI_CBS_ATA= new PublicKey("${poolLpfiKeypair.toString()}");\n\n`
    pubkeys += poolLpfiKeyString;

    writePublicKeys(pubkeys, "cbs_lptokens_ata");
    
    // initialize
    await program.rpc.createLptokenAta({
      accounts: {
        authority: creatorKeypair.publicKey,
        config,
        lpsolMint,
        lpusdMint,
        lpfiMint,
        cbsPda: PDA[0],
        poolLpsol: poolLpsolKeypair,
        poolLpusd: poolLpusdKeypair,
        poolLpfi: poolLpfiKeypair,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });
}

create_lptoken_ata();