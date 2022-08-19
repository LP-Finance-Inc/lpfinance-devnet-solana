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
import { getCreatorKeypair, writePublicKey, writePublicKeys } from "./utils";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const wsol_mint = "wsol_mint";
const msol_mint = "msol_mint";
const stsol_mint = "stsol_mint";
const scnsol_mint = "scnsol_mint";
const usdc_mint = "usdc_mint";
const btc_mint = "btc_mint";
const eth_mint = "eth_mint";
const ray_mint = "ray_mint";
const srm_mint = "srm_mint";
const avax_mint = "avax_mint";
const fida_mint = "fida_mint";
const ftt_mint = "ftt_mint";
const ftm_mint = "ftm_mint";
const gmt_mint = "gmt_mint";
const luna_mint = "luna_mint";
const matic_mint = "matic_mint";
const usdt_mint = "usdt_mint";

const initialize_config = async () => {
    return;
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.TestTokens as Program<TestTokens>;
    
    try {
        // Config
        const configKeypair = anchor.web3.Keypair.generate();
        writePublicKey(configKeypair.publicKey, 'test_tokens_config')

        // Find PDA from `cbs protocol` for state account
        const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX)],
            program.programId
        );
        writePublicKey(stateAccount, 'test_tokens_pda')
        
        let pubkeys = "";

        const [wsolMint, wsolMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(wsol_mint)],
            program.programId
        );
        pubkeys += `export const WSOL_MINT = new PublicKey("${wsolMint.toBase58()}")\n`;
    
        const [msolMint, msolMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(msol_mint)],
            program.programId
        );
        // bumps.poolMsol = poolMsolBump;
        pubkeys += `export const MSOL_MINT = new PublicKey("${msolMint.toBase58()}")\n`;
    
        const [stsolMint, stsolMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(stsol_mint)],
            program.programId
        );
        pubkeys += `export const STSOL_MINT = new PublicKey("${stsolMint.toBase58()}")\n`;
    
        const [scnsolMint, scnsolMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(scnsol_mint)],
            program.programId
        );
        pubkeys += `export const SCNSOL_MINT = new PublicKey("${scnsolMint.toBase58()}")\n`;
    
        const [usdcMint, usdcMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(usdc_mint)],
            program.programId
        );
        pubkeys += `export const USDC_MINT = new PublicKey("${usdcMint.toBase58()}")\n`;
    
        const [btcMint, btcMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(btc_mint)],
            program.programId
        );
        pubkeys += `export const BTC_MINT = new PublicKey("${btcMint.toBase58()}")\n`;
    
        const [ethMint, ethMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(eth_mint)],
            program.programId
        );
        pubkeys += `export const ETH_MINT = new PublicKey("${ethMint.toBase58()}")\n`;
    
        const [rayMint, rayMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(ray_mint)],
            program.programId
        );
        pubkeys += `export const RAY_MINT = new PublicKey("${rayMint.toBase58()}")\n`;
    
        const [srmMint, srmMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(srm_mint)],
            program.programId
        );
        pubkeys += `export const SRM_MINT = new PublicKey("${srmMint.toBase58()}")\n`;
    
        const [avaxMint, avaxMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(avax_mint)],
            program.programId
        );
        pubkeys += `export const AVAX_MINT = new PublicKey("${avaxMint.toBase58()}")\n`;
    
        const [fidaMint, fidaMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(fida_mint)],
            program.programId
        );
        pubkeys += `export const FIDA_MINT = new PublicKey("${fidaMint.toBase58()}")\n`;
    
        const [fttMint, fttMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(ftt_mint)],
            program.programId
        );
        pubkeys += `export const FTT_MINT = new PublicKey("${fttMint.toBase58()}")\n`;
    
        const [ftmMint, ftmMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(ftm_mint)],
            program.programId
        );
        pubkeys += `export const FTM_MINT = new PublicKey("${ftmMint.toBase58()}")\n`;
    
        const [gmtMint, gmtMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(gmt_mint)],
            program.programId
        );
        pubkeys += `export const GMT_MINT = new PublicKey("${gmtMint.toBase58()}")\n`;
    
        const [lunaMint, lunaMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(luna_mint)],
            program.programId
        );
        pubkeys += `export const LUNA_MINT = new PublicKey("${lunaMint.toBase58()}")\n`;
    
        const [maticMint, maticMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(matic_mint)],
            program.programId
        );
        pubkeys += `export const MATIC_MINT = new PublicKey("${maticMint.toBase58()}")\n`;
    
        const [usdtMint, usdtMintBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(usdt_mint)],
            program.programId
        );
        pubkeys += `export const USDT_MINT = new PublicKey("${usdtMint.toBase58()}")\n`;
        writePublicKeys(pubkeys, "test_tokens");
    
        const authority = creatorKeypair.publicKey;

        // create tokens
        await program.rpc.createToken1({
            accounts: {
                authority,
                stateAccount,
                config: configKeypair.publicKey,
                wsolMint,
                msolMint,
                stsolMint,
                scnsolMint,
                usdcMint,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
            },
            signers: [configKeypair]
        });

        await program.rpc.createToken2({
            accounts: {
                authority,
                stateAccount,
                config: configKeypair.publicKey,
                btcMint,
                ethMint,
                rayMint,
                srmMint,
                avaxMint,
                fidaMint,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
            }
        });

        await program.rpc.createToken3({
            accounts: {
                authority,
                stateAccount,
                config: configKeypair.publicKey,
                fttMint,
                ftmMint,
                gmtMint,
                lunaMint,
                maticMint,
                usdtMint,
                systemProgram: SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
            }
        });
    } catch (err) {
        console.log(err)
    }

}

initialize_config();