import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TestTokens } from "../target/types/test_tokens";
import {
  Connection,
  SYSVAR_RENT_PUBKEY,
  PublicKey,
  SystemProgram
} from "@solana/web3.js";
import { NETWORK, PREFIX, TokenArr } from "./config";
import { getATAPublicKey, getCreatorKeypair, getPublicKey, writePublicKey, writePublicKeys } from "./utils";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const convert_to_wei = (val) => (parseFloat(val) * 1e9).toString();

const airdrop_token = async () => {
    
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

        // const configData = await program.account.config.fetch(config);
        // const tokenMint = configData.wsolMint;

        // Find PDA from `test tokens` for state account
        const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX)],
            program.programId
        );

        const receiver = new PublicKey("BTu6x99R9Tay73YJ5h2p4iWtEfw2DhovHkiuL94Kafqw");
        const tokenArr = TokenArr;
        for (let i = 0; i < tokenArr.length; i++) {
            const tokenMint = tokenArr[i]
            const userToken = await getATAPublicKey(tokenMint, receiver);
    
            const amount = convert_to_wei(200);
            // faucet usdc
            const tx = await program.rpc.airdropToken(new anchor.BN(amount), {
                accounts: {
                    owner: authority,
                    receiver,
                    stateAccount,
                    tokenMint: tokenMint,
                    userToken: userToken,
                    systemProgram: SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    rent: SYSVAR_RENT_PUBKEY,
                },
            });
            console.log("Result:", tx)

        }
    } catch (err) {
        console.log(err)
    }

}

airdrop_token();