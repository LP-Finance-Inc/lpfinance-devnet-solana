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

const get_user_accounts = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;
    try {
        const accountData = await program.account.userAccount.all();
        
        const len = accountData.length;

        let list = []
        for(let i = 0; i < len; i++) {
            list.push({
                "user_account pubkey" : accountData[i].publicKey.toBase58(), // .substring(0,10) + '...',
                "owner": accountData[i].account.owner.toBase58().substring(0,5) + '...',
                "borrowedLpusd" : accountData[i].account.borrowedLpusd.toString(),
                "borrowedLpsol": accountData[i].account.borrowedLpsol.toString(),
                "rayAmount" : accountData[i].account.rayAmount.toString(),
                "wsolAmount" : accountData[i].account.wsolAmount.toString(),
                "msolAmount" : accountData[i].account.msolAmount.toString(),
                "srmAmount" : accountData[i].account.srmAmount.toString(),
                "scnsolAmount" : accountData[i].account.scnsolAmount.toString(),
                "stsolAmount" : accountData[i].account.stsolAmount.toString(),
                "lpsolAmount" : accountData[i].account.lpsolAmount.toString(),
                "lpusdAmount" : accountData[i].account.lpusdAmount.toString(),
                "lpfiAmount" : accountData[i].account.lpfiAmount.toString(),
            })
        }
        console.table(list);
    } catch (err) {
        console.log(err)
    }

    try {
        console.log(creatorKeypair.publicKey.toBase58())
        const [userAccount, bump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
            program.programId
        );
        const accountData = await program.account.userAccount.fetch(userAccount);
        console.table([
            { "Property": "owner", "Value": accountData.owner.toBase58()},
            { "Property": "step num", "Value" : accountData.stepNum.toString()},,
            { "Property": "escrowLpusd amount", "Value" : accountData.escrowLpusdAmount.toString()},
            { "Property": "borrowedLpusd", "Value": accountData.borrowedLpusd.toString()},
            { "Property": "borrowedLpsol", "Value" : accountData.borrowedLpsol.toString()},
            { "Property": "rayAmount", "Value": accountData.rayAmount.toString()},
            { "Property": "wsolAmount", "Value" : accountData.wsolAmount.toString()},
            { "Property": "msolAmount", "Value": accountData.msolAmount.toString()},
            { "Property": "srmAmount", "Value" : accountData.srmAmount.toString()},
            { "Property": "scnsolAmount", "Value": accountData.scnsolAmount.toString()},
            { "Property": "stsolAmount", "Value" : accountData.stsolAmount.toString()},
            { "Property": "lpsolAmount", "Value": accountData.lpsolAmount.toString()},
            { "Property": "lpusdAmount", "Value" : accountData.lpusdAmount.toString()},
            { "Property": "lpfiAmount", "Value" : accountData.lpfiAmount.toString()},
            { "Property": "Lending: rayAmount", "Value": accountData.lendingRayAmount.toString()},
            { "Property": "Lending: wsolAmount", "Value" : accountData.lendingWsolAmount.toString()},
            { "Property": "Lending: msolAmount", "Value": accountData.lendingMsolAmount.toString()},
            { "Property": "Lending: srmAmount", "Value" : accountData.lendingSrmAmount.toString()},
            { "Property": "Lending: scnsolAmount", "Value": accountData.lendingScnsolAmount.toString()},
            { "Property": "Lending: stsolAmount", "Value" : accountData.lendingStsolAmount.toString()}
        ]);
    } catch(err) {
        console.log(err)
    }
}

get_user_accounts();