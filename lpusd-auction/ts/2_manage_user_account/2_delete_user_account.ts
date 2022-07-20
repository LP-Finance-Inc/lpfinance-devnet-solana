import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpusdAuction } from "../../target/types/lpusd_auction";
import {
  Connection,
  PublicKey
} from "@solana/web3.js";
import { NETWORK, PREFIX } from "../config";
import { getCreatorKeypair } from "../utils";

const { Wallet } = anchor;

const create_user_account = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.LpusdAuction as Program<LpusdAuction>;

    const [userAccount, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
        program.programId
    );

    try {
        const accountData = await program.account.userAccount.fetch(userAccount);
        if( accountData !== undefined) {
            await program.rpc.deleteUserAccount({
                accounts: {
                    userAccount,
                    owner: creatorKeypair.publicKey
                }
            })

            console.log("Account has been deleted!");
        }
    } catch (err) {
        console.log("Account does not exist.") 
    }
}

create_user_account();