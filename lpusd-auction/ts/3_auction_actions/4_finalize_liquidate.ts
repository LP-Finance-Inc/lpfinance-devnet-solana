import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpusdAuction } from "../../target/types/lpusd_auction";
import {
  Connection,
  SYSVAR_RENT_PUBKEY, PublicKey
} from "@solana/web3.js";

import { 
    CBSProtocolIDL,
    CBS_PREFIX,
    NETWORK, 
    PREFIX
} from "../config";

import { getCreatorKeypair, getPublicKey, print_config_data, print_user_data } from "../utils";

const { Wallet } = anchor;

const finalize_liquidate = async () => {
  const connection = new Connection(NETWORK, "confirmed");

  const creatorKeypair = getCreatorKeypair();

  anchor.setProvider(new anchor.AnchorProvider(connection, new Wallet(creatorKeypair), anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpusdAuction as Program<LpusdAuction>;
  // Config
  const config = getPublicKey('auction_config');  

  const PDA = await PublicKey.findProgramAddress(
    [Buffer.from(PREFIX)],
    program.programId
  );


  const [userAccount, bump] = await PublicKey.findProgramAddress(
    [Buffer.from(PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
    program.programId
  );

  const cbsProgramId = new PublicKey(CBSProtocolIDL.metadata.address);
  const [cbsAccount, cbsBump] = await PublicKey.findProgramAddress(
    [Buffer.from(CBS_PREFIX), Buffer.from(creatorKeypair.publicKey.toBuffer())],
    cbsProgramId
  );

  // STEP: 7
  const tx7 = await program.rpc.distributeRewardFromLiquidate({
    accounts: {
      owner: creatorKeypair.publicKey,
      config,
      cbsAccount,
      auctionPda: PDA[0],
      cbsProgram: cbsProgramId,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: SYSVAR_RENT_PUBKEY
    },
  });

  console.log("Distribute reward successfully", tx7)

  const auctionConfigDataAfterDeposit = await program.account.config.fetch(config);
  print_config_data(auctionConfigDataAfterDeposit)

  const userData = await program.account.userAccount.fetch(userAccount);
  print_user_data(userData)
}

finalize_liquidate();
