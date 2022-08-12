import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StableSwap } from "../target/types/stable_swap";

describe("stable-swap", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.StableSwap as Program<StableSwap>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
