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
import { getATAPublicKey, getCreatorKeypair, writePublicKey, writePublicKeys } from "./utils";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const lpsol_mint = "lpsol_mint";
const lpusd_mint = "lpusd_mint";
const lpdao_mint = "lpdao_mint";

const initialize_config = async () => {
    return;
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.LpfinanceTokens as Program<LpfinanceTokens>;
    
    try {

        const authority = creatorKeypair.publicKey;

        // Config
        const configKeypair = anchor.web3.Keypair.generate();
        writePublicKey(configKeypair.publicKey, 'lpfinance_tokens_config')

        // Find PDA from `cbs protocol` for state account
        const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX)],
            program.programId
        );
        writePublicKey(stateAccount, 'lpfinance_tokens_pda')
        
        let pubkeys = "";

        const [lpsolMint, lpsolMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(lpsol_mint)],
            program.programId
        );

        pubkeys += `export const LpSOL_MINT = new PublicKey("${lpsolMint.toBase58()}")\n`;
        
        const [lpusdMint, lpusdMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(lpusd_mint)],
            program.programId
        );
        pubkeys += `export const LpUSD_MINT = new PublicKey("${lpusdMint.toBase58()}")\n`;
        
        // DAO Token name is LpFI
        const [lpdaoMint, lpdaoMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(lpdao_mint)],
            program.programId
        );
        pubkeys += `export const LpFI_MINT = new PublicKey("${lpdaoMint.toBase58()}")\n`;

        const userDaoToken = await getATAPublicKey(
            lpdaoMint, 
            authority,
        )
       
        writePublicKeys(pubkeys, "lpfinance_tokens");
    
        // create tokens
        await program.rpc.initialize({
            accounts: {
                authority,
                stateAccount,
                config: configKeypair.publicKey,
                lpsolMint,
                lpusdMint,
                lpdaoMint,
                userDaotoken: userDaoToken,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
            },
            signers: [configKeypair]
        });
    } catch (err) {
        console.log(err)
    }

}

initialize_config();