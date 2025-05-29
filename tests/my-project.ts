import * as anchor from '@coral-xyz/anchor';
import { assert } from 'chai';
import BN from 'bn.js';


describe("my_project", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.myProject;
  const calculator = anchor.web3.Keypair.generate();

  it("Creates a calculator", async () => {
    await program.methods.create("Hello Anchor!").accounts({
      calculator: calculator.publicKey,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([calculator]).rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.greeting.length > 0);
    assert.strictEqual(account.greeting, "Hello Anchor!");
  });

  it('adds two numbers', async () => {
    await program.methods.add(new BN(2), new BN(3)).accounts({
      calculator: calculator.publicKey,
    }).rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new BN(5))); // Use .eq() to compare BN objects
  });

  it('subtracts two numbers', async () => {
    await program.methods.substract(new BN(32), new BN(33)).accounts({
      calculator: calculator.publicKey,
    }).rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new BN(-1)));
  })

  it('multiplies two numbers', async () => {
    await program.methods.multiply(new BN(2), new BN(3)).accounts({
      calculator: calculator.publicKey,
    }).rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new BN(6)));
  })

  it('divides two numbers', async () => {
    await program.methods.divide(new BN(10), new BN(3)).accounts({
      calculator: calculator.publicKey,
    }).rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.result.eq(new BN(3)));
    assert.ok(account.reminder.eq(new BN(1)));
  })

  it('finds the remainder of two numbers', async () => {
    await program.methods.remainder(new BN(10), new BN(3)).accounts({
      calculator: calculator.publicKey,
    }).rpc();

    const account = await program.account.calculator.fetch(calculator.publicKey);
    assert.ok(account.reminder.eq(new BN(1)));
  })
});
