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
    PREFIX,
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
    SwapRouterIDL,
    StableSwapIDL,
    TestTokenIDL,
    wSOLMint,
    USDCMint
} from "../config";

import { convert_to_wei, getATAPublicKey, getCreatorKeypair, getPublicKey, print_config_data, print_user_data } from "../utils";
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

  const swapRouterProgramId = new PublicKey(SwapRouterIDL.metadata.address);
  const lptokenProgramId = new PublicKey(LpfinanceTokenIDL.metadata.address);
  const stableswapProgramId = new PublicKey(StableSwapIDL.metadata.address);
  const testTokenProgramId = new PublicKey(TestTokenIDL.metadata.address);
  
  const PDA = await PublicKey.findProgramAddress(
    [Buffer.from(PREFIX)],
    program.programId
  );

  const lpusdMint= auctionConfigData.lpusdMint as PublicKey;
  const lpusdAta = auctionConfigData.poolLpusd as PublicKey;

  const lpsolMint= auctionConfigData.lpsolMint as PublicKey;
  const lpsolAta = auctionConfigData.poolLpsol as PublicKey;

  const wsolMint= wSOLMint;
  const usdcMint = USDCMint;

  console.log("PDA:", PDA[0].toBase58());

  const [userAccount, bump] = await PublicKey.findProgramAddress(
    [Buffer.from(PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
    program.programId
  );

  const cbsProgramId = new PublicKey(CBSProtocolIDL.metadata.address);
  const [cbsAccount, cbsBump] = await PublicKey.findProgramAddress(
    [Buffer.from(CBS_PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
    cbsProgramId
  );
  
  // STEP: 1
  // await program.rpc.burnLpusdLiquidate({
  //   accounts: {
  //       owner: creatorKeypair.publicKey,
  //       auctionPda: PDA[0],
  //       config: config,
  //       cbsAccount,
  //       lpusdMint,
  //       lpusdAta,
  //       lpsolMint,
  //       lpsolAta,
  //       stableLpsolPool: StableLpsolPool,
  //       stableLpusdPool: StableLpusdPool,
  //       pythUsdcAccount,
  //       pythRayAccount,
  //       pythSolAccount,
  //       pythMsolAccount,
  //       pythSrmAccount,
  //       pythScnsolAccount,
  //       pythStsolAccount,
  //       liquidityPool: LiquidityPool,
  //       cbsProgram: cbsProgramId,
  //       lptokensProgram: lptokenProgramId,
  //       systemProgram: anchor.web3.SystemProgram.programId,
  //       tokenProgram: TOKEN_PROGRAM_ID,
  //       rent: SYSVAR_RENT_PUBKEY
  //   },
  // });

  const swap_escrow_pool_pda = await PublicKey.findProgramAddress(
    [
        Buffer.from("swap-escrow"),
        PDA[0].toBuffer()
    ],
    swapRouterProgramId
  ); 

  const escrowAtaLpsol = await getATAPublicKey(lpsolMint, swap_escrow_pool_pda[0]);
  const escrowAtaLpusd = await getATAPublicKey(lpusdMint, swap_escrow_pool_pda[0]);
  const escrowAtaUsdc = await getATAPublicKey(usdcMint, swap_escrow_pool_pda[0]);
  const escrowAtaWsol = await getATAPublicKey(wsolMint, swap_escrow_pool_pda[0]);

  const stableswapPoolAtaLpsol = await getATAPublicKey(lpsolMint, StableLpsolPool);
  const stableswapPoolAtaLpusd = await getATAPublicKey(lpusdMint, StableLpusdPool);
  const stableswapPoolAtaUsdc = await getATAPublicKey(usdcMint, StableLpusdPool);
  const stableswapPoolAtaWsol = await getATAPublicKey(wsolMint, StableLpsolPool);
  
  const tokenStateAccount = new PublicKey("FEL9EygF1C3d5cwD2ZXkpmaQMBtdxKd1mvYRrD81KNVY");
  
  console.log("Escrow", escrowAtaLpsol.toBase58(), lpsolMint.toBase58(), swap_escrow_pool_pda[0].toBase58())
  // STEP: 2
  await program.rpc.burnLpsolLiquidate1({
    accounts: {
      owner: creatorKeypair.publicKey,
      cbsAccount,
      auctionPda: PDA[0],
      stableLpsolPool: StableLpsolPool,
      stableLpusdPool: StableLpusdPool,
      swapEscrow: swap_escrow_pool_pda[0],
      tokenStateAccount,
      // tokenLpsol:lpsolMint,
      tokenLpusd: lpusdMint,
      tokenUsdc: usdcMint,
      tokenWsol: wsolMint,
      pythUsdc: pythUsdcAccount,
      pythWsol: pythSolAccount,
      // auctionAtaLpsol: lpsolAta,
      auctionAtaLpusd: lpusdAta,
      // stableswapPoolAtaLpsol,
      stableswapPoolAtaLpusd,
      stableswapPoolAtaUsdc,
      // stableswapPoolAtaWsol,
      // escrowAtaLpsol,
      escrowAtaLpusd,
      escrowAtaUsdc,
      escrowAtaWsol,
      testtokensProgram: testTokenProgramId,
      swaprouterProgram: swapRouterProgramId,
      stableswapProgram: stableswapProgramId,
      cbsProgram: cbsProgramId,
      lptokensProgram: lptokenProgramId,
      systemProgram: anchor.web3.SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY
    },
  });

  console.log("Burn LpSOL and LpUSD successfully")

  const auctionConfigDataAfterDeposit = await program.account.config.fetch(config);
  print_config_data(auctionConfigDataAfterDeposit)

  const userData = await program.account.userAccount.fetch(userAccount);
  print_user_data(userData)
}

liquidate();
