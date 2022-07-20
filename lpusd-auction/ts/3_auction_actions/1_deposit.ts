import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpusdAuction } from "../../target/types/lpusd_auction";
import {
  Connection,
  SYSVAR_RENT_PUBKEY, PublicKey
} from "@solana/web3.js";

import { 
  NETWORK, 
  PREFIX
} from "../config";

import { convert_to_wei, getATAPublicKey, getCreatorKeypair, getPublicKey, print_config_data, print_user_data } from "../utils";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

const { Wallet } = anchor;

const deposit = async () => {
  const connection = new Connection(NETWORK, "confirmed");

  const creatorKeypair = getCreatorKeypair();

  anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpusdAuction as Program<LpusdAuction>;

  // Config
  const config = getPublicKey('auction_config');  
  const auctionConfigData = await program.account.config.fetch(config);
  const lpusdMint= auctionConfigData.lpusdMint as PublicKey;
  const poolLpusd= auctionConfigData.poolLpusd as PublicKey;
  const userLpusd = await getATAPublicKey(lpusdMint, creatorKeypair.publicKey);

  const [userAccount, bump] = await PublicKey.findProgramAddress(
    [Buffer.from(PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
    program.programId
  );

  const PDA = await PublicKey.findProgramAddress(
    [Buffer.from(PREFIX)],
    program.programId
  );    

  const deposit_wei = convert_to_wei("100");
  const deposit_amount = new anchor.BN(deposit_wei);
  
  await program.rpc.depositLpusd(deposit_amount, {
    accounts: {
      userAuthority: creatorKeypair.publicKey,
      auctionPda: PDA[0],
      config,
      lpusdMint,
      userLpusd,
      poolLpusd,
      userAccount,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: SYSVAR_RENT_PUBKEY,
    },
  });

  console.log("Deposit successfully")

  const auctionConfigDataAfterDeposit = await program.account.config.fetch(config);
  print_config_data(auctionConfigDataAfterDeposit)

  const userData = await program.account.userAccount.fetch(userAccount);
  print_user_data(userData)
}

deposit();

