// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");

const { PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY } = anchor.web3;

const idl = require("../target/idl/lpfinance_accounts.json");
const programID = idl.metadata.address;

console.log("ProgramID", programID);

const cbsprogram = new PublicKey("8NSpbuD66CrveJYufKZWiJPneVak7Ri74115qpiP8xw4"); 
const BIG_WHITELIST_LEN = 10000;

module.exports = async function (provider) {
  // Configure client to use the provider.
  anchor.setProvider(provider);

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);

  try {
    const whitelistAccountSize = 8 + (32 * BIG_WHITELIST_LEN);

    const configAccount = anchor.web3.Keypair.generate();
    const whiteListData = anchor.web3.Keypair.generate();
    // console.log("ConfigData: ", configAccount.secretKey);
    // console.log("WhiteListData: ", whiteListData.secretKey);
    console.log("ConfigAccount:", configAccount.publicKey.toBase58());
    console.log("WhiteListAccount:", whiteListData.publicKey.toBase58());

    // Signer
    const authority = provider.wallet.publicKey;
       
    // initialize
    const init_tx = await program.rpc.initialize(cbsprogram, {
      accounts: {
        authority,
        whitelist: whiteListData.publicKey,
        config: configAccount.publicKey,
        systemProgram: SystemProgram.programId,
        rent: SYSVAR_RENT_PUBKEY,
      },
      signers: [configAccount, whiteListData],
      instructions: [
        SystemProgram.createAccount({
          fromPubkey: program.provider.wallet.publicKey,
          lamports:
             await program.provider.connection.getMinimumBalanceForRentExemption(
                whitelistAccountSize
             ),
          newAccountPubkey: whiteListData.publicKey,
          programId: program.programId,
          space: whitelistAccountSize,
        }),
      ],
    });

    console.log("Initialize tx", init_tx);

    // const addys = [];
    // addys.push(new PublicKey("FuRNteV4mDLdvBG1dwPZXKdY5MopQz8pCAx5BJ1XUojw"));
    // addys.push(new PublicKey("YwwpaoBBeNT6zHNT3n1EqhWdCeHjQsCC7Y8ZFdTy6RL"));
    // // addys.push(new PublicKey("YwwpaoBBeNT6zHNT3n1EqhWdCeHjQsCC7Y8ZFdTy6RL"));

    // const tx = await program.rpc.addWhitelistAddresses(addys, {
    //   accounts: {
    //     config: configAccount.publicKey,
    //     whitelist: whiteListData.publicKey,
    //     authority
    //   }
    // });

    // console.log("Tx: ", tx);

    // let accountData = await program.account.whiteList.fetch(whiteListData.publicKey);
    // console.log("Account List1: ", accountData.addresses[0].toBase58());
    // console.log("Account List2: ", accountData.addresses[1].toBase58());

    // const addresses = whiteListJson.addresses;
    // const config = new PublicKey("2N9QkRVTD7nxsPADPYjscZ2LEFyidT4ke3gX2K9xiQV2");
    // const whitelist = new PublicKey("C5LzyP3dxUsLhExJwsrYeN2qgTiPZiw69RAwZmoCJ9uZ");
    // const configData = await program.account.config.fetch(config);
    // // console.log("Counter:", configData);
    // const counter = configData.counter;
    // // return;
    // for (let i = counter + 1; i < addresses.length; i++) { // 
    //   const addys = [];
    //   addys.push(new PublicKey(addresses[i]));

    //   const tx = await program.rpc.addWhitelistAddresses(addys, {
    //     accounts: {
    //       whitelist,
    //       config,
    //       authority
    //     }
    //   });
    //   console.log(i);
    // }


  } catch (err) {
    console.log("Transaction error: ", err);
  }
}

// 2022-06-30 devnet
// ProgramID 3ukdrhHrDPkirhTXArwU4AJjTXPafcY7XHeDCBDYLqKu
// ConfigAccount: Ci2mQYyAV2RA86dFMWazAQxutd3pX7vpoyPe2fv33S2u
// WhiteListAccount: HADJ37pxxkTpe4DHLJRdVXV6dFqm1iMCuxsH8K3pkstJ
// Initialize tx 2HNK6cGFtg1HCEjtda4m746qiTi3xesgmihCuPjKQrANTzH2FHn8iTscTFatdcPDyKQbihCTQASnopPRaE49Dk5H
