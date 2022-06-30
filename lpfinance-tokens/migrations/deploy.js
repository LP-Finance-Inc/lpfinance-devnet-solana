// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");
const { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, getAssociatedTokenAddress } = require('@solana/spl-token')
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } = anchor.web3;

const idl = require("../target/idl/lpfinance_tokens.json");
const programID = idl.metadata.address;

console.log("ProgramID", programID);

const PREFIX = "lptokens";

const lpsol_mint = "lpsol_mint";
const lpusd_mint = "lpusd_mint";
const lpdao_mint = "lpdao_mint";

const convert_to_wei = (val) => (parseFloat(val) * 1e9).toString();

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  try {
    const stateAccount = new PublicKey("64iaARaRU9sXwLmAVy1a5NkYVM82GJ9Lvk2VfJ8PMChk");
    // Signer
    const authority = provider.wallet.publicKey;
    const lptokenMint = new PublicKey("3x96fk94Pp4Jn2PWUexAXYN4eLK8TVYXHUippdYCHK1p");
    // mint: PublicKey, owner: PublicKey, allowOwnerOffCurve?: boolean, programId?: PublicKey, associatedTokenProgramId?: PublicKey
    const userLptoken = await getAssociatedTokenAddress(
      lptokenMint, 
      authority, 
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )
    
    console.log("UserLpToken", userLptoken.toBase58(), authority.toBase58())
    const wei_val = convert_to_wei(550000000);
    const amount = new anchor.BN(wei_val);
    // await program.rpc.ownerMintLptoken(
    //   amount,
    //   {
    //     accounts: {
    //       owner: authority,
    //       stateAccount,
    //       lptokenMint,
    //       userLptoken,
    //       systemProgram: SystemProgram.programId,
    //       associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    //       tokenProgram: TOKEN_PROGRAM_ID,
    //       rent: SYSVAR_RENT_PUBKEY,
    //     }
    // });
    
    const config = new PublicKey("3Lpjwy6tGj4XQVJBMcr8ESRpLDgdat3ozedQD5AjSf5a");
    await program.rpc.burnLptoken(
      amount,
      {
        accounts: {
          cbsAccount: authority,
          stateAccount,
          config,
          cbsLptoken: userLptoken,
          lptokenMint,
          systemProgram: SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          rent: SYSVAR_RENT_PUBKEY
        }
      }
    )
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
    // Find PDA from `cbs protocol` for state account
    const [stateAccount, stateAccountBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX)],
      program.programId
    );
    console.log("State-Account:", stateAccount.toBase58());

    const [lpsolMint, lpsolMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(lpsol_mint)],
      program.programId
    );
    console.log("LpSOL mint:", lpsolMint.toBase58());

    const [lpusdMint, lpusdMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(lpusd_mint)],
      program.programId
    );
    console.log("LpUSD Mint:", lpusdMint.toBase58());

    const [lpdaoMint, lpdaoMintBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(lpdao_mint)],
      program.programId
    );
    console.log("LpFI Mint:", lpdaoMint.toBase58());
    
    // Signer
    const authority = provider.wallet.publicKey;

    const configAccount = anchor.web3.Keypair.generate();
    console.log("Config: ", configAccount.publicKey.toBase58());
    // mint: PublicKey, owner: PublicKey, allowOwnerOffCurve?: boolean, programId?: PublicKey, associatedTokenProgramId?: PublicKey
    const userDaotoken = await getAssociatedTokenAddress(
      lpdaoMint, 
      authority, 
      true,
      TOKEN_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    )

    await program.rpc.initialize({
      accounts: {
        authority,
        stateAccount,
        config: configAccount.publicKey,
        lpsolMint,
        lpusdMint,
        lpdaoMint,
        userDaotoken,
        systemProgram: SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [configAccount]
    });

  } catch (err) {
    console.log("Transaction error: ", err);
  }
} */

// 2022-06-01
// ProgramID GaVnsa8z34xSeYNDydTAdbiT64KxMgVXBXnimXXJT4Hw
// State-Account: 64iaARaRU9sXwLmAVy1a5NkYVM82GJ9Lvk2VfJ8PMChk
// Config:  3Lpjwy6tGj4XQVJBMcr8ESRpLDgdat3ozedQD5AjSf5a

// LpSOL mint: 5jmsfTrYxWSKgrZp4Y8cziTWvt7rqmTCiJ75FbLqFTVZ
// LpUSD Mint: 3GB97goPSqywzcXybmVurYW7jSxRdGuS28nj74W8fAtL
// LpFI Mint: 3x96fk94Pp4Jn2PWUexAXYN4eLK8TVYXHUippdYCHK1p