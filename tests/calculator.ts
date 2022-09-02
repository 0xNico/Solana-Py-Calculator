import * as anchor from '@project-serum/anchor'
import { BN, Program, web3, AnchorProvider } from '@project-serum/anchor'
import { findProgramAddressSync } from '@project-serum/anchor/dist/cjs/utils/pubkey'
const assert = require('assert')


import { Calculator } from '../target/types/calculator'


describe('calculator', () => {
  // Setting the anchor provider for the tests. 
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider)
  console.log(provider)

 // Setting the program constant
 const program = anchor.workspace.Calculator as Program<Calculator>
 console.log(program)
 
 // Create some accounts for testing purpose. 
 const owner = provider.wallet.publicKey
 const calculator = web3.PublicKey.findProgramAddressSync(
  [Buffer.from('Calculator'), owner.toBuffer()],
  program.programId
 )[0]

 // Attempt Calculator Initialization
 it('Inits the calculator', async () => {
  await program.methods.initCalculator().accounts({ owner, calculator }).rpc()
 })

 // Attempt some calculator operations
 it('Does some operations', async () => {
  // Addition
  const add2 = await program.methods
  .doOperation({ add: true }, new BN(2))
  .accounts({ owner, calculator })
  .instruction()
  // Multiplication
  const mul3 = await program.methods
  .doOperation({ mul: true }, new BN(3))
  .accounts({ owner, calculator })
  .instruction()
  // Subtraction
  const sub1 = await program.methods
  .doOperation({ sub: true }, new BN(1))
  .accounts({ owner, calculator})
  .instruction()
  
  // Build Transaction
  const tx = new web3.Transaction()
  tx.add(add2, mul3, sub1)
  await provider.sendAndConfirm(tx)

  // Get the on-chain data
  const calculatorAccount = await program.account.calculator.fetch(calculator)

  // Assertion 
  assert.ok(calculatorAccount.display.toNumber() === 5)
 })

 // Security tests for accounts
 it('Prevents fake transactions', async () => {
  let hacker = new web3.Keypair()

  let shouldFail = await program.methods
    .resetCalculator()
    .accounts({
      owner: hacker.publicKey,
      calculator,
    })
    .instruction()

  // Build Transaction
  let tx = new web3.Transaction()
  tx.add(shouldFail)
  await provider
    .sendAndConfirm(tx, [hacker])
    .then(() => assert.ok(false)) // Error on success, we want a failure. 
    .catch(console.log)
 })
}) // End of test describer
