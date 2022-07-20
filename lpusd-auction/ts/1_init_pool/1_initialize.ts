import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpusdAuction } from "../../target/types/lpusd_auction";
import {
  Connection,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";
import { NETWORK } from "../config";
import { getCreatorKeypair, writePublicKey } from "../utils";

const { Wallet } = anchor;

const initialize_config = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.LpusdAuction as Program<LpusdAuction>;
    
    // Config
    const configKeypair = anchor.web3.Keypair.generate();
    writePublicKey(configKeypair.publicKey, 'auction_config')

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