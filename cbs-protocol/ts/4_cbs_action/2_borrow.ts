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
  pythRayAccount,
  pythUsdcAccount,
  pythSolAccount,
  pythMsolAccount,
  pythSrmAccount,
  pythScnsolAccount,
  pythStsolAccount,
  LpfinanceTokenIDL,
  LpfinanceTokenPDA,
  LpfinanceTokenConfig,
  LiquidityPool,
  StableLpusdPool,
  StableLpsolPool,
  SolendConfig,
  ApricotConfig,
} from "../config";

import { convert_to_wei, getATAPublicKey, getCreatorKeypair, getPublicKey } from "../utils";
import { TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const borrow = async () => {
  const connection = new Connection(NETWORK, "confirmed");

  const creatorKeypair = getCreatorKeypair();

  anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;

  // Config
  const config = getPublicKey('cbs_config');  
  const cbsConfigData = await program.account.config.fetch(config);

  const lptokenMint= cbsConfigData.lpusdMint as PublicKey;
  const userLptoken = await getATAPublicKey(lptokenMint, creatorKeypair.publicKey);

  const [userAccount, bump] = await PublicKey.findProgramAddress(
    [Buffer.from(PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
    program.programId
  );

  const PDA = await PublicKey.findProgramAddress(
    [Buffer.from(PREFIX)],
    program.programId
  );

  const lptokenProgramId = LpfinanceTokenIDL.metadata.address;

  const borrow_wei = convert_to_wei("5");
  const borrow_amount = new anchor.BN(borrow_wei);
  
  const tx = await program.rpc.borrowLptoken(borrow_amount, {
    accounts: {
        userAuthority: creatorKeypair.publicKey,
        userAccount,
        cbsPda: PDA[0],
        config: config,
        tokensState: LpfinanceTokenPDA,
        lptokenConfig: LpfinanceTokenConfig,
        userLptoken: userLptoken,
        stableLpusdPool: StableLpusdPool,
        stableLpsolPool: StableLpsolPool,
        lptokenMint,        
        pythUsdcAccount,
        pythRayAccount,
        pythSolAccount,
        pythMsolAccount,
        pythSrmAccount,
        pythScnsolAccount,
        pythStsolAccount,
        solendConfig: SolendConfig,
        apricotConfig: ApricotConfig,
        liquidityPool: LiquidityPool,
        lptokensProgram: lptokenProgramId,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY
    },
  });

  console.log("Borrow successfully", tx)

  const cbsConfigDataAfterDeposit = await program.account.config.fetch(config);
  print_config_data(cbsConfigDataAfterDeposit)

  const userData = await program.account.userAccount.fetch(userAccount);
  print_user_data(userData)
}

borrow();

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