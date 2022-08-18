import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SwapRouter } from "../target/types/swap_router";
import { SignerWallet } from "@saberhq/solana-contrib";
import { 
    TOKEN_PROGRAM_ID, 
    ASSOCIATED_TOKEN_PROGRAM_ID
} from '@solana/spl-token';
import {
  PublicKey,
  Connection,
  SYSVAR_RENT_PUBKEY,
} from "@solana/web3.js";

import {
  getKeypair,
  getCreatorKeypair,
  getPublicKey,
  writePublicKey,
  getProgramId,
  getATAPublicKey,
  writePublicKeys
} from "./utils";
import { NETWORK } from "./config";
import { TokenDataArr } from "./tokens";

const create_pools = async () => {
    
    const connection = new Connection(NETWORK, "confirmed");

    const authKeypair = getCreatorKeypair(); // getKeypair("creator");
    console.log("Creator address:", authKeypair.publicKey.toBase58());

    const provider = new SignerWallet(authKeypair).createProvider(connection);
    anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.SwapRouter as Program<SwapRouter>;

    const swap_escrow_pool_pda = await PublicKey.findProgramAddress(
        [
            Buffer.from("swap-escrow")
        ],
        program.programId
    ); 
    console.log("Swap Escrow Pool PDA address:", swap_escrow_pool_pda[0].toBase58());
    console.log("Swap Escrow Pool PDA bump:", swap_escrow_pool_pda[1]);
    
    writePublicKey(swap_escrow_pool_pda[0], 'swap_router_config');

    const tokenDataArr = TokenDataArr;
    const len = tokenDataArr.length;

    let pubkeys = "";
    for (let i = 0; i < len; i++) {
        const tokenData = tokenDataArr[i];
        const tokenAta = await getATAPublicKey(tokenData.token, swap_escrow_pool_pda[0]);
        const poolString = `const ${tokenData.key} = new PublicKey("${tokenAta.toString()}");\n`
        pubkeys += poolString;

        try {
            const accountInfo = (await connection.getTokenAccountBalance(tokenAta)).value;
            console.log(tokenData.key, accountInfo);
        } catch (err) {
            console.log(poolString)
            const result = await program.rpc.createTokenAta({
                accounts: {
                    user: authKeypair.publicKey,
                    swapPda: swap_escrow_pool_pda[0],
                    tokenSrc: tokenData.token,
                    escrowAtaLpusd: tokenAta,
                    systemProgram: anchor.web3.SystemProgram.programId,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    rent: SYSVAR_RENT_PUBKEY
                },
            });
            console.log("result : ", result);
        }
    }
    
    writePublicKeys(pubkeys, 'swap_router_pools');
};

create_pools();