import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CbsProtocol } from "../../target/types/cbs_protocol";
import {
  Connection,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { NETWORK } from "../config";
import { getCreatorKeypair, writePublicKey } from "../utils";

const { Wallet } = anchor;

const PREFIX = "cbs-pda";

const initialize_config = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;
    
    // Config
    const configKeypair = anchor.web3.Keypair.generate();
    writePublicKey(configKeypair.publicKey, 'cbs_config')

    // initialize
    await program.rpc.initialize({
      accounts: {
        authority: creatorKeypair.publicKey,
        config: configKeypair.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [configKeypair]
    });
}

initialize_config();