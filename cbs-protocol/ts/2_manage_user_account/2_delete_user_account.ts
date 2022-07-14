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