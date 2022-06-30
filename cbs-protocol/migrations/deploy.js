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
const PREFIX = "cbsprotocol1";

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
        lpsolMint,
        lpusdMint,
        lpfiMint,
        poolLpsol,
        poolLpusd,
        poolLpfi,
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
        srmMint,
        scnsolMint,
        stsolMint,
        poolRay,
        poolWsol,
        poolMsol,
        poolSrm,
        poolScnsol,
        poolStsol,
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
// ProgramID 8NSpbuD66CrveJYufKZWiJPneVak7Ri74115qpiP8xw4
// Config:  9gzCVpwVMSW29MZyh5DtxQtNCB8n9Lc5NWDpH3knkdiT
// State-Account: DgTJJkqH89PJwpTstVVfNZ22CEfx8vqLHsMBfvfY6zVW
// Pool-RAY: 8YBkgfB2F1EAnSxQJwheTvaRcqPuGYzw98q9uemYK32g
// Pool-wSOL: 21ewc3oryWoc6coMz5h9TUo6djJMTbYJEKgseozQp35b
// Pool-MSOL: GxFz5cCidDxAFM6Hzq3ASzCBy1b8bxLK7DrA1hiHbTJ4
// Pool-SRM: GQSECtcwiSqHPef7bZVYquU5B9hh3mRiJZHPESKnYmLb
// Pool-SCNSOL: 5KWjrGUWJufAnA8KsEVbgCVhtZekZXV3kZDZVnwH4KAv
// Pool-STSOL: ENJYakYczWYWMX1iVCpmqoSZhNaGX5Mj5YM77U6LsxG5
// Pool-LpSOL: CrAFe3bmrgqCNAx87gf8L9vwheFPWWFmqwKMQN8TaxiQ
// Pool-LpUSD: 64C8Xmb9rumieArK7CAtiHKP9xatm7uTvkfmVtuzxdbn
// Pool-LpFi: 8Fu8HnrkVkKyweWr3Ybq8r8BqaZuzKm1nojRQNoRBPJG

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