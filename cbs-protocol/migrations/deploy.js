// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");
const { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token } = require('@solana/spl-token')
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } = anchor.web3;
// const { cbsAddrs } = require('./wallets');

const idl = require("../target/idl/cbs_protocol.json");
const programID = idl.metadata.address;

console.log("ProgramID", programID);
const PREFIX = "cbsprotocol3";

const pool_ray = "pool_ray";
const pool_wsol = "pool_wsol";
const pool_msol = "pool_msol";
const pool_srm = "pool_srm";
const pool_scnsol = "pool_scnsol";
const pool_stsol = "pool_stsol";
const pool_lpsol = "pool_lpsol";
const pool_lpusd = "pool_lpusd";
const pool_lpfi = "pool_lpfi";

// Test Token's MINT
const rayMint = new PublicKey("CAtFbjnodtzt1mpxyJwPKfWP6MkTisckMk9KHUgSxX7v"); 
const wsolMint = new PublicKey("6hPAQy93EbDzwHyU843zcWKATy8NrJ1ZsKCRi2JkuXcT");
const msolMint = new PublicKey("AzRQUJPKxv8L9xfHPeGgKcsXXrjbYekW5mVvbMdw11Mp");
const srmMint = new PublicKey("2F988bKHUgPaw6mHwuPfdQhiRg1XtCJuDh4hrvVpT3wD");
const scnsolMint = new PublicKey("8eijEjgBCSk8vJcjwV1geZQp8tzvXTXgc7Xgg8qthKyJ");
const stsolMint = new PublicKey("3gb5MH7VF6o6mWbuBX7V8d1KtWX1pCSYMAwFa296rPuP");

const lpsolMint = new PublicKey("5jmsfTrYxWSKgrZp4Y8cziTWvt7rqmTCiJ75FbLqFTVZ"); 
const lpusdMint = new PublicKey("3GB97goPSqywzcXybmVurYW7jSxRdGuS28nj74W8fAtL");
const lpfiMint = new PublicKey("3x96fk94Pp4Jn2PWUexAXYN4eLK8TVYXHUippdYCHK1p");

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  try {
    // const config  = new PublicKey("6bUzHQxih8vuMtZL7fm2xsfSt55zDuL4m9RwrqXk9YDp");
    const configAccount = anchor.web3.Keypair.generate();
    console.log("Config: ", configAccount.publicKey.toBase58());

    // Find PDA from `cbs protocol` for state account
    const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX)],
      program.programId
    );
    console.log("State-Account:", stateAccount.toBase58());

    // Find PDA for `RAY pool`
    const [poolRay, poolRayBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(pool_ray)],
      program.programId
    );
    console.log("Pool-RAY:", poolRay.toBase58());

    // Find PDA for `wSOL pool`
    const [poolWsol, poolWsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(pool_wsol)],
      program.programId
    );
    console.log("Pool-wSOL:", poolWsol.toBase58());

    // Find PDA for `msol pool`
    const [poolMsol, poolMsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(pool_msol)],
      program.programId
    );
    console.log("Pool-MSOL:", poolMsol.toBase58());

    // Find PDA for `srm pool`
    const [poolSrm, poolSrmBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(pool_srm)],
      program.programId
    );
    console.log("Pool-SRM:", poolSrm.toBase58());

    // Find PDA for `scnsol pool`
    const [poolScnsol, poolScnsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(pool_scnsol)],
      program.programId
    );
    console.log("Pool-SCNSOL:", poolScnsol.toBase58());

    // Find PDA for `stsol pool`
    const [poolStsol, poolStsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(pool_stsol)],
      program.programId
    );
    console.log("Pool-STSOL:", poolStsol.toBase58());
    
    // Find PDA for `lpsol pool`
    const [poolLpsol, poolLpsolBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(pool_lpsol)],
      program.programId
    );
    console.log("Pool-LpSOL:", poolLpsol.toBase58());

    // Find PDA for `lpusd pool`
    const [poolLpusd, poolLpusdBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(pool_lpusd)],
      program.programId
    );
    console.log("Pool-LpUSD:", poolLpusd.toBase58());

    // Find PDA for `lpfi pool`
    const [poolLpfi, poolLpfiBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(pool_lpfi)],
      program.programId
    );
    console.log("Pool-LpFi:", poolLpfi.toBase58());

    // Signer
    const authority = provider.wallet.publicKey;
       
    // UpdateConfig
    // await program.rpc.updateConfig({
    //   accounts: {
    //     owner: authority,
    //     config,
    //     stateAccount,
    //     rayMint,
    //     wsolMint,
    //     msolMint,
    //     ethMint,
    //     poolRay,
    //     poolEth,
    //     poolWsol,
    //     poolMsol,
    //     lpsolMint,
    //     lpusdMint,
    //     lpfiMint,
    //     lpethMint,
    //     poolLpsol,
    //     poolLpusd,
    //     poolLpfi,
    //     poolLpeth,
    //     systemProgram: SystemProgram.programId,
    //     tokenProgram: TOKEN_PROGRAM_ID,
    //     rent: SYSVAR_RENT_PUBKEY,
    //   }
    // });

    // initialize
    await program.rpc.initialize({
      accounts: {
        authority,
        stateAccount,
        config: configAccount.publicKey,
        rayMint,
        wsolMint,
        msolMint,
        ethMint,
        ustMint,
        srmMint,
        scnsolMint,
        stsolMint,
        usdtMint,
        lpsolMint,
        lpusdMint,
        lpfiMint,
        lpethMint,
        poolLpsol,
        poolLpusd,
        poolLpfi,
        poolLpeth,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [configAccount]
    });

    await program.rpc.initializePool({
      accounts: {
        authority,
        stateAccount,
        config: configAccount.publicKey,
        rayMint,
        wsolMint,
        msolMint,
        ethMint,
        ustMint,
        srmMint,
        scnsolMint,
        stsolMint,
        usdtMint,
        poolRay,
        poolEth,
        poolWsol,
        poolMsol,
        poolUst,
        poolSrm,
        poolScnsol,
        poolStsol,
        poolUsdt,
        lpsolMint,
        lpusdMint,
        lpfiMint,
        lpethMint,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });

  } catch (err) {
    console.log("Transaction error: ", err);
  }
}

// 2022-06-30
// ProgramID 3f39cgs9wPLVv4vGySNecjKtefe5MJYkFEEj3v6bPequ
// Config:  4mND9qtpmZN5fEk48TMy2tUEaSh5QFGL64ruFXMwuwRA
// State-Account: HKakh92meu61n3kchSPpNDveCwHno9ymeamN9yZbXt1z
// Pool-ETH: 5N45aAx4aAj5CmD5wiievuG9bnY3wwzLNzXUdgsXhpy
// Pool-USDC: HoKqptd4zJzE5w5RvAXt39hHs4WDFvXnSry8xyedrrYc
// Pool-BTC: J1bgGA5Khj3t9bPepVpS6rPD5o1Pa4JdxypWJQrXt6Zu
// Pool-MSOL: GpCTx7w81RAqdJgC9HmJxwshUmagb1tpqG76beVpPwYA
// Pool-UST: 8jde3rbntGh34KSk2V48q7w6WeFCCjEFLsxtV6AB2pWp
// Pool-SRM: G7W5Dzby8sV8FobviTvogkzuksneFBCncy1kwjuoQea5
// Pool-SCNSOL: 5RVTEUCfePKfwnvQdgW3nhKyndyPXAM2TJfUKXuT1dzP
// Pool-STSOL: GYWxoNG6adwwe6Za6V7QQCxKFsDvJXxDNdhPrLuGu1CC
// Pool-USDT: 51FovGmzpBmxy31yHAnhu4Ftb5VgoYV4bYxNhQdoPVAZ
// Pool-LpSOL: HbYcBPKmKcNVFgvQt8in3KFTG6GQtYHXDM7Dwq2q7JKo
// Pool-LpUSD: Bc7AzMJcUbE1c3CLHpXtyY7g9x23BBryJgqXRPvwHyCB
// Pool-LpBTC: E6Bu3gzRbTuPmWRiCArJqtAvUzYFwJYf9nYZSjSarTJT
// Pool-LpETH: GvbQ59hQsovMGaz2W4rq6xCxtF43UJdugq9mJocYWTpU

/*
module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  for (const idx in cbsAddrs) {
    try {
      console.log(cbsAddrs[idx])
      const authority = new PublicKey(cbsAddrs[idx]);
      const [userAccount, userAccountBump] = await PublicKey.findProgramAddress(
        [Buffer.from(PREFIX), Buffer.from(authority.toBuffer())],
        program.programId
      );
  
      await program.rpc.fixUserAccount( new anchor.BN("0"), {
        accounts: {
          userAccount
        }
      });
    } catch (err) {
      console.log(err)
    }
  }
} */