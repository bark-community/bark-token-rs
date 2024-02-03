describe("Test", () => {
  it("Airdrop", async () => {
    // Fetch my balance
    const balance = await pg.connection.getBalance(pg.wallet.publicKey);
    console.log(`My balance is ${balance} lamports`);

    // Airdrop 2 SOL
    const airdropAmount = 2 * web3.LAMPORTS_PER_SOL;
    const txHash = await pg.connection.requestAirdrop(
      pg.wallet.publicKey,
      airdropAmount
    );

    // Confirm transaction
    await pg.connection.confirmTransaction(txHash);

    // Fetch new balance
    const newBalance = await pg.connection.getBalance(pg.wallet.publicKey);
    console.log(`New balance is ${newBalance} lamports`);

    // Assert balances
    assert(balance + airdropAmount === newBalance);
  });
});
