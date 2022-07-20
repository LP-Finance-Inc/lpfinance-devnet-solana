import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpusdAuction } from "../../target/types/lpusd_auction";
import {
  Connection,
  SYSVAR_RENT_PUBKEY,
  PublicKey
} from "@solana/web3.js";
import { NETWORK, PREFIX } from "../config";
import { getCreatorKeypair, print_user_data, writePublicKey } from "../utils";
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
