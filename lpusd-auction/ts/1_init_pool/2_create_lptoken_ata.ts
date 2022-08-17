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
import { LpfinanceTokenConfig, NETWORK, PREFIX } from "../config";
import { getATAPublicKey, getCreatorKeypair, getPublicKey, writePublicKeys } from "../utils";

const { Wallet } = anchor;

const create_lptoken_ata = async () => {
    
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
    const lptokenProgram = new anchor.Program(LpfinanceTokenIDL as anchor.Idl, LpfinanceTokenIDL.metadata.address);
    const lptokenConfigData = await lptokenProgram.account.config.fetch(LpfinanceTokenConfig);
    
    const lpsolMint = lptokenConfigData.lpsolMint as PublicKey;
    const lpusdMint = lptokenConfigData.lpusdMint as PublicKey;
    // console.log(lpfiMint.toBase58())

    const PDA = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX)],
      program.programId
  );    
    // Find PDA for `lpsol pool`
    const poolLpsolKeypair = await getATAPublicKey(lpsolMint, PDA[0]) // anchor.web3.Keypair.generate();  
    const poolLpsolKeyString = `const poolLpsol = new PublicKey("${poolLpsolKeypair.toString()}");\n`
    pubkeys += poolLpsolKeyString;

      // Find PDA for `lpusd pool`
    const poolLpusdKeypair = await getATAPublicKey(lpusdMint, PDA[0]) // anchor.web3.Keypair.generate();  
    const poolLpusdKeyString = `const poolLpusd = new PublicKey("${poolLpusdKeypair.toString()}");\n`
    pubkeys += poolLpusdKeyString;

    const auctionPDAKeyString = `const auctionPDA = new PublicKey("${PDA[0].toString()}");`
    pubkeys += auctionPDAKeyString;

    writePublicKeys(pubkeys, "auction_lptokens_ata");

    // initialize
    await program.rpc.createLptokenAta({
      accounts: {
        authority: creatorKeypair.publicKey,
        config,
        lpsolMint,
        lpusdMint,
        auctionPda: PDA[0],
        poolLpsol: poolLpsolKeypair,
        poolLpusd: poolLpusdKeypair,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });
}

create_lptoken_ata();