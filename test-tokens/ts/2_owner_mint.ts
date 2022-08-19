import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TestTokens } from "../target/types/test_tokens";
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
    const program = anchor.workspace.TestTokens as Program<TestTokens>;
    
    try {
        // Authority
        const authority = creatorKeypair.publicKey;
        // Config
        const config = getPublicKey('test_tokens_config')
        console.log("Config:", config.toBase58())

        const configData = await program.account.config.fetch(config);
        const msolMint = configData.msolMint;

        // Find PDA from `test tokens` for state account
        const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX)],
            program.programId
        );
        const userToken = await getATAPublicKey(msolMint, authority);

        const amount = 1000 * 1e9;
        // faucet usdc
        const tx = await program.rpc.ownerMintToken(new anchor.BN(amount), {
            accounts: {
                owner: authority,
                stateAccount,
                tokenMint: msolMint,
                userToken: userToken,
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