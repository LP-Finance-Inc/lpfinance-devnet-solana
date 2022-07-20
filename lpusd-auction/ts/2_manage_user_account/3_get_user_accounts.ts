import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpusdAuction } from "../../target/types/lpusd_auction";
import {
  Connection,
  SYSVAR_RENT_PUBKEY,
  PublicKey
} from "@solana/web3.js";
import { NETWORK } from "../config";
import { getCreatorKeypair } from "../utils";

const { Wallet } = anchor;

const get_user_accounts = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.LpusdAuction as Program<LpusdAuction>;
    try {
        const accountData = await program.account.userAccount.all();
        
        const len = accountData.length;

        let list = []
        for(let i = 0; i < len; i++) {
            list.push({
                "user_account pubkey" : accountData[i].publicKey.toBase58(), // .substring(0,10) + '...',
                "owner": accountData[i].account.owner.toBase58().substring(0,5) + '...',
                "borrowedLpusd" : accountData[i].account.lpusdAmount.toString(),
            })
        }
        console.table(list);
    } catch (err) {
        console.log(err)
    }
}

get_user_accounts();