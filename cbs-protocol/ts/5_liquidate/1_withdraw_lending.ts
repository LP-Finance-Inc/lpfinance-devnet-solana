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
  SolendIDL,
  SolendConfig,
  ApricotConfig,
  ApricotIDL,
  solendPool,
  apricotPool,
  StableLpsolPool,
  StableLpusdPool,
  LiquidityPool,
  SolendStateAccount,
  ApricotStateAccount
} from "../config";

import { convert_to_wei, getATAPublicKey, getCreatorKeypair, getPublicKey } from "../utils";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const withdraw_lending_ctokens = async () => {
    const connection = new Connection(NETWORK, "confirmed");

    const creatorKeypair = getCreatorKeypair();

    anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
    const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;

    // Config
    const config = getPublicKey('cbs_config');  
    const cbsConfigData = await program.account.config.fetch(config);
    const solendAccount = getPublicKey('cbs_solend_account')
    const apricotAccount = getPublicKey('cbs_apricot_account')

    const PDA = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX)],
        program.programId
    );    

    const solendProgramId = new PublicKey(SolendIDL.metadata.address) 

    const apricotProgramId = new PublicKey(ApricotIDL.metadata.address) 

    let tokenDatas = [];
    tokenDatas.push({
        destMint: cbsConfigData.rayMint,    cbsPool: cbsConfigData.poolRay
    })
    tokenDatas.push({
        destMint: cbsConfigData.wsolMint,    cbsPool: cbsConfigData.poolWsol
    })
    tokenDatas.push({
        destMint: cbsConfigData.msolMint,    cbsPool: cbsConfigData.poolMsol
    })
    tokenDatas.push({
        destMint: cbsConfigData.srmMint,    cbsPool: cbsConfigData.poolSrm
    })
    tokenDatas.push({
        destMint: cbsConfigData.scnsolMint,    cbsPool: cbsConfigData.poolScnsol
    })
    tokenDatas.push({
        destMint: cbsConfigData.stsolMint,    cbsPool: cbsConfigData.poolStsol
    })

    const accountData = await program.account.userAccount.all();        
    const len = accountData.length;
    for(let j = 0; j < len; j++) {
        const userAccount = accountData[j].publicKey;
        try {
            const userData = await program.views.getLtv({
                accounts: {
                    userAccount,
                    stableLpsolPool: StableLpsolPool,
                    stableLpusdPool: StableLpusdPool,
                    pythUsdcAccount,
                    pythRayAccount,
                    pythSolAccount,
                    pythMsolAccount,
                    pythSrmAccount,
                    pythScnsolAccount,
                    pythStsolAccount,
                    liquidityPool: LiquidityPool,
                    solendConfig: SolendConfig,
                    apricotConfig: ApricotConfig,
                }
            });
            const LTV = userData[0];
            const R_TOTAL = userData[1];
            const R_BORROW = userData[2];
            console.log("LTV:", Number(LTV), Number(R_TOTAL), Number(R_BORROW));

            if (Number(LTV) > 90) {
                for (let i = 0; i < tokenDatas.length; i++) {
                    let tokenData = tokenDatas[i];
                    const tx = await program.rpc.withdrawLending({
                        accounts: {
                            userAccount,
                            cbsPda: PDA[0],
                            config,
                            stableLpsolPool: StableLpsolPool,
                            stableLpusdPool: StableLpusdPool,
                            pythUsdcAccount,
                            pythRayAccount,
                            pythSolAccount,
                            pythMsolAccount,
                            pythSrmAccount,
                            pythScnsolAccount,
                            pythStsolAccount,
                            liquidityPool: LiquidityPool,
                            destMint: tokenData.destMint,
                            cbsPool: tokenData.cbsPool,
                            solendConfig: SolendConfig,
                            solendAccount,
                            solendPool,
                            solendStateAccount: SolendStateAccount,
                            apricotConfig: ApricotConfig,
                            apricotAccount,
                            apricotPool,
                            apricotStateAccount: ApricotStateAccount,
                            solendProgram: solendProgramId,
                            apricotProgram: apricotProgramId,
                            systemProgram: anchor.web3.SystemProgram.programId,
                            tokenProgram: TOKEN_PROGRAM_ID,
                            rent: SYSVAR_RENT_PUBKEY,
                        },
                    });
                    console.log("Deposit successfully", tx)
                }
                const userData = await program.account.userAccount.fetch(userAccount);
                print_user_data(userData)
            }
        } catch (e) {
            console.log("Failed", e)
        }
    }

    const cbsConfigDataAfterDeposit = await program.account.config.fetch(config);
    print_config_data(cbsConfigDataAfterDeposit)
}

withdraw_lending_ctokens();

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