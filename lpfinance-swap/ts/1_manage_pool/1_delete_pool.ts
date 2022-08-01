import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { LpfinanceSwap } from "../../target/types/lpfinance_swap";
import { SignerWallet } from "@saberhq/solana-contrib";

import {
  Connection,
} from "@solana/web3.js";

import {
    getCreatorKeypair
} from "../utils";
import { NETWORK } from "../config";
import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";

const delete_pool = async () => {
    
  const creatorKeypair = getCreatorKeypair();

  const connection = new Connection(NETWORK, "confirmed");
  const provider = new SignerWallet(creatorKeypair).createProvider(connection);
  // console.log(provider)
  anchor.setProvider(new anchor.AnchorProvider(connection, provider.wallet, anchor.AnchorProvider.defaultOptions()));
  const program = anchor.workspace.LpfinanceSwap as Program<LpfinanceSwap>;

  const poolAccounts = await program.account.poolInfo.all([
    {
      memcmp: {
        offset: 12,
        bytes: bs58.encode(Buffer.from('pool')),
      }
    }
  ]);

  if (poolAccounts.length > 0){
    await program.rpc.deletePool({
        accounts: {
            pool: poolAccounts[0].publicKey,
            creator: poolAccounts[0].account.creator,
        },
    });
    console.log("Delete a Pool completed!");
    console.log("deleted pool pubkey: ", poolAccounts[0].publicKey.toBase58())
    console.log("remained :", poolAccounts.length-1);
  }else{
    console.log("There are not pools")
  }
};

delete_pool();
