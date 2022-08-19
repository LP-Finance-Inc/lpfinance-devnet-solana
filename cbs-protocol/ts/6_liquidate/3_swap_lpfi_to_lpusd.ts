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
  USDCMint,
  AuctionIDL,
  StableSwapIDL,
  TestTokenIDL,
  EscrowUSDC,
  LiquidityPool,
  UniswapIDL
} from "../config";

import { getATAPublicKey, getCreatorKeypair, getPublicKey } from "../utils";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const swap_lpfi_to_lpusd = async () => {
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
    const UniswapProgramId = new PublicKey(UniswapIDL.metadata.address);

    const stableswapPoolAtaLpusd = await getATAPublicKey(cbsConfigData.lpusdMint, StableLpusdPool);
    const stableswapPoolAtaUsdc = await getATAPublicKey(USDCMint, StableLpusdPool);
        
    // LpFI <-> USDC
    const UniswapPool = getPublicKey('lpfi-usdc-pool');
    const uniswapPoolAtaLpfi = await getATAPublicKey(
        cbsConfigData.lpfiMint,
        UniswapPool
    );
      
    const uniswapPoolAtaUsdc = await getATAPublicKey(
        USDCMint,
        UniswapPool
    );
       
    const userAccountData = await program.account.userAccount.fetch(userAccount);
    try {
      if (userAccountData.stepNum == 6) {
        const tx = await program.rpc.liquidateSwapLpfitoken({
          accounts: {
              userAccount: userAccount,
              cbsPda: PDA[0],
              stableSwapPool: StableLpusdPool,
              uniswapPool: LiquidityPool,

              tokenLpfi: cbsConfigData.lpfiMint,
              tokenUsdc: USDCMint,
              tokenLpusd: cbsConfigData.lpusdMint,

              escrowAtaUsdc: EscrowUSDC,
              cbsAtaLpusd: cbsConfigData.poolLpusd, 
              cbsAtaLpfi: cbsConfigData.poolLpfi,

              auctionLpusd: auctionLpusd,

              uniswapPoolAtaLpfi,
              uniswapPoolAtaUsdc,
              stableswapPoolAtaLpusd,
              stableswapPoolAtaUsdc,

              stableswapProgram: StableSwapProgramId,
              uniswapProgram: UniswapProgramId,
              systemProgram: anchor.web3.SystemProgram.programId,
              tokenProgram: TOKEN_PROGRAM_ID,
              associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
              rent: SYSVAR_RENT_PUBKEY,
          },
        });
        console.log("Liquidate lpfi->lpusd successfully", tx)
      } else {
        console.log("You already passed LpFI liquidation step");
      }
        
    } catch (e) {
        console.log("Failed", e);
    }
    const userData = await program.account.userAccount.fetch(userAccount);
    print_user_data(userData)
    const cbsConfigDataAfterDeposit = await program.account.config.fetch(config);
    print_config_data(cbsConfigDataAfterDeposit)
}

swap_lpfi_to_lpusd();

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
    { "Property": "step num", "Value" : userData.stepNum.toString()},
  ]);
}