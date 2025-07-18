import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { LiquidStaking } from "../target/types/liquid_staking";
import { LAMPORTS_PER_SOL, PublicKey, TransactionConfirmationStrategy } from "@solana/web3.js"
import { createMint } from "@solana/spl-token";

describe("liquidity_staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.liquidStaking as Program<LiquidStaking>;
  const admin = provider.wallet as anchor.Wallet;
  const user = anchor.web3.Keypair.generate();
  
  const exchange_rate: number = 1.0;
  const unbound_delay: BN = new BN(86400);

  let stakingTokenMint: PublicKey;
  let underlayingMint: PublicKey;

  before(async () => {
    await Promise.all(
      ([admin, user].map(keypair => {
        return provider.connection
          .requestAirdrop(keypair.publicKey, 10 * LAMPORTS_PER_SOL)
          .then((signature) => {
            provider.connection.confirmTransaction(
              {signature} as TransactionConfirmationStrategy,
                "processed"
              );
          })
      }))
    )

    stakingTokenMint = await createMint(
      connection,
      admin.payer,
      admin.publicKey,
      admin.publicKey,
      6
    );

    underlayingMint = await createMint(
      connection,
      admin.payer,
      admin.publicKey,
      admin.publicKey,
      6
    );


  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize(exchange_rate, unbound_delay)
      .accounts({
        signer: admin.publicKey,
        stakingTokenMint,
        underlayingMint,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
