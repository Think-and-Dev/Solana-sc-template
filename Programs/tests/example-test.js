const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe('example-test', () => {

  // Use a local provider.
  const provider = anchor.Provider.local();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.ExampleTest;

  it('Creates and initializes an account in a single transaction (simplified)',async ()=>{

    //THE ACCOUNT TO CREATE
    const myAccount = anchor.web3.Keypair.generate();

    await program.rpc.initialize(new anchor.BN(1234), {
        accounts: {
          myAccount: myAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [myAccount],
    });

    // Fetch the newly created account from the cluster.
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check it's state was initialized.
    assert.ok(account.data.eq(new anchor.BN(1234)));
    
    // Store the account for the next test.
    _myAccount = myAccount;

  });

  it("Updates a previously created account", async () => {
    const myAccount = _myAccount;


    // Invoke the update rpc.
    await program.rpc.update(new anchor.BN(4321), {
      accounts: {
        myAccount: myAccount.publicKey,
      },
    });

    // Fetch the newly updated account.
    const account = await program.account.myAccount.fetch(myAccount.publicKey);

    // Check it's state was mutated.
    assert.ok(account.data.eq(new anchor.BN(4321)));

    // #endregion update-test
  });


});
