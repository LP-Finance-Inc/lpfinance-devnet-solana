import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpfinanceTokens } from "../target/types/lpfinance_tokens";
import {
  Connection,
  SYSVAR_RENT_PUBKEY,
  PublicKey,
  SystemProgram
} from "@solana/web3.js";
import { NETWORK, PREFIX } from "./config";
import { getATAPublicKey, getCreatorKeypair, getPublicKey, writePublicKey, writePublicKeys } from "./utils";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const owner_mint = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.LpfinanceTokens as Program<LpfinanceTokens>;
    
    try {
        // Authority
        const authority = creatorKeypair.publicKey;
        // Config
        const config = getPublicKey('lpfinance_tokens_config')

        console.log("Config:", config.toBase58(), authority.toBase58())
        const configData = await program.account.config.fetch(config);
        const lptoken = configData.lpsolMint;

        // Find PDA from `test tokens` for state account
        const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX)],
            program.programId
        );
        const userLpToken = await getATAPublicKey(lptoken, authority);

        const amount = 1 * 1e9;
        // faucet usdc
        const tx = await program.rpc.ownerMintLptoken(new anchor.BN(amount), {
            accounts: {
                owner: authority,
                stateAccount,
                lptokenMint: lptoken,
                userLptoken: userLpToken,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
            },
        });
        console.log("Result:", tx)
    } catch (err) {
        console.log(err)
    }

}

owner_mint();