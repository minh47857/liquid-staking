import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { LiquidStaking } from "../target/types/liquid_staking";
import { LAMPORTS_PER_SOL, PublicKey, TransactionConfirmationStrategy } from "@solana/web3.js"
import { AuthorityType, createMint, getOrCreateAssociatedTokenAccount, mintTo, setAuthority, getAccount, Account, getAssociatedTokenAddress } from "@solana/spl-token";
import { POOL_CONFIG_SEED, POOL_SEED, USER_UNBOUND_REQUEST_SEED } from "./constants";
import { assert } from "chai";
import { setTimeout } from "timers/promises";

describe("liquidity_staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.liquidStaking as Program<LiquidStaking>;
  const admin = provider.wallet as anchor.Wallet;
  const user = anchor.web3.Keypair.generate();

  const unbound_delay: BN = new BN(5);

  let stakingTokenMint: PublicKey;
  let underlayingMint: PublicKey;

  let userUnderlyingTokenAccount: Account;
  let poolUnderlayingAccount: Account;
  let adminUnderlayingAccount: Account;

  let userStakingTokenAccount: Account;

  let poolConfigPda: PublicKey;
  let poolPda: PublicKey;
  let userUnboundRequestPda: PublicKey;

  before(async () => {
    await Promise.all(
      ([admin, user].map( async (keypair) => {
        return provider.connection
          .requestAirdrop(keypair.publicKey, 100 * LAMPORTS_PER_SOL)
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

    [poolConfigPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(POOL_CONFIG_SEED), stakingTokenMint.toBytes(), underlayingMint.toBytes()],
      program.programId,
    );

    [poolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(POOL_SEED), stakingTokenMint.toBytes(), underlayingMint.toBytes()],
      program.programId,
    );

    [userUnboundRequestPda] = PublicKey.findProgramAddressSync(
      [Buffer.from(USER_UNBOUND_REQUEST_SEED), user.publicKey.toBytes(), poolPda.toBytes()],
      program.programId,
    );
    
    userUnderlyingTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      admin.payer,
      underlayingMint,
      user.publicKey
    );

    adminUnderlayingAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      admin.payer,
      underlayingMint,
      admin.publicKey
    );
  
    userStakingTokenAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      admin.payer,
      stakingTokenMint,
      user.publicKey
    );

    await mintTo(
      connection,
      admin.payer,
      underlayingMint,
      userUnderlyingTokenAccount.address,
      admin.payer,
      20 * 1e6,
    );

    await mintTo(
      connection,
      admin.payer,
      underlayingMint,
      adminUnderlayingAccount.address,
      admin.payer,
      20 * 1e6,
    );
  });

  it("initialized!", async () => {
    const tx = await program.methods
      .initialize(unbound_delay)
      .accounts({
        signer: admin.publicKey,
        stakingTokenMint,
        underlayingMint,
      })
      .rpc();
    console.log("Your transaction signature", tx);

    await setAuthority(
      connection,
      admin.payer,
      stakingTokenMint,
      admin.publicKey,
      AuthorityType.MintTokens,
      poolConfigPda,
    );
  });

  it("It stake", async () => {
    poolUnderlayingAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      admin.payer,
      underlayingMint,
      poolConfigPda,
      true,
    );
    
    console.log(program.programId, poolConfigPda, poolPda);

    let poolConfig = await program.account.poolConfig.fetch(poolConfigPda);
    let pool = await program.account.pool.fetch(poolPda);

    assert.equal(pool.totalStaked.toNumber(), 0);

    await program.methods
      .stake(new BN(5 * 1e6))
      .accounts({
        signer: user.publicKey,
        stakingTokenMint,
        underlayingMint,
      })
      .signers([user])
      .rpc();
      
    pool = await program.account.pool.fetch(poolPda);
    
    console.log("after staked", pool.totalStaked.toNumber());
    console.log(
      "user staking token account amount: ",
      (await getAccount(connection, userStakingTokenAccount.address)).amount.toString()
    )
    console.log(
      "pool underlaying token account amount: ",
      (await getAccount(connection, poolUnderlayingAccount.address)).amount.toString()
    )
  });

    it("add reward",async() => {
      poolUnderlayingAccount = await getOrCreateAssociatedTokenAccount(
        connection,
        admin.payer,
        underlayingMint,
        poolConfigPda,
        true,
      );

      await program.methods
      .addReward(new BN(5000))
      .accounts({
        admin: admin.publicKey,
        stakingTokenMint,
        underlayingMint,
      })
      .rpc();

      let pool = await program.account.pool.fetch(poolPda);
      console.log("pool accumulated reward: ", pool.accumulatedReward.toNumber());
    });

  it("unstake", async () => {
    const UNSTAKE_AMOUNT = 2 * 1e6;

    const poolBefore = await program.account.pool.fetch(poolPda);
    const userStakingTokenBefore = await connection.getTokenAccountBalance(userStakingTokenAccount.address);
    console.log(poolBefore.totalStaked.toNumber());

    const currentExchangeRate = (5 * 1e6 + 5 * 1e3) / (5 * 1e6);

  //   const simulation = await program.methods
  //   .unstake(new BN(2 * 1e6))
  //   .accounts({
  //     signer: user.publicKey,
  //     stakingTokenMint,
  //     underlayingMint,
  //   })
  //   .signers([user])
  //   .simulate();

  // // Luôn hiển thị logs
  // console.log("=== PROGRAM LOGS ===");
  // if (simulation.raw) {
  //   simulation.raw.forEach((log, index) => {
  //     if (log.includes('Program log:')) {
  //       console.log(`${index}: ${log}`);
  //     }
  //   });
  // }

    await program.methods
      .unstake(new BN(UNSTAKE_AMOUNT))
      .accounts({
        signer: user.publicKey,
        stakingTokenMint,
        underlayingMint,
      })
      .signers([user])
      .rpc();

    const poolAfter = await program.account.pool.fetch(poolPda);
    const userStakingTokenAfter = await connection.getTokenAccountBalance(userStakingTokenAccount.address);

    console.log(poolAfter.totalStaked.toNumber(), poolAfter.exchangeRate);

    assert.equal(
      poolBefore.totalStaked.toNumber(),
      (poolAfter.totalStaked.toNumber() +  UNSTAKE_AMOUNT * currentExchangeRate)
    );

    assert.equal(
      Number(userStakingTokenBefore.value.amount),
      Number(userStakingTokenAfter.value.amount) + UNSTAKE_AMOUNT
    );
  });

  it("withdraw", async () => {
    try {
      await program.methods
      .withdraw()
      .accounts({
        signer: user.publicKey,
        stakingTokenMint,
        underlayingMint,
      })
      .signers([user])
      .rpc();
      
      assert.fail("should throw error");
    } catch (error) {
      console.log(error);
    }

    const balanceBefore = await connection.getTokenAccountBalance(userUnderlyingTokenAccount.address);
    const userUnboundRequestBefore = await program.account.userUnboundRequest.fetch(userUnboundRequestPda);
    console.log(`balance before: ${Number(balanceBefore.value.amount)}` , userUnboundRequestBefore)

    await setTimeout(6000);

    await program.methods
    .withdraw()
    .accounts({
      signer: user.publicKey,
      stakingTokenMint,
      underlayingMint,
    })
    .signers([user])
    .rpc();

    const userUnboundRequestAfter = await program.account.userUnboundRequest.fetch(userUnboundRequestPda);
    const balanceAfter = await connection.getTokenAccountBalance(userUnderlyingTokenAccount.address);
    console.log(`balance after: ${Number(balanceAfter.value.amount)}` , userUnboundRequestAfter)
  });

  it("add more reward", async () => {
    const NEW_REWARD = 5000;
    await program.methods
      .addReward(new BN(NEW_REWARD))
      .accounts({
        admin: admin.publicKey,
        stakingTokenMint,
        underlayingMint
      })
      .rpc();

    const pool = await program.account.pool.fetch(poolPda);
    assert.equal(pool.accumulatedReward.toNumber(), NEW_REWARD * 2);
  })
});
