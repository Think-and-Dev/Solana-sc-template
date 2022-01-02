# Solana programs template with Anchor

### Resources

* [Official docs](https://project-serum.github.io/anchor/getting-started/introduction.html)

********************

### Programs

1. Basic0
2. Arguments (example_test)
2. Accounts validation (account_validate)
3. Cross Program Invocation (cpi) ---> WIP
  * puppet_master: 
  * puppet:

### Start a proyect

```anchor init new-project-name```

### Build a program

```anchor build```

* Once run, you should see your build artifacts, as usual, in your target/ directory. Additionally, a target/idl/basic_0.json: 

```json
{
  "version": "0.0.0",
  "name": "basic",
  "instructions": [
    {
      "name": "initialize",
      "accounts": [],
      "args": []
    }
  ]
}
```
### Deploying

* To deploy to a local cluster you must start your node first, and have generated a key-pair

```
solana-keygen new -o ~/.config/solana/id.json
solana airdrop 4
solana-test-validator
anchor deploy
```

### Workspaces

* The workspace namespace provides access to all programs in the local project and is automatically updated to reflect the latest deployment, making it easy to change your program, update your JavaScript, and run your tests in a fast feedback loop.

* When you run anchor init name_workspace you are actually creating a workspaces where your programs will live.

### Test

```anchor test``

* In case you want to test the typescript file, you could run: 

```anchor run test_ts```

### Arguments and accounts 

* **IMPORTANT INFORMATION**: 

>If you've developed on Solana before, you might notice two things 1) the ordering of the accounts doesn't matter and 2) the isWritable and isSigner options are not specified on the account anywhere. In both cases, the framework takes care of these details for you, by reading the IDL.















