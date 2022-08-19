import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpusdAuction } from "../../target/types/lpusd_auction";
import {
  Connection,
  SYSVAR_RENT_PUBKEY, PublicKey
} from "@solana/web3.js";

import { 
    CBSProtocolIDL,
    CBS_PREFIX,
    NETWORK, 
    AUCTION_PREFIX,
    StableLpsolPool,
    StableLpusdPool,
    pythRayAccount,
    pythUsdcAccount,
    pythSolAccount,
    pythMsolAccount,
    pythSrmAccount,
    pythScnsolAccount,
    pythStsolAccount,
    LiquidityPool,
    LpfinanceTokenIDL,
    StableSwapIDL,
    TestTokenIDL,
    SolendConfig,
    ApricotConfig
} from "../config";

import { getATAPublicKey, getCreatorKeypair, getPublicKey, print_config_data, print_user_data } from "../utils";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const liquidate = async () => {
  const connection = new Connection(NETWORK, "confirmed");

  const creatorKeypair = getCreatorKeypair();

  anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpusdAuction as Program<LpusdAuction>;
  // Config
  const config = getPublicKey('auction_config');  
  const auctionConfigData = await program.account.config.fetch(config);

  const stableswapProgramId = new PublicKey(StableSwapIDL.metadata.address);
  const testTokenProgramId = new PublicKey(TestTokenIDL.metadata.address);
  
  const PDA = await PublicKey.findProgramAddress(
    [Buffer.from(AUCTION_PREFIX)],
    program.programId
  );

  const lpusdMint= auctionConfigData.lpusdMint as PublicKey;
  const lpusdAta = auctionConfigData.poolLpusd as PublicKey;
  const lpsolMint= auctionConfigData.lpsolMint as PublicKey;
  const lpsolAta = auctionConfigData.poolLpsol as PublicKey;

  const usdcMint= auctionConfigData.usdcMint as PublicKey;
  const usdcAta = auctionConfigData.poolUsdc as PublicKey;
  const wsolMint= auctionConfigData.wsolMint as PublicKey;
  const wsolAta = auctionConfigData.poolWsol as PublicKey;


  console.log(
    "PDA:", 
    lpusdAta.toBase58(),
    lpsolAta.toBase58(),
    usdcAta.toBase58(),
    wsolAta.toBase58(),
  );

  const [userAccount, bump] = await PublicKey.findProgramAddress(
    [Buffer.from(AUCTION_PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
    program.programId
  );

  const cbsProgramId = new PublicKey(CBSProtocolIDL.metadata.address);
  const cbsProgram = new anchor.Program(CBSProtocolIDL as anchor.Idl, CBSProtocolIDL.metadata.address);

  const [cbsAccount, cbsBump] = await PublicKey.findProgramAddress(
    [Buffer.from(CBS_PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
    cbsProgramId
  );
  
  const cbsAccountData = await cbsProgram.account.userAccount.fetch(cbsAccount);
  if (cbsAccountData.stepNum == 0) {

    if (cbsAccountData.lendingRayAmount.toString() != "0" ||
      cbsAccountData.lendingWsolAmount.toString() != "0" ||
      cbsAccountData.lendingMsolAmount.toString() != "0" ||
      cbsAccountData.lendingSrmAmount.toString() != "0" ||
      cbsAccountData.lendingScnsolAmount.toString() != "0" ||
      cbsAccountData.lendingStsolAmount.toString() != "0"
    ) {
      console.log("You should withdraw Lending amount to avoid overflow collaterals");
      return;
    }

    const userData = await cbsProgram.views.getLtv({
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
    if (Number(LTV) < 94) {
      console.log("You cannot Liquidate");
      return;
    }

    // STEP: 1
    await program.rpc.burnLpusdLiquidate({
      accounts: {
        userAuthority: creatorKeypair.publicKey,
        owner: creatorKeypair.publicKey,
        userAccount,
        auctionPda: PDA[0],
        config: config,
        cbsAccount,
        lpusdMint,
        lpusdAta,
        lpsolMint,
        lpsolAta,
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
        cbsProgram: cbsProgramId,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY
      },
    });
  }

  const stableswapPoolAtaLpsol = await getATAPublicKey(lpsolMint, StableLpsolPool);
  const stableswapPoolAtaLpusd = await getATAPublicKey(lpusdMint, StableLpusdPool);
  const stableswapPoolAtaUsdc = await getATAPublicKey(usdcMint, StableLpusdPool);
  const stableswapPoolAtaWsol = await getATAPublicKey(wsolMint, StableLpsolPool);
  
  const tokenStateAccount = new PublicKey("FEL9EygF1C3d5cwD2ZXkpmaQMBtdxKd1mvYRrD81KNVY");
  
  console.log("UserAccount:", userAccount.toBase58())

  // STEP: 2
  if (cbsAccountData.stepNum == 1) {
    const tx2 = await program.rpc.burnLpsolLiquidate1({
      accounts: {
        owner: creatorKeypair.publicKey,
        userAccount,
        cbsAccount,
        auctionPda: PDA[0],
        stableLpsolPool: StableLpsolPool,
        stableLpusdPool: StableLpusdPool,
        tokenStateAccount,
        tokenLpusd: lpusdMint,
        tokenUsdc: usdcMint,
        tokenWsol: wsolMint,
        pythUsdc: pythUsdcAccount,
        pythWsol: pythSolAccount,
        auctionAtaLpusd: lpusdAta,
        auctionAtaUsdc: usdcAta,
        auctionAtaWsol: wsolAta,
        stableswapPoolAtaLpusd: stableswapPoolAtaLpusd,
        stableswapPoolAtaUsdc: stableswapPoolAtaUsdc,
        testtokensProgram: testTokenProgramId,
        stableswapProgram: stableswapProgramId,
        cbsProgram: cbsProgramId,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY
      },
    });

    console.log("Burn usdc to wSOL", tx2)
  }

  // STEP: 3
  if (cbsAccountData.stepNum == 2) {
    const tx3 = await program.rpc.burnLpsolLiquidate2({
      accounts: {
        owner: creatorKeypair.publicKey,
        userAccount,
        cbsAccount,
        auctionPda: PDA[0],
        stableLpsolPool: StableLpsolPool,
        tokenLpsol: lpsolMint,
        tokenWsol: wsolMint,
        auctionAtaLpsol: lpsolAta,
        auctionAtaWsol: wsolAta,
        stableswapPoolAtaLpsol: stableswapPoolAtaLpsol,
        stableswapPoolAtaWsol: stableswapPoolAtaWsol,
        stableswapProgram: stableswapProgramId,
        cbsProgram: cbsProgramId,
        systemProgram: anchor.web3.SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY
      },
    });

    console.log("Burn LpSOL successfully", tx3)
  }

  const auctionConfigDataAfterDeposit = await program.account.config.fetch(config);
  print_config_data(auctionConfigDataAfterDeposit)

  const userData = await program.account.userAccount.fetch(userAccount);
  print_user_data(userData)
}

liquidate();
