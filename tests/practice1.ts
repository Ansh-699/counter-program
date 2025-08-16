import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { Practice1 } from "../target/types/practice1";

describe("practice1", () => {
  // Configure the client to use the local cluster
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.Practice1 as Program<Practice1>;

  it("Initialize + increment + increment_data + decrement", async () => {
    // PDA for initialized account
    const [pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("initialize_account")],
      program.programId
    );

    // Call initialize
    await program.methods.initialize().accounts({
      signer: provider.wallet.publicKey,
      initialized_account: pda,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    // increment (sets data to 0)
    await program.methods.increment().accounts({
      signer: provider.wallet.publicKey,
      initialized_account: pda,
    }).rpc();

    // Fetch account state, should be 0
  let state = await program.account.accountState.fetch(pda);
  assert.equal(state.data, 0);

    // increment_data (should increment to 1)
    await program.methods.incrementData().accounts({
      initialized_account: pda,
      signer: provider.wallet.publicKey,
    }).rpc();

    state = await program.account.accountState.fetch(pda);
    assert.equal(state.data, 1);

    // Decrement (should decrement to 0)
    await program.methods.decrement().accounts({
      initialized_account: pda,
      signer: provider.wallet.publicKey,
    }).rpc();

    state = await program.account.accountState.fetch(pda);
    assert.equal(state.data, 0);
  });
});
