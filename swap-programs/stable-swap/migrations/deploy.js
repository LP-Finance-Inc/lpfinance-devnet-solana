// Migrations are an early feature. Currently, they're nothing more than this
// single deploy script that's invoked from the CLI, injecting a provider
// configured from the workspace's Anchor.toml.

const anchor = require("@project-serum/anchor");
const zip = require("lodash.zip");

const invariant = require("tiny-invariant");

const { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createMint, createAccount } = require('@solana/spl-token')
const { 
  PublicKey, SystemProgram, SYSVAR_RENT_PUBKEY, LAMPORTS_PER_SOL, 
  sendAndConfirmTransaction,
  Transaction, Connection, clusterApiUrl, Keypair 
} = anchor.web3;

const idl = require("../target/idl/stable_swap.json");
const programID = idl.metadata.address;
const TOKEN_DECIMALS = 9;
const DEFAULT_TOKEN_DECIMALS = 6;

console.log("ProgramID", programID);
const PREFIX = "stableswap";

// Test Token's MINT
const admin = new PublicKey("DZJf5yDnQ2boy4ZVHHvKhRyinKAy8mS6JCm8U8DaDeEx"); 
const swapProgram = new PublicKey("DhxTG5bfGbCrGgX3ZfTRnFTbkjK8XZG9a1FP7RHxb7Sk"); 

const fees_a = new PublicKey("6wYCtwPtNAnKnMzPHcwNmrW5gfrz7fMa1newupYxfKEi");
const mintA = new PublicKey("8z9n6D5pJgdoLsHgnXtvXtVdsYveg7WJPyvoFoHT2DHd"); // 10000

const fees_b = new PublicKey("BFEsqdpVABGvpGzVWiiAGBX4b1k1K2mDXfzSGwfQb8Zt");
const mintB = new PublicKey("4hAwncYcvBTxgYPJZ1y6dHHSuzNJc1KLEzM9pgx2Jh13"); // 10000

const createPDAAccount = async (keypair, accountKeypair,newAccountPubkey, space, connection) => {
  const transaction = new Transaction();

  const instruction = SystemProgram.createAccount({
    fromPubkey: keypair.publicKey,
    newAccountPubkey: newAccountPubkey,
    space,
    lamports: 0,
    programId: swapProgram,
  });
  transaction.add(instruction);
  var signature = await sendAndConfirmTransaction(
     connection, 
     transaction, 
     [keypair, accountKeypair]);
  console.log(signature);
}

const afterDeployed = async (provider) => {
  try {
    // Configure client to use the provider.
    anchor.setProvider(provider);
    const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');
    // Add your deploy script here
    const program = new anchor.Program(idl, programID);

    const swap = new PublicKey("vxs9hBmmmmaAT7DBZSgmSWrQz9VDhNc8X5pCN1giFvo");
    const swapAuthority = new PublicKey("FFdngKSLBzQGUqyHfgqzavFEFqjZ9yP1RFNEMgNc88QR");
    const reserveA = new PublicKey("GtfC9zF4nqurmTw51n8NVaASrpm1jSXvhaRTNCgCm4a2");
    const reserveB = new PublicKey("7WgjrBZwULVo3p4gRmoKELkTYsCZWgkgz1J6SZi41kNq");
    const mintLP = new PublicKey("CdbxmZDQy7dZ6MMM8D1mZQHaDpkiAUGqmD331EXoL9Dv");
    const outputLp = new PublicKey("GjTiFFtVJXkbXGKQzpSWVNhQNff2VorcpTGTVtSa8tKU");
    const nonce = 253;
    // Signer
    const authority = provider.wallet.publicKey;

    const amp_factor = new anchor.BN("1000");
    const swap_fee_numerator = new anchor.BN("100"); // 0.1%
    const swap_fee_denominator = new anchor.BN("100000");
    const zero_denominator = new anchor.BN("100");
    const zero_numerator = new anchor.BN("0");
    const fees = {
      admin_trade_fee_numerator: zero_numerator,
      admin_trade_fee_denominator: zero_denominator,
      admin_withdraw_fee_numerator: zero_numerator,
      admin_withdraw_fee_denominator: zero_denominator,
      trade_fee_numerator: swap_fee_numerator,
      trade_fee_denominator: swap_fee_denominator,
      withdraw_fee_numerator: zero_numerator,
      withdraw_fee_denominator: zero_denominator
    }

    const [sortedMintA, sortedReserveA, sortedMintB, sortedReserveB, sortedFeeA, sortedFeeB] =
    comparePubkeys(mintA, mintB) !== -1
      ? [mintB, reserveB, mintA, reserveA, fees_b, fees_a]
      : [mintA, reserveA, mintB, reserveB, fees_a, fees_b];

    console.log(sortedReserveA.toBase58(), sortedMintB.toBase58())
    // console.log(program)
    await program.rpc.createPool( 
      nonce,
      amp_factor,
      fees, 
      {
        accounts: {
          swap,
          admin,
          swapAuthority: swapAuthority,
          tokenA: { 
            reserve: sortedReserveA,
            fees: sortedFeeA,
            mint: sortedMintA
          },
          tokenB: { 
            reserve: sortedReserveB,
            fees: sortedFeeB,
            mint: sortedMintB
          },
          poolMint: mintLP,
          outputLp: outputLp,
          tokenProgram: TOKEN_PROGRAM_ID,
          swapProgram,
          systemProgram: SystemProgram.programId,        
        }
    });
  } catch (err) {
    console.log(err)
    // console.log("Error", err.programErrorStack[0].toBase58())
  }
}

module.exports = async function (provider) {
  // await afterDeployed();
  // return;
  
  // Configure client to use the provider.
  anchor.setProvider(provider);
  const connection = new Connection(clusterApiUrl('devnet'), 'confirmed');

  const fromWallet = Keypair.generate();
  // Airdropping tokens to a payer.
  await provider.connection.confirmTransaction(
    await provider.connection.requestAirdrop(fromWallet.publicKey, LAMPORTS_PER_SOL),                                                        
    "confirmed"
  );

  // Add your deploy script here
  const program = new anchor.Program(idl, programID);
  // Signer
  const authority = provider.wallet.publicKey;
  const swap_account = anchor.web3.Keypair.generate();
  const swap = swap_account.publicKey;
  console.log("swap", swap.toBase58());
  await createPDAAccount(fromWallet,swap_account, swap, 395, connection);

  const [swapAuthority, nonce] = await PublicKey.findProgramAddress(
    [Buffer.from(swap.toBuffer())],
    swapProgram
  );
  console.log("swapAuthority", swapAuthority.toBase58(), nonce);
  
  const auxiliaryKeypairA = Keypair.generate();
  const reserveA = await createAccount(
    connection,
    fromWallet,
    mintA,
    swapAuthority,
    auxiliaryKeypairA
  )
  console.log("reserveA", reserveA.toBase58())

  const auxiliaryKeypairB = Keypair.generate();
  const reserveB = await createAccount(
    connection,
    fromWallet,
    mintB,
    swapAuthority,
    auxiliaryKeypairB
  )
  console.log("reserveB", reserveB.toBase58())

  const mintLP = await createMint(
    connection,
    fromWallet,
    swapAuthority,
    null,
    DEFAULT_TOKEN_DECIMALS
  );

  console.log("mintLP", mintLP.toBase58())

  const auxiliaryKeypairOutputLp = Keypair.generate();
  const outputLp = await createAccount(
    connection,
    fromWallet,
    mintLP,
    swapAuthority,
    auxiliaryKeypairOutputLp
  )
  console.log("outputLp", outputLp.toBase58())

  const [sortedMintA, sortedReserveA, sortedMintB, sortedReserveB] =
  comparePubkeys(mintA, mintB) !== -1
    ? [mintB, reserveB, mintA, reserveA]
    : [mintA, reserveA, mintB, reserveB];


  const amp_factor = new anchor.BN("1000");
  const swap_fee_numerator = new anchor.BN("100"); // 0.1%
  const swap_fee_denominator = new anchor.BN("100000");
  const zero_denominator = new anchor.BN("100");
  const zero_numerator = new anchor.BN("0");
  const fees = {
    admin_trade_fee_numerator: zero_numerator,
    admin_trade_fee_denominator: zero_denominator,
    admin_withdraw_fee_numerator: zero_numerator,
    admin_withdraw_fee_denominator: zero_denominator,
    trade_fee_numerator: swap_fee_numerator,
    trade_fee_denominator: swap_fee_denominator,
    withdraw_fee_numerator: zero_numerator,
    withdraw_fee_denominator: zero_denominator
  }

  // console.log(program)
  await program.rpc.createPool( 
    nonce,
    amp_factor,
    fees, 
    {
      accounts: {
        authority,
        swap,
        admin,
        swapAuthority: swapAuthority,
        tokenA: { 
          reserve: sortedReserveA,
          fees: fees_a,
          mint: sortedMintA
        },
        tokenB: { 
          reserve: sortedReserveB,
          fees: fees_b,
          mint: sortedMintB
        },
        poolMint: mintLP,
        outputLp: outputLp,
        tokenProgram: TOKEN_PROGRAM_ID,
        swapProgram,
        systemProgram: SystemProgram.programId,        
      },
      signers: [swap_account]
  });
}

const comparePubkeys = (a, b) => {
  const bytesA = a.toBytes();
  const bytesB = b.toBytes();
  return (
    zip(bytesA, bytesB)
      .map(([a, b]) => {
        invariant(typeof a === "number" && typeof b === "number", "a and b");
        if (a > b) {
          return 1;
        } else if (a < b) {
          return -1;
        }
        return null;
      })
      .find((x) => x !== null) ?? 0
  );
};