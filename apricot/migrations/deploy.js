// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");
const { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, Token } = require('@solana/spl-token')
const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, Keypair } = anchor.web3;

const idl = require("../target/idl/apricot.json");
const programID = idl.metadata.address;

console.log("ProgramID", programID);
const PREFIX = "apricot0";

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  try {

    // Signer
    const authority = provider.wallet.publicKey; 
    // CBS account      
    const user = new PublicKey('Gu6urAi7fg5eV5Jvohdo6U1pxYTSxheh5RxUzprdENLq');
    const [userAccount, userAccountBump] = await PublicKey.findProgramAddress(
      [Buffer.from(PREFIX), Buffer.from(user.toBuffer())],
      program.programId
    );
    console.log("CBS apricot account", userAccount.toBase58());

    // initUserAccount
    // await program.rpc.initUserAccount({
    //   accounts: {
    //     userAccount,
    //     user,
    //     userAuthority: authority,
    //     systemProgram: SystemProgram.programId,
    //     rent: SYSVAR_RENT_PUBKEY,
    //   }
    // });

  } catch (err) {
    console.log("Transaction error: ", err);
  }
}

/*
module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);
}
*/