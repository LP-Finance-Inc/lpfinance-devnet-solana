// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");
const TOKEN = require("@solana/spl-token");
const { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token, getAssociatedTokenAddress } = require('@solana/spl-token')
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Connection } = anchor.web3;

const idl = require("../target/idl/lpfinance_swap.json");
const programID = idl.metadata.address;

console.log("ProgramID", programID);
const PREFIX = "lpfiswap0";


// Test Token's MINT
const usdcMint = new PublicKey("6ybV587PY2z6DX4Pf1tTh8oEhnuR6wwXLE8LHinKQKYV"); 
const rayMint = new PublicKey("CAtFbjnodtzt1mpxyJwPKfWP6MkTisckMk9KHUgSxX7v");
const msolMint = new PublicKey("AzRQUJPKxv8L9xfHPeGgKcsXXrjbYekW5mVvbMdw11Mp");
const wsolMint = new PublicKey("6hPAQy93EbDzwHyU843zcWKATy8NrJ1ZsKCRi2JkuXcT"); 
const srmMint = new PublicKey("2F988bKHUgPaw6mHwuPfdQhiRg1XtCJuDh4hrvVpT3wD");
const scnsolMint = new PublicKey("8eijEjgBCSk8vJcjwV1geZQp8tzvXTXgc7Xgg8qthKyJ");
const stsolMint = new PublicKey("3gb5MH7VF6o6mWbuBX7V8d1KtWX1pCSYMAwFa296rPuP");

const lpsolMint = new PublicKey("5jmsfTrYxWSKgrZp4Y8cziTWvt7rqmTCiJ75FbLqFTVZ"); 
const lpusdMint = new PublicKey("3GB97goPSqywzcXybmVurYW7jSxRdGuS28nj74W8fAtL");
const lpfiMint = new PublicKey("3x96fk94Pp4Jn2PWUexAXYN4eLK8TVYXHUippdYCHK1p");

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

/*
module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  try {

    // const configAccount = anchor.web3.Keypair.generate();
    // console.log("Config: ", configAccount.publicKey.toBase58());

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
        usdcMint,
        lpfiMint,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });

    await createSinglePool(program, authority, stateAccount, usdcMint, poolUsdc);
    await createSinglePool(program, authority, stateAccount, lpfiMint, poolLpfi);
    await createSinglePool(program, authority, stateAccount, rayMint, poolRay);
    await createSinglePool(program, authority, stateAccount, msolMint, poolMsol);
    await createSinglePool(program, authority, stateAccount, wsolMint, poolWsol);
    await createSinglePool(program, authority, stateAccount, srmMint, poolSrm);
    await createSinglePool(program, authority, stateAccount, scnsolMint, poolScnsol);
    await createSinglePool(program, authority, stateAccount, stsolMint, poolStsol);

    const userTokena = await getAssociatedTokenAddress(
      lpfiMint,
      authority,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const userTokenb = await getAssociatedTokenAddress(
      usdcMint,
      authority,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    // Find PDA for `liquidityPool pool`
    const [liquidityPool, liquidityPoolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_lpfi, pool_usdc],
      program.programId
    );
    console.log("LiquidityPool:", liquidityPool.toBase58());

    await createTokenPair(
      program, 
      authority, 
      lpfiMint, 
      usdcMint, 
      liquidityPool, 
      userTokena, 
      userTokenb, 
      poolLpfi, 
      poolUsdc
    );
    await addTokenLiquidityPool(
      program,
      authority,
      lpfiMint,
      usdcMint,
      liquidityPool,
      userTokena,
      userTokenb,
      poolLpfi,
      poolUsdc
    );
  } catch (err) {
    console.log("Transaction error: ", err);
  }
}
*/
const createSinglePool = async (program, authority, stateAccount, tokenMint, tokenPool) => {
  try {
    await program.rpc.initializePool({
      accounts: {
        authority,
        stateAccount,
        // config,
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


module.exports = async function(provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  // const priceData = await program.account.poolInfo.fetch(new PublicKey("DcB2ZfvRU5ac9FgYF9doWYjHWtRXbStH7wDhciQcmF6v"));
  // console.log(priceData);

  try {

    // Signer
    const authority = provider.wallet.publicKey;
    

    const userTokena = await getAssociatedTokenAddress(
      lpfiMint,
      authority,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const userTokenb = await getAssociatedTokenAddress(
      usdcMint,
      authority,
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    // Find PDA for `liquidityPool pool`
    const [liquidityPool, liquidityPoolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), pool_lpfi, pool_usdc],
      program.programId
    );
    console.log("LiquidityPool:", liquidityPool.toBase58(), authority.toBase58(), userTokena.toBase58(), userTokenb.toBase58());

    const poolLpfi = new PublicKey("GnFDvH41YPW3DtQcaHEk4xZhG3Xeh9XXgwsC2EWHkoSb");
    const poolUsdc = new PublicKey("26UWs5QHCgNPeLBpQvjQpZ9TmonKLJFmx21SHXBZ7w9V");
    await createTokenPair(program, authority, lpfiMint, usdcMint, liquidityPool, userTokena, userTokenb, poolLpfi, poolUsdc);
    await addTokenLiquidityPool(
      program,
      authority,
      lpfiMint,
      usdcMint,
      liquidityPool,
      userTokena,
      userTokenb,
      poolLpfi,
      poolUsdc
    );
  } catch (err) {
    console.log(err)
  }
}

const convert_to_wei = (val) => (parseFloat(val) * 1e9).toString();
const createTokenPair = async (
  program,
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
  program,
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
    await program.rpc.addLiquidity(
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
// 2022-07-01 devnet
// ProgramID 87jyVePaEbZAYAcAjtGwQTcC4LU188KxJdynUzFWJKHA
// State-Account: Gj9NXzkjoY3DW4XRroyZMjDDrBzvc2HCtDaGtWgaZmDC
// Pool-USDC: 26UWs5QHCgNPeLBpQvjQpZ9TmonKLJFmx21SHXBZ7w9V
// Pool-RAY: 5gS9zobDfjzdGwfM4mFubusq5Uqkr2WwyhrntXoigR9S
// Pool-MSOL: APgMT9TK3Th1XcfHGAgGuyVZGyn12YHJD6HYPG8Zj5Tf
// Pool-wSOL: BTSKPGQPERh3QjnaBfZmDKKwVjvv4jPYQDNVUJ2mik3d
// Pool-SRM: BVrTba4EVWpeRkZ3JJUqDcfuP11bfzeTiC5jhczkdGym
// Pool-SCNSOL: 95QWqKXjFvcsfcpFh6Uo8pJ9JHcN5wJcPCgzLb7hVAeX
// Pool-STSOL: EUZJuR3Ws8VaCypcFHCeg9mHbrtmX9t6ZH4ERAh857MP
// Pool-LpUSD: EigZFwsHC5uUiPtBn7xMD19uv4Rb8ajZQKZWpisagis4
// Pool-LpSOL: E8JD5qTobkasrxLFyv8AZCw8aY1mP5AgzVfGedztCWm
// Pool-LpFi: GnFDvH41YPW3DtQcaHEk4xZhG3Xeh9XXgwsC2EWHkoSb
// LiquidityPool: DcB2ZfvRU5ac9FgYF9doWYjHWtRXbStH7wDhciQcmF6v