import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CbsProtocol } from "../../target/types/cbs_protocol";
import {
  Connection,
  SYSVAR_RENT_PUBKEY, PublicKey, SystemProgram
} from "@solana/web3.js";

import { 
    NETWORK, 
    PREFIX, 
    StableLpsolPool, 
    StableSwapIDL, 
    SwapRouterIDL,
} from "../config";

import { convert_to_wei, getATAPublicKey, getCreatorKeypair, getPublicKey } from "../utils";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const repay_wsol = async () => {
    const connection = new Connection(NETWORK, "confirmed");

    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;
    // Config
    const config = getPublicKey('cbs_config');  
    const cbsConfigData = await program.account.config.fetch(config);

    const tokenSrc= cbsConfigData.wsolMint as PublicKey;
    const tokenDest= cbsConfigData.lpsolMint as PublicKey;    
    const cbsAtaSrc = cbsConfigData.poolWsol as PublicKey;    
    const cbsAtaDest = cbsConfigData.poolLpsol as PublicKey;    

    const swapAtaSrc = await getATAPublicKey(tokenSrc, StableLpsolPool);
    const swapAtaDest = await getATAPublicKey(tokenDest, StableLpsolPool);


    const userAtaSrc = await getATAPublicKey(tokenSrc, creatorKeypair.publicKey);

    const [userAccount, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
        program.programId
    );

    // cbs pda
    const PDA = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX)],
        program.programId
    );

    // ====== swap router =====
    const swapRouterProgramId = new PublicKey(SwapRouterIDL.metadata.address);
    const stableswapProgramId = new PublicKey(StableSwapIDL.metadata.address);
    const swap_escrow_pool_pda = await PublicKey.findProgramAddress(
        [
            Buffer.from("swap-escrow"),
            PDA[0].toBuffer()
        ],
        swapRouterProgramId
    ); 

    const escrowAtaSrc = await getATAPublicKey(tokenSrc, swap_escrow_pool_pda[0]);
    const escrowAtaDest = await getATAPublicKey(tokenDest, swap_escrow_pool_pda[0]);

    const repay_wei = convert_to_wei("0.1");
    const repay_amount = new anchor.BN(repay_wei);
    console.log(
        "creatorKeypair" , creatorKeypair.publicKey.toBase58(), "\n",
        "userAccount" , userAccount.toBase58(), "\n",
        "config" , config.toBase58(), "\n",
        "swap_escrow_pool_pda" , swap_escrow_pool_pda[0].toBase58(), "\n",
        "StableLpsolPool" , StableLpsolPool.toBase58(), "\n",
        "tokenSrc" , tokenSrc.toBase58(), "\n",
        "tokenDest" , tokenDest.toBase58(), "\n",
        "userAtaSrc" , userAtaSrc.toBase58(), "\n",
        "cbsAtaSrc" , cbsAtaSrc.toBase58(), "\n",
        "cbsAtaDest" , cbsAtaDest.toBase58(), "\n",
        "swapAtaSrc" , swapAtaSrc.toBase58(), "\n",
        "swapAtaDest" , swapAtaDest.toBase58(), "\n",
        "escrowAtaSrc" , escrowAtaSrc.toBase58(), "\n",
        "escrowAtaDest" , escrowAtaDest.toBase58(), "\n",
        "PDA" , PDA[0].toBase58(), "\n",
        "swapRouterProgramId" , swapRouterProgramId.toBase58(), "\n",
        "stableswapProgramId" , stableswapProgramId.toBase58()
    )

    if (tokenSrc == cbsConfigData.wsolMint) {
        const tx = await program.rpc.repayWsol(repay_amount, {
            accounts: {
                userAuthority: creatorKeypair.publicKey,
                userAccount,
                config: config,
                swapEscrow: swap_escrow_pool_pda[0],
                stableSwapPool: StableLpsolPool,
                tokenSrc,
                tokenDest,
                userAtaSrc,
                cbsAtaSrc,
                cbsAtaDest,
                swapAtaSrc,
                swapAtaDest,
                escrowAtaSrc,
                escrowAtaDest,
                cbsPda: PDA[0],
                swapProgram: swapRouterProgramId,
                stableswapProgram: stableswapProgramId,       
                systemProgram: SystemProgram.programId,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                tokenProgram: TOKEN_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY
            },
        });
        console.log("repay successfully: ", tx);
    }


    const cbsConfigDataAfterDeposit = await program.account.config.fetch(config);
    print_config_data(cbsConfigDataAfterDeposit)

    const userData = await program.account.userAccount.fetch(userAccount);
    print_user_data(userData)
}

repay_wsol();

const print_config_data = (configData) => {     
  console.log("===== Config Data =====") 
  
  console.table([
    { "Property": "borrowed_lpusd", "Value" : configData.totalBorrowedLpusd.toString()},
    { "Property": "borrowed_lpsol", "Value": configData.totalBorrowedLpsol.toString()},
    { "Property": "deposited_wsol", "Value" : configData.totalDepositedWsol.toString()},
    { "Property": "deposited_ray", "Value": configData.totalDepositedRay.toString()},
    { "Property": "deposited_msol", "Value" : configData.totalDepositedMsol.toString()},
    { "Property": "deposited_srm", "Value" : configData.totalDepositedSrm.toString()},
    { "Property": "deposited_scnsol", "Value" : configData.totalDepositedScnsol.toString()},
    { "Property": "deposited_stsol", "Value" : configData.totalDepositedStsol.toString()},
    { "Property": "deposited_lpsol", "Value" : configData.totalDepositedLpsol.toString()},
    { "Property": "deposited_lpusd", "Value" : configData.totalDepositedLpusd.toString()},
    { "Property": "deposited_lpfi", "Value" : configData.totalDepositedLpfi.toString()},
    { "Property": "liquidation_run", "Value" : configData.liquidationRun.toString()},
  ]);
}

const print_user_data = (userData) => {   
  console.log("===== User Data =====") 

  console.table([
    { "Property": "owner", "Value": userData.owner.toBase58()},
    { "Property": "borrowed_lpusd", "Value" : userData.borrowedLpusd.toString()},
    { "Property": "borrowed_lpsol", "Value": userData.borrowedLpsol.toString()},
    { "Property": "ray_amount", "Value" : userData.rayAmount.toString()},
    { "Property": "wsol_amount", "Value" : userData.wsolAmount.toString()},
    { "Property": "msol_amount", "Value" : userData.msolAmount.toString()},
    { "Property": "srm_amount", "Value" : userData.srmAmount.toString()},
    { "Property": "scnsol_amount", "Value" : userData.scnsolAmount.toString()},
    { "Property": "stsol_amount", "Value" : userData.stsolAmount.toString()},
    { "Property": "lpsol_amount", "Value" : userData.lpsolAmount.toString()},
    { "Property": "lpusd_amount", "Value" : userData.lpusdAmount.toString()},
    { "Property": "lpfi_amount", "Value" : userData.lpfiAmount.toString()},
    { "Property": "lending_ray_amount", "Value" : userData.lendingRayAmount.toString()},
    { "Property": "lending_wsol_amount", "Value" : userData.lendingWsolAmount.toString()},
    { "Property": "lending_msol_amount", "Value" : userData.lendingMsolAmount.toString()},
    { "Property": "lending_srm_amount", "Value" : userData.lendingSrmAmount.toString()},
    { "Property": "lending_scnsol_amount", "Value" : userData.lendingScnsolAmount.toString()},
    { "Property": "lending_stsol_amount", "Value" : userData.lendingStsolAmount.toString()},
  ]);
}