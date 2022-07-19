
import * as anchor from "@project-serum/anchor";
import {
    Connection,
    PublicKey
} from "@solana/web3.js";
import { 
    NETWORK,
    ApricotIDL,
    SolendIDL,
    SolendConfig,
    ApricotConfig,
} from "../config";
import { getPublicKey, getCreatorKeypair } from "../utils";

const { Wallet } = anchor;

const get_lending_account = async () => {    
    const connection = new Connection(NETWORK, "confirmed");

    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));

    // Config
    const solendAccount = getPublicKey('cbs_solend_account');
    const solendProgramId = new PublicKey(SolendIDL.metadata.address)
    const solendProgram = new anchor.Program(SolendIDL as anchor.Idl, solendProgramId)

    const apricotAccount = getPublicKey('cbs_apricot_account');
    const apricotProgramId = new PublicKey(ApricotIDL.metadata.address) 
    const apricotProgram = new anchor.Program(ApricotIDL as anchor.Idl, apricotProgramId);

    const solendAccountData = await solendProgram.account.userAccount.fetch(solendAccount);
    const apricotAccountData = await apricotProgram.account.userAccount.fetch(apricotAccount);

    const solendConfigData = await solendProgram.account.config.fetch(SolendConfig);
    const apricotConfigData = await apricotProgram.account.config.fetch(ApricotConfig);

    console.log("=== Solend =====")
    print_user_data(solendAccountData)

    console.log("=== Apricot =====")
    print_user_data(apricotAccountData)
}

get_lending_account();

const print_user_data = (userData) => {   
  
    console.table([
      { "Property": "owner", "Value": userData.owner.toBase58()},
      { "Property": "ray_amount", "Value" : userData.rayAmount.toString()},
      { "Property": "wsol_amount", "Value" : userData.wsolAmount.toString()},
      { "Property": "msol_amount", "Value" : userData.msolAmount.toString()},
      { "Property": "srm_amount", "Value" : userData.srmAmount.toString()},
      { "Property": "scnsol_amount", "Value" : userData.scnsolAmount.toString()},
      { "Property": "stsol_amount", "Value" : userData.stsolAmount.toString()},
    ]);
}

const print_config_data = (configData) => {   
  
    console.table([
      { "Property": "owner", "Value": configData.owner.toBase58()},
      { "Property": "ray_amount", "Value" : configData.rayAmount.toString()},
      { "Property": "wsol_amount", "Value" : configData.wsolAmount.toString()},
      { "Property": "msol_amount", "Value" : configData.msolAmount.toString()},
      { "Property": "srm_amount", "Value" : configData.srmAmount.toString()},
      { "Property": "scnsol_amount", "Value" : configData.scnsolAmount.toString()},
      { "Property": "stsol_amount", "Value" : configData.stsolAmount.toString()},
    ]);
}