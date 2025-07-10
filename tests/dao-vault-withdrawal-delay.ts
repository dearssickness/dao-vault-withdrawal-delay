import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DaoVaultWithdrawalDelay } from "../target/types/dao_vault_withdrawal_delay";

describe("dao-vault-withdrawal-delay", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.daoVaultWithdrawalDelay as Program<DaoVaultWithdrawalDelay>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
