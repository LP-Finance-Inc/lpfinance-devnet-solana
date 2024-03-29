import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CbsProtocol } from "../../target/types/cbs_protocol";
import {
  Connection,
  SYSVAR_RENT_PUBKEY, PublicKey
} from "@solana/web3.js";

import { 
  NETWORK, 
  PREFIX, 
  pythUsdcAccount,
  pythSolAccount,
  StableLpusdPool,
  StableLpsolPool,
  tokenStateAccount,
  USDC_Mint,
  AuctionIDL,
  StableSwapIDL,
  TestTokenIDL,
  EscrowUSDC
} from "../config";

import { getATAPublicKey, getCreatorKeypair, getPublicKey } from "../utils";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const swap_lpsol_to_lpusd = async () => {
    const connection = new Connection(NETWORK, "confirmed");

    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;

    const AuctionConfig = getPublicKey("auction_config")
    const auctionProgram = new anchor.Program( AuctionIDL as anchor.Idl, AuctionIDL.metadata.address);
    const auctionConfigData = await auctionProgram.account.config.fetch(AuctionConfig);
    const auctionLpusd = auctionConfigData.poolLpusd as PublicKey;

    // Config
    const config = getPublicKey('cbs_config');  
    const cbsConfigData = await program.account.config.fetch(config);
    
    const PDA = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX)],
        program.programId
    );    
    
    const [userAccount, bump] = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
        program.programId
    );

    const StableSwapProgramId = new PublicKey(StableSwapIDL.metadata.address);
    const TestTokenProgramId = new PublicKey(TestTokenIDL.metadata.address);

    const stableswapPoolAtaLpusd = await getATAPublicKey(cbsConfigData.lpusdMint, StableLpusdPool);
    const stableswapPoolAtaUsdc = await getATAPublicKey(USDC_Mint, StableLpusdPool);
    const stableswapPoolAtaLpsol = await getATAPublicKey(cbsConfigData.lpsolMint, StableLpsolPool);
    const stableswapPoolAtaWsol = await getATAPublicKey(cbsConfigData.wsolMint, StableLpsolPool);

    console.log("UserAccount:", userAccount.toBase58())    

    try {
      const userAccountData = await program.account.userAccount.fetch(userAccount);
      if (userAccountData.stepNum == 4) {
        if (userAccountData.rayAmount.toString() != "0" ||
          userAccountData.wsolAmount.toString() != "0" ||
          userAccountData.msolAmount.toString() != "0" ||
          userAccountData.scnsolAmount.toString() != "0" ||
          userAccountData.srmAmount.toString() != "0" ||
          userAccountData.stsolAmount.toString() != "0"
        ) {
          console.log("You need to liquidate the normal tokens first");
        }

        const tx = await program.rpc.liquidateSwapLpsoltoken1({
            accounts: {
                userAccount: userAccount,
                cbsPda: PDA[0],
                config,
                stableSwapPool: StableLpsolPool,
                tokenStateAccount,

                pythWsol: pythSolAccount,
                pythUsdc: pythUsdcAccount,

                tokenWsol: cbsConfigData.wsolMint,
                tokenLpsol: cbsConfigData.lpsolMint,
                tokenUsdc: USDC_Mint,

                cbsAtaWsol: cbsConfigData.poolWsol,
                cbsAtaUsdc: EscrowUSDC,
                cbsAtaLpsol: cbsConfigData.poolLpsol,

                stableswapPoolAtaLpsol,
                stableswapPoolAtaWsol,

                stableswapProgram: StableSwapProgramId,
                testtokensProgram: TestTokenProgramId,
                systemProgram: anchor.web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                rent: SYSVAR_RENT_PUBKEY,
            },
        });
        console.log("Liquidate lpsol->wsol->usdc successfully", tx)
      }
      if (userAccountData.stepNum == 5) {
        const tx2 = await program.rpc.liquidateSwapLpsoltoken2({
          accounts: {
              userAccount: userAccount,
              cbsPda: PDA[0],
              stableSwapPool: StableLpusdPool,

              tokenUsdc: USDC_Mint,
              tokenLpusd: cbsConfigData.lpusdMint,

              cbsAtaUsdc: EscrowUSDC,
              cbsAtaLpusd: cbsConfigData.poolLpusd, 

              auctionLpusd: auctionLpusd,

              stableswapPoolAtaLpusd,
              stableswapPoolAtaUsdc,

              stableswapProgram: StableSwapProgramId,
              systemProgram: anchor.web3.SystemProgram.programId,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
              rent: SYSVAR_RENT_PUBKEY,
          },
        });
        console.log("Liquidate lpsol->lpusd successfully", tx2)
      }
    } catch (e) {
        console.log("Failed", e);
    }
    const userData = await program.account.userAccount.fetch(userAccount);
    print_user_data(userData)
    const cbsConfigDataAfterDeposit = await program.account.config.fetch(config);
    print_config_data(cbsConfigDataAfterDeposit)
}

swap_lpsol_to_lpusd();

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
    { "Property": "step num", "Value" : userData.stepNum.toString()},
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
    { "Property": "step num", "Value" : userData.stepNum.toString()},
  ]);
}