
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
    APRICOT_PREFIX,
    ApricotIDL,
} from "../config";
import { getCreatorKeypair, getPublicKey, writePublicKey } from "../utils";

const { Wallet } = anchor;


const create_accounts = async () => {
    
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
    
    const apricotProgramId = new PublicKey(ApricotIDL.metadata.address)

    const [apricotAccount, apricotBump] = await PublicKey.findProgramAddress(
        [Buffer.from(APRICOT_PREFIX), Buffer.from(PDA[0].toBuffer())],
        apricotProgramId
    );
    
    writePublicKey(apricotAccount, 'cbs_apricot_account')

    // initialize
    await program.rpc.createApricotCbsAccount({
      accounts: {
        owner: creatorKeypair.publicKey,
        config: config,
        cbsPda: PDA[0],
        apricotAccount,
        apricotProgram: apricotProgramId,
        systemProgram: anchor.web3.SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });

    console.table([
        {"Property" : "cbs_apricot_account", "Value": apricotAccount.toBase58()},
    ]);
}

create_accounts();