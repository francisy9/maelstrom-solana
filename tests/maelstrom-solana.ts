import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MaelstromSolana } from "../target/types/maelstrom_solana";
import assert from "assert";
import { PublicKey, Keypair, Transaction } from "@solana/web3.js";

describe("maelstrom-solana", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MaelstromSolana as Program<MaelstromSolana>;
  const programId = program.programId;
  const authority = provider.wallet;

  const maliciousUser = Keypair.generate();
  let programDataAccount;

  it("Finding program data account address", async () => {
    [programDataAccount] = await PublicKey.findProgramAddressSync(
      [programId.toBuffer()],
      new PublicKey("BPFLoaderUpgradeab1e11111111111111111111111")
    );
  })

  it("Fails when malicious user attempts to initialize token mint", async () => {
    try {
      const { blockhash, lastValidBlockHeight } = await program.provider.connection.getLatestBlockhash();

      const airdropTx = await program.provider.connection.requestAirdrop(maliciousUser.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
      await program.provider.connection.confirmTransaction({
        signature: airdropTx,
        blockhash, 
        lastValidBlockHeight,
      });

      const tx = new Transaction();
      const ixs = await program.methods
        .initializeTokenMint()
        .accounts({
          authority: maliciousUser.publicKey,
          programData: programDataAccount,
        })
        .instruction();

      tx.add(ixs);
      tx.recentBlockhash = blockhash;
      tx.sign(maliciousUser);

      const txSignature = await program.provider.connection.sendRawTransaction(tx.serialize());
      await program.provider.connection.confirmTransaction({
        signature: txSignature,
        blockhash,
        lastValidBlockHeight,
      });

      assert.fail("Expected an error, but the transaction succeeded");
    } catch (error) {
      // Note: if account is already initialized, that error message precedes the raw constraint violation
      checkErrorKeyword(error.message, "0x7d3");
    }
  });
  
  it("Token mint account is initialized!", async () => {
    const tx = await program.methods
      .initializeTokenMint()
      .accounts({
        programData: programDataAccount,
        authority: authority.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
  
  it("Fails to reinitialize token mint", async () => {
    try {
      await program.methods
        .initializeTokenMint()
        .accounts({
          programData: programDataAccount,
          authority: authority.publicKey,
        })
        .rpc();
      assert.fail("Expected an error, but the transaction succeeded");
    } catch (error) {
      checkErrorKeyword(error.message, "0x0");
    }
  });
  
});


function checkErrorKeyword(errorMessage, errorKeyword) {
  assert(errorMessage.includes(errorKeyword));
}