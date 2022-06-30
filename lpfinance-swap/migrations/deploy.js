// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");
const TOKEN = require("@solana/spl-token");
const { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token, getAssociatedTokenAddress } = require('@solana/spl-token')
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } = anchor.web3;

const idl = require("../target/idl/lpfinance_swap.json");
const programID = idl.metadata.address;

console.log("ProgramID", programID);
const PREFIX = "lpfiswap";


// Test Token's MINT
const usdcMint = new PublicKey("8cCs2Th4ivThrJPrkgAWNTegQgMcuBmY7TASv7FPhitj"); 
const rayMint = new PublicKey("25ggxgxMqejf5v9WSQWboqxpsrik1u94PCP5EwPBYeEJ");
const msolMint = new PublicKey("3dDwpZWQqCc5SttGJ2yNnYUnLSBnh9cjWJQPeKNDmDTz");
const wsolMint = new PublicKey("CZqXAbuUzGngd97oLjR1bcWkkZrz7MsKAbTJX9oT5Epv"); 
const srmMint = new PublicKey("GB8u3PRkQoi73v5Tctqj5he4M441S2QfqMpcaAsnozE6");
const scnsolMint = new PublicKey("GXFmXhwBMfXq5utccyNcQRrfQuBVjjprHKSqLzi3P7vn");
const stsolMint = new PublicKey("CJGeMYvL7s2k8VHooJ1JvgZsCJqrSEExmPkpFBZskAfV");

const lpsolMint = new PublicKey("9Mcq5PQsEXuSY19ei8CqzRawPdPSAH1VM63GqtZU3x18"); 
const lpusdMint = new PublicKey("8YawjpcTDs3SsR7bsCHDb4b1Yv3PAKULB5xZ5VNunroJ");
const lpfiMint = new PublicKey("B8w6e1gSCHE4xNhPhaK5y3cYYBwKMmfJqfe3C9692mGW");

const pool_usdc = Buffer.from(usdcMint.toBuffer());
const pool_ray = Buffer.from(rayMint.toBuffer());
const pool_msol = Buffer.from(msolMint.toBuffer());
const pool_wsol = Buffer.from(wsolMint.toBuffer());
const pool_srm = Buffer.from(srmMint.toBuffer());
const pool_scnsol = Buffer.from(scnsolMint.toBuffer());
const pool_stsol = Buffer.from(stsolMint.toBuffer());
const pool_lpsol = Buffer.from(lpsolMint.toBuffer());
const pool_lpusd = Buffer.from(lpusdMint.toBuffer());
const pool_lpfi = Buffer.from(lpfiMint.toBuffer());

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  try {

    const configAccount = anchor.web3.Keypair.generate();
    console.log("Config: ", configAccount.publicKey.toBase58());

    // Find PDA from `cbs protocol` for state account
    const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX)],
      program.programId
    );
    console.log("State-Account:", stateAccount.toBase58());

    // Find PDA for `usdc pool`
    const [poolUsdc, poolUsdcBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_usdc],
      program.programId
    );
    console.log("Pool-USDC:", poolUsdc.toBase58());

    // Find PDA for `RAY pool`
    const [poolRay, poolRayBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_ray],
      program.programId
    );
    console.log("Pool-RAY:", poolRay.toBase58());


    // Find PDA for `msol pool`
    const [poolMsol, poolMsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_msol],
      program.programId
    );
    console.log("Pool-MSOL:", poolMsol.toBase58());


    // Find PDA for `wsol pool`
    const [poolWsol, poolWsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_wsol],
      program.programId
    );
    console.log("Pool-wSOL:", poolWsol.toBase58());

    // Find PDA for `srm pool`
    const [poolSrm, poolSrmBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_srm],
      program.programId
    );
    console.log("Pool-SRM:", poolSrm.toBase58());

    // Find PDA for `scnsol pool`
    const [poolScnsol, poolScnsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_scnsol],
      program.programId
    );
    console.log("Pool-SCNSOL:", poolScnsol.toBase58());

    // Find PDA for `stsol pool`
    const [poolStsol, poolStsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_stsol],
      program.programId
    );
    console.log("Pool-STSOL:", poolStsol.toBase58());
    
    // Find PDA for `lpusd pool`
    const [poolLpusd, poolLpusdBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_lpusd],
      program.programId
    );
    console.log("Pool-LpUSD:", poolLpusd.toBase58());

    // Find PDA for `lpsol pool`
    const [poolLpsol, poolLpsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_lpsol],
      program.programId
    );
    console.log("Pool-LpSOL:", poolLpsol.toBase58());

    // Find PDA for `lpbtc pool`
    const [poolLpfi, poolLpfiBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_lpfi],
      program.programId
    );
    console.log("Pool-LpFi:", poolLpfi.toBase58());
 
    // Signer
    const authority = provider.wallet.publicKey;
    
    // const config = new PublicKey("7jgbQyMsLkinSJ6fQHtUHwaUyKdwkGGE7scTFeTV8qzw");

    // initialize
    await program.rpc.initialize({
      accounts: {
        authority,
        stateAccount,
        // config,
        config: configAccount.publicKey,
        // lpusdMint,
        // lpsolMint,
        usdcMint,
        lpfiMint,
        // lpethMint,
        // poolLpfi,
        // poolLpeth,
        // poolLpsol,
        // poolLpusd,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [configAccount]
    });

    await createSinglePool(program, authority, stateAccount, usdcMint, poolUsdc, configAccount.publicKey);
    await createSinglePool(program, authority, stateAccount, lpfiMint, poolLpfi, configAccount.publicKey);
    await createSinglePool(program, authority, stateAccount, rayMint, poolRay, configAccount.publicKey);
    await createSinglePool(program, authority, stateAccount, msolMint, poolMsol, configAccount.publicKey);
    await createSinglePool(program, authority, stateAccount, wsolMint, poolWsol, configAccount.publicKey);
    await createSinglePool(program, authority, stateAccount, srmMint, poolSrm, configAccount.publicKey);
    await createSinglePool(program, authority, stateAccount, scnsolMint, poolScnsol, configAccount.publicKey);
    await createSinglePool(program, authority, stateAccount, stsolMint, poolStsol, configAccount.publicKey);

    const userTokena = await getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      lpfiMint,
      authority
    );

    const userTokenb = await getAssociatedTokenAddress(
      ASSOCIATED_TOKEN_PROGRAM_ID,
      TOKEN_PROGRAM_ID,
      usdcMint,
      authority
    );

    // Find PDA for `liquidityPool pool`
    const [liquidityPool, liquidityPoolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_lpfi, pool_usdc],
      program.programId
    );
    console.log("LiquidityPool:", liquidityPool.toBase58());

    await createTokenPair(authority, lpfiMint, usdcMint, liquidityPool, userTokena, userTokenb, poolLpfi, poolUsdc);
  } catch (err) {
    console.log("Transaction error: ", err);
  }
}

const createSinglePool = async (program, authority, stateAccount, tokenMint, tokenPool, config) => {
  try {
    await program.rpc.initializePool({
      accounts: {
        authority,
        stateAccount,
        config,
        tokenMint,
        tokenPool,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });
  } catch (err) {
    console.log(err)
  }
}

const convert_to_wei = (val) => (parseFloat(val) * 1e9).toString();
const createTokenPair = async (
  authority,
  tokenaMint,
  tokenbMint,
  liquidityPool,
  userTokena,
  userTokenb,
  tokenaPool,
  tokenbPool
) => {
  try {
    const a_wei = convert_to_wei(10000000);
    const b_wei = convert_to_wei(10000000);
    const tokena_amount = new anchor.BN(a_wei);
    const tokenb_amount = new anchor.BN(b_wei);
    await program.rpc.createPair(
      tokena_amount, tokenb_amount,
    {
      accounts: {
        authority,
        tokenaMint,
        tokenbMint,
        liquidityPool,
        userTokena,
        userTokenb,
        tokenaPool,
        tokenbPool,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });
  } catch (err) {
    console.log(err)
  }
}

const addTokenLiquidityPool = async (
  authority,
  tokenaMint,
  tokenbMint,
  liquidityPool,
  userTokena,
  userTokenb,
  tokenaPool,
  tokenbPool
) => {
  try {
    await program.rpc.addLiquidity({
      accounts: {
        authority,
        tokenaMint,
        tokenbMint,
        liquidityPool,
        userTokena,
        userTokenb,
        tokenaPool,
        tokenbPool,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });
  } catch (err) {
    console.log(err)
  }
}
// 2022-04-13 devnet
// ProgramID 6dMiU9ZmaFTPeLPco5rMjXCbUUyJZyRvHPccXXTefTLu
// Config-Account: 7jgbQyMsLkinSJ6fQHtUHwaUyKdwkGGE7scTFeTV8qzw
// State-Account: 35oKiStiHmkrfCaFEyHs5suMiLHsM5VsAFQ3peKknkDV
// Pool-USDC: DPeobw5yJS1dkfZaE2Z67gGXsfdDfq5PdXpkBY5HheLG
// Pool-BTC: 84mg2xQAYsVLLGCesLWh1As8YicVkfcWiGKwDes2xg5R
// Pool-MSOL: Ci8PRat8mtgspMyVxHJLj4G32TXJphBKdex8Ka2Ej2TF
// Pool-ETH: 4KcLB2PVsitzKu2pzHuQF2ACB8EQVnKa463Wjn8EtwkB
// Pool-UST: F9uxM2ijA2wVBbnbX92QAP85PfxwaqMBiYA1QoNZrrst
// Pool-SRM: 9cZi4DnSWELQPFaZdSD5fWD77sPEeGALPQhz5E7FYMFT
// Pool-SCNSOL: F4RFismMeTCaDjVGwkTv6oHaNEipQu43EbpbszfFxvAz
// Pool-STSOL: CSQvYuTZzuFfSx8ZxvDnhFkB7gVJHsQEiw8w36KCwmEN
// Pool-USDT: Gdqm6TiL1rnHCXaurBDr4Tek5t1jFNeiAzxc5KEZYMvD
// Pool-LpUSD: 6PTciQETNSwB3FkiivjnU4KLWTr78KBhuPfBt8TazUWZ
// Pool-LpSOL: BTX4Wauvb4GRkThrwXBJpGfm3AkqGCKAWhnD4z9MSsPs
// Pool-LpBTC: HSWbnqVXb8YHMha3rk2Hc2dedP7iLZtvKETzj33cF6FC
// Pool-LpETH: 8YfoUygmJfPgBmGaNqX7oWC2ZggYexJ6Bs21P7f1ct3C