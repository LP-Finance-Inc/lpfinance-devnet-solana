const anchor = require("@project-serum/anchor");
const { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token } = require('@solana/spl-token')
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } = anchor.web3;

const idl = require("../target/idl/test_tokens.json");
const programID = idl.metadata.address;

console.log("ProgramID", programID);

const PREFIX = "test-tokens";

const wsol_mint = "wsol_mint";
const msol_mint = "msol_mint";
const stsol_mint = "stsol_mint";
const scnsol_mint = "scnsol_mint";
const usdc_mint = "usdc_mint";
const btc_mint = "btc_mint";
const eth_mint = "eth_mint";
const ray_mint = "ray_mint";
const srm_mint = "srm_mint";
const avax_mint = "avax_mint";
const fida_mint = "fida_mint";
const ftt_mint = "ftt_mint";
const ftm_mint = "ftm_mint";
const gmt_mint = "gmt_mint";
const luna_mint = "luna_mint";
const matic_mint = "matic_mint";
const usdt_mint = "usdt_mint";

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here.
  const program = new anchor.Program(idl, programID);

  try {
    // Find PDA from `cbs protocol` for state account
    const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX)],
      program.programId
    );
    console.log("State-Account:", stateAccount.toBase58());

    // await add_new();
    const configAccount = anchor.web3.Keypair.generate();
    console.log("Config: ", configAccount.publicKey.toBase58());

    const [wsolMint, wsolMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(wsol_mint)],
      program.programId
    );
    console.log("wSOL mint:", wsolMint.toBase58());

    const [msolMint, msolMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(msol_mint)],
      program.programId
    );
    // bumps.poolMsol = poolMsolBump;
    console.log("MSOL Mint:", msolMint.toBase58());

    const [stsolMint, stsolMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(stsol_mint)],
      program.programId
    );
    console.log("stsol-Mint:", stsolMint.toBase58());

    const [scnsolMint, scnsolMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(scnsol_mint)],
      program.programId
    );
    console.log("scnSOL Mint:", scnsolMint.toBase58());

    const [usdcMint, usdcMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(usdc_mint)],
      program.programId
    );
    console.log("USDC Mint:", usdcMint.toBase58());

    const [btcMint, btcMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(btc_mint)],
      program.programId
    );
    console.log("Btc-Mint:", btcMint.toBase58());

    const [ethMint, ethMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(eth_mint)],
      program.programId
    );
    console.log("ETH mint:", ethMint.toBase58());

    const [rayMint, rayMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(ray_mint)],
      program.programId
    );
    console.log("Ray mint:", rayMint.toBase58());

    const [srmMint, srmMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(srm_mint)],
      program.programId
    );
    console.log("SRM Mint:", srmMint.toBase58());

    const [avaxMint, avaxMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(avax_mint)],
      program.programId
    );
    console.log("Avax-Mint:", avaxMint.toBase58());

    const [fidaMint, fidaMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(fida_mint)],
      program.programId
    );
    console.log("fida-Mint:", fidaMint.toBase58());

    const [fttMint, fttMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(ftt_mint)],
      program.programId
    );
    console.log("ftt-Mint:", fttMint.toBase58());

    const [ftmMint, ftmMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(ftm_mint)],
      program.programId
    );
    console.log("ftm-Mint:", ftmMint.toBase58());

    const [gmtMint, gmtMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(gmt_mint)],
      program.programId
    );
    console.log("gmt-Mint:", gmtMint.toBase58());

    const [lunaMint, lunaMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(luna_mint)],
      program.programId
    );
    console.log("luna-Mint:", lunaMint.toBase58());

    const [maticMint, maticMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(matic_mint)],
      program.programId
    );
    console.log("matic-Mint:", maticMint.toBase58());

    const [usdtMint, usdtMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(usdt_mint)],
      program.programId
    );
    console.log("USDT-Mint:", usdtMint.toBase58());

    const authority = provider.wallet.publicKey;
    // initialize
    await program.rpc.initialize({
      accounts: {
        authority,
        stateAccount,
        config: configAccount.publicKey,
        wsolMint,
        msolMint,
        stsolMint,
        scnsolMint,
        usdcMint,
        btcMint,
        ethMint,
        rayMint,
        srmMint,
        avaxMint,
        fidaMint,
        fttMint,
        ftmMint,
        gmtMint,
        lunaMint,
        maticMint,
        usdtMint,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [configAccount]
    });
  } catch (err) {
    console.log("Transaction error: ", err);
  }
};


// 2022-06-01 Devnet deployment
// ProgramID 3QTW9aZp4U2xoj9UfvTF6PEL3UZzfEHi8UtNruhw7GHL
// State-Account: FEL9EygF1C3d5cwD2ZXkpmaQMBtdxKd1mvYRrD81KNVY
// Config:  3Pguudq3L6AHwnSKaPVngwuu9JfFNM2x7sv5WeMSrsw8

// Token List
// wSOL mint: 6hPAQy93EbDzwHyU843zcWKATy8NrJ1ZsKCRi2JkuXcT
// MSOL Mint: AzRQUJPKxv8L9xfHPeGgKcsXXrjbYekW5mVvbMdw11Mp
// stsol-Mint: 3gb5MH7VF6o6mWbuBX7V8d1KtWX1pCSYMAwFa296rPuP
// scnSOL Mint: 8eijEjgBCSk8vJcjwV1geZQp8tzvXTXgc7Xgg8qthKyJ
// USDC Mint: 6ybV587PY2z6DX4Pf1tTh8oEhnuR6wwXLE8LHinKQKYV
// Btc-Mint: 4NAbav42C1BZdKASxuiKbzTFQKSqcZXG7ZZLDwfiZCGe
// ETH mint: 49ZEVDFHe18DDcyAe4fuRrhuf3DQpTDAAUodkaDsCcco
// Ray mint: CAtFbjnodtzt1mpxyJwPKfWP6MkTisckMk9KHUgSxX7v
// SRM Mint: 2F988bKHUgPaw6mHwuPfdQhiRg1XtCJuDh4hrvVpT3wD
// Avax-Mint: FzUkBfKMr8YULR2cNiVHoUF9zH3rA5Zv99BzFohgqQxo
// fida-Mint: BdY3ZJSd66ADaoLVnCiZWLEX4XANxj8a9vXFBGedqtP6
// ftt-Mint: EZvZWjRHqHSf3ge1T13Y1GmTgW2oNWdsaeErxu8fDpBo
// ftm-Mint: FtdjvSFvRHAVcebM2zxfyFJXfDGdGQL1pXtMnAd9AQRG
// gmt-Mint: Hn2UGJ1jM9Tw9oidCJwLdhWpcczS4MrMdb48XvCDMmnP
// luna-Mint: 8sLT5gE4YgcdDgnL6gxy2a9NZ79t46jQgrX87q7iqFPN
// matic-Mint: 6sxP334TsRHEznCMaUNKSzv8xmpTQZXY11fqszF5vYMJ
// USDT-Mint: 4ohBE15Y2L3rPF6T6TXcHwLv7Dtkd9hwHRMBS7UDaw3V