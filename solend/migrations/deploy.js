// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");
const { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token } = require('@solana/spl-token')
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } = anchor.web3;

const idl = require("../target/idl/solend.json");
const programID = idl.metadata.address;

console.log("ProgramID", programID);
const PREFIX = "solend0";

const pool_ray = "pool_ray";
const pool_wsol = "pool_wsol";
const pool_msol = "pool_msol";
const pool_srm = "pool_srm";
const pool_scnsol = "pool_scnsol";
const pool_stsol = "pool_stsol";

// Test Token's MINT
const rayMint = new PublicKey("CAtFbjnodtzt1mpxyJwPKfWP6MkTisckMk9KHUgSxX7v"); 
const wsolMint = new PublicKey("6hPAQy93EbDzwHyU843zcWKATy8NrJ1ZsKCRi2JkuXcT");
const msolMint = new PublicKey("AzRQUJPKxv8L9xfHPeGgKcsXXrjbYekW5mVvbMdw11Mp");
const srmMint = new PublicKey("2F988bKHUgPaw6mHwuPfdQhiRg1XtCJuDh4hrvVpT3wD");
const scnsolMint = new PublicKey("8eijEjgBCSk8vJcjwV1geZQp8tzvXTXgc7Xgg8qthKyJ");
const stsolMint = new PublicKey("3gb5MH7VF6o6mWbuBX7V8d1KtWX1pCSYMAwFa296rPuP");

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  try {

    // Signer
    const authority = provider.wallet.publicKey; 
    // CBS account      
    const user = new PublicKey('8NSpbuD66CrveJYufKZWiJPneVak7Ri74115qpiP8xw4');
    const [userAccount, userAccountBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(user.toBuffer())],
      program.programId
    );
    console.log("CBS solend account", userAccount.toBase58());

    // initUserAccount
    await program.rpc.initUserAccount({
      accounts: {
        userAccount,
        user,
        userAuthority: authority,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      }
    });

  } catch (err) {
    console.log("Transaction error: ", err);
  }
}

/*
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
    
    // Signer
    const authority = provider.wallet.publicKey;       

    // initialize
    await program.rpc.initialize({
      accounts: {
        authority,
        stateAccount,
        config: configAccount.publicKey,
        rayMint,
        wsolMint,
        msolMint,
        srmMint,
        scnsolMint,
        stsolMint,
        poolWsol,
        poolRay,
        poolMsol,
        poolSrm,
        poolScnsol,
        poolStsol,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [configAccount]
    });

  } catch (err) {
    console.log("Transaction error: ", err);
  }
}
*/

// 2022-06-30
// CBS solend account : 6ArpAjPPRXoWvrNSuFfS9mnJXHWZnT33RcabftCWXi7F

// ProgramID BgN7NGCcQgFMVs9U8tQzht7NNrKCdQjcZ8g4GBRwgMrb
// Config:  68SQXmcLmJzEUUm5MxudGZfJiPHsMEu3rQboTuNEabUT
// State-Account: 76XJ35ToUi7ivAc9p62t8t4ukvE9BQPsNRSbUKiZmmuW
// Pool-RAY: AjU3jz8zc7vksB42VhvH3D1Rx5M5s2Bfh94WbJdYS79Y
// Pool-wSOL: BuTudvKNcgdUQ6r8zsiUVjviupbvP4X1obxFwzE797yK
// Pool-MSOL: 8yY8cKyP1sQNEBummXs2joVhijDSdzQwcBU2VcVG3z9w
// Pool-SRM: 3azEFYaMnyRBZ79uGpiEFvt4JHB3BjESTj99gAqKvi4z
// Pool-SCNSOL: 4aEuDicF4H4J8PWL3YGLVf9c38pbkfTKHQMRhiuzc4z2
// Pool-STSOL: 3zLmj7hcrUwGdd6EQKeD5k3Lags8eG9jCwreAsjG7YwD