import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Solend } from "../target/types/solend";
import {
  Connection,
  SYSVAR_RENT_PUBKEY,
  PublicKey,
  SystemProgram
} from "@solana/web3.js";
import { NETWORK, PREFIX } from "./config";
import { getCreatorKeypair, writePublicKey, writePublicKeys } from "./utils";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { MSOL_Mint, Ray_Mint, scnSOL_Mint, SRM_Mint, stsol_Mint, wSOL_Mint } from "./tokens";

const { Wallet } = anchor;

const pool_ray = "pool_ray";
const pool_wsol = "pool_wsol";
const pool_msol = "pool_msol";
const pool_srm = "pool_srm";
const pool_scnsol = "pool_scnsol";
const pool_stsol = "pool_stsol";

const initialize_config = async () => {
    return;
    const connection = new Connection(NETWORK, "confirmed");
  
    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.Solend as Program<Solend>;
    
    try {
        // Config
        const configKeypair = anchor.web3.Keypair.generate();
        writePublicKey(configKeypair.publicKey, 'solend_config')

        // Find PDA from `cbs protocol` for state account
        const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX)],
            program.programId
        );
        writePublicKey(stateAccount, 'solend_pda')
        
        let pubkeys = "";

        // Find PDA for `RAY pool`
        const [poolRay, poolRayBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(pool_ray)],
            program.programId
        );
        pubkeys += `export const RAY_Solend_ATA = new PublicKey("${poolRay.toBase58()}")\n`;
    
        // Find PDA for `wSOL pool`
        const [poolWsol, poolWsolBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(pool_wsol)],
            program.programId
        );
        pubkeys += `export const WSOL_Solend_ATA = new PublicKey("${poolWsol.toBase58()}")\n`;    
        
        // Find PDA for `msol pool`
        const [poolMsol, poolMsolBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(pool_msol)],
            program.programId
        );
        pubkeys += `export const MSOL_Solend_ATA = new PublicKey("${poolMsol.toBase58()}")\n`;
    
        // Find PDA for `srm pool`
        const [poolSrm, poolSrmBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(pool_srm)],
            program.programId
        );
        pubkeys += `export const SRM_Solend_ATA = new PublicKey("${poolSrm.toBase58()}")\n`;
    
        // Find PDA for `scnsol pool`
        const [poolScnsol, poolScnsolBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(pool_scnsol)],
            program.programId
        );
        pubkeys += `export const SCNSOL_Solend_ATA = new PublicKey("${poolScnsol.toBase58()}")\n`;
    
        // Find PDA for `stsol pool`
        const [poolStsol, poolStsolBump] = await PublicKey.findProgramAddress(
            [Buffer.from(PREFIX), Buffer.from(pool_stsol)],
            program.programId
        );
        pubkeys += `export const STSOL_Solend_ATA = new PublicKey("${poolStsol.toBase58()}")\n`;

        writePublicKeys(pubkeys, "solend_pools");
    
        const authority = creatorKeypair.publicKey;

        // create tokens
        // initialize
        await program.rpc.initialize({
            accounts: {
            authority,
            stateAccount,
            config: configKeypair.publicKey,
            rayMint: Ray_Mint,
            wsolMint: wSOL_Mint,
            msolMint: MSOL_Mint,
            srmMint: SRM_Mint,
            scnsolMint: scnSOL_Mint,
            stsolMint: stsol_Mint,
            poolWsol,
            poolRay,
            poolMsol,
            poolSrm,
            poolScnsol,
            poolStsol,
            systemProgram: SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            rent: SYSVAR_RENT_PUBKEY,
            },
            signers: [configKeypair]
        });
    } catch (err) {
        console.log(err)
    }

}

initialize_config();