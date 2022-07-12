import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { CbsProtocol } from "../target/types/cbs_protocol";

describe("cbs-protocol", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.CbsProtocol as Program<CbsProtocol>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
