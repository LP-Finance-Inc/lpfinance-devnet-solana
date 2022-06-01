import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { TestTokens } from "../target/types/test_tokens";

describe("test-tokens", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TestTokens as Program<TestTokens>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
