import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Airdrop } from "../target/types/airdrop";
import { assert } from "chai";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";

describe("airdrop", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const provider=anchor.getProvider();
  const recipient=anchor.web3.Keypair.generate();

  const program = anchor.workspace.airdrop as Program<Airdrop>;

  it("Is initialized!", async () => {

    // await provider.connection.requestAirdrop(recipient.publicKey,1*LAMPORTS_PER_SOL)

    const tx=await program.methods.airdrop(new anchor.BN(1*LAMPORTS_PER_SOL)).accounts({
      sender:provider.publicKey,
      recipient:recipient.publicKey
    }).rpc();

    console.log("Your transaction signature:",tx);
    const account=await provider.connection.getAccountInfo(recipient.publicKey);
    console.log('Account:',account)
    console.log('Recipient:',recipient.publicKey.toBase58())
    console.log(provider.publicKey.toBase58())
    assert.equal(account.lamports,1*LAMPORTS_PER_SOL)

  });
});
