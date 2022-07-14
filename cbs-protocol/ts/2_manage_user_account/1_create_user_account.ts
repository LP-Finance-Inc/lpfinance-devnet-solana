import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CbsProtocol } from "../../target/types/cbs_protocol";
import {
  Connection,
  SYSVAR_RENT_PUBKEY,
  PublicKey
} from "@solana/web3.js";
import { NETWORK, PREFIX } from "../config";
import { getCreatorKeypair, writePublicKey } from "../utils";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const create_user_account = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;

    const [userAccount, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
        program.programId
    );

    try {
        const accountData = await program.account.userAccount.fetch(userAccount);
        print_user_data(accountData)
    } catch (err) {
        // initialize
        await program.rpc.initUserAccount({
            accounts: {
                userAccount,
                userAuthority: creatorKeypair.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
                rent: SYSVAR_RENT_PUBKEY,
            },
        });    
        console.log("Account has been created!");

        const accountData = await program.account.userAccount.fetch(userAccount);
        print_user_data(accountData)
    }
}

create_user_account();

const print_user_data = (userData) => {   
    console.log("===== User Data =====") 
  
    console.table([
      { "Property": "owner", "Value": userData.owner.toBase58()},
      { "Property": "borrowed_lpusd", "Value" : userData.borrowedLpusd.toString()},
      { "Property": "borrowed_lpsol", "Value": userData.borrowedLpsol.toString()},
      { "Property": "ray_amount", "Value" : userData.rayAmount.toString()},
      { "Property": "wsol_amount", "Value" : userData.wsolAmount.toString()},
      { "Property": "msol_amount", "Value" : userData.msolAmount.toString()},
      { "Property": "srm_amount", "Value" : userData.srmAmount.toString()},
      { "Property": "scnsol_amount", "Value" : userData.scnsolAmount.toString()},
      { "Property": "stsol_amount", "Value" : userData.stsolAmount.toString()},
      { "Property": "lpsol_amount", "Value" : userData.lpsolAmount.toString()},
      { "Property": "lpusd_amount", "Value" : userData.lpusdAmount.toString()},
      { "Property": "lpfi_amount", "Value" : userData.lpfiAmount.toString()},
      { "Property": "lending_ray_amount", "Value" : userData.lendingRayAmount.toString()},
      { "Property": "lending_wsol_amount", "Value" : userData.lendingWsolAmount.toString()},
      { "Property": "lending_msol_amount", "Value" : userData.lendingMsolAmount.toString()},
      { "Property": "lending_srm_amount", "Value" : userData.lendingSrmAmount.toString()},
      { "Property": "lending_scnsol_amount", "Value" : userData.lendingScnsolAmount.toString()},
      { "Property": "lending_stsol_amount", "Value" : userData.lendingStsolAmount.toString()},
    ]);
  }