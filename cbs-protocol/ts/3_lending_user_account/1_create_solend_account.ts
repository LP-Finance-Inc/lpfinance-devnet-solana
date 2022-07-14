
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CbsProtocol } from "../../target/types/cbs_protocol";
import {
  Connection,
  SYSVAR_RENT_PUBKEY,
  PublicKey
} from "@solana/web3.js";
import { 
    NETWORK,
    PREFIX,
    SolendIDL,
    SOLEND_PREFIX,
} from "../config";
import { getCreatorKeypair, getPublicKey, writePublicKey } from "../utils";

const { Wallet } = anchor;


const create_solend_account = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;
    
    // Config
    const config = getPublicKey('cbs_config');

    const PDA = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX)],
        program.programId
    );    
    
    const solendProgramId = new PublicKey(SolendIDL.metadata.address)
    const [solendAccount, solendBump] = await PublicKey.findProgramAddress(
        [Buffer.from(SOLEND_PREFIX), Buffer.from(PDA[0].toBuffer())],
        solendProgramId
    );
    
    writePublicKey(solendAccount, 'cbs_solend_account')

    // initialize
    await program.rpc.createSolendCbsAccount({
      accounts: {
        owner: creatorKeypair.publicKey,
        config: config,
        cbsPda: PDA[0],
        solendAccount,
        solendProgram: solendProgramId,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });

    console.table([
        {"Property" : "cbs_solend_account", "Value": solendAccount.toBase58()},
    ]);
}

create_solend_account();