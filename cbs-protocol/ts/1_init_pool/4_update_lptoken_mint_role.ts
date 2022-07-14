import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CbsProtocol } from "../../target/types/cbs_protocol";
import LpfinanceTokenIDL from "../../../lpfinance-tokens/target/idl/lpfinance_tokens.json";

import { 
    TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import {
  PublicKey,
  Connection,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { LpfinanceTokenConfig, LpfinanceTokenPDA, NETWORK, PREFIX } from "../config";
import { getCreatorKeypair, getPublicKey, writePublicKeys } from "../utils";

const { Wallet } = anchor;

const update_lptoken_mint_role = async () => {  
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;
    const PDA = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX)],
        program.programId
    );    

    const lptokenProgram = new anchor.Program(LpfinanceTokenIDL as anchor.Idl, LpfinanceTokenIDL.metadata.address);
    // initialize
    await lptokenProgram.rpc.updateCbsAccount(
        PDA[0],
        {
            accounts: {
                owner: creatorKeypair.publicKey,
                config: LpfinanceTokenConfig,
                stateAccount: LpfinanceTokenPDA
            }
        });

    console.log("Successfully Done")
}

update_lptoken_mint_role();