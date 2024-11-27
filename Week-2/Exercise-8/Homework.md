# Exercise 8

## SOLANA TOKENS

This section needs us to create and experiment with Fungible and Non-Fungible Tokens using `spl-token-cli`.

Following are the tasks that need to be performed:

1. Create a fungible token with supply of 10,000
2. Create a Non-Fungible Token
3. Try sending these tokens to your teams
4. Also check upon the details of these tokens
5. Do use `transfer` and `transfer --fund-recipient` commands to send tokens

All these can be done by taking [this](https://solana.bootcampnotes.xyz/lesson6.html#/5) particular lesson in consideration.

## SOLANA PROGRAMS

Here we have to solve `examples_baremetal` directory of the [SolanaBootcamp Repository](https://github.com/ExtropyIO/SolanaBootcamp). For this exercise, first two programs are to be solved.

However, we have to set-up our environment first in order to run these programs smoothly. For this, do check out the given README.md file in the repository.

Now, the main issue occurred (for me) when I was trying to run `npm run call:1` command for interacting with `example1-helloworld` program after it has been deployed. Some functions from `@solana/web3.js` were deprecated and hence I had to make some changes in the code. Here's how [utils.ts](https://github.com/0xScratch/SolanaBootcamp/blob/main/utils/utils.ts) will look after the changes has been made and now things gonna work right, atleast it did for me!

**Note**: Here's the [commit](https://github.com/0xScratch/SolanaBootcamp/commit/4964ea88a7fb2366175360a7f9479f4b4597d8f8#diff-34b033806cd95cba13daa6c56736722fb1f3b9de424bb99ba34ed00336b00131R13) in order to check out the changes made in the code.

### Example 1 - Hello World

There were two instructions give for this program:

- Modify the existing `msg!` in the program to log `program ID` -> [Solution](https://github.com/0xScratch/SolanaBootcamp/blob/4964ea88a7fb2366175360a7f9479f4b4597d8f8/examples_baremetal/example1-helloworld/rust/src/lib.rs#L14)
- Retrieve the total program size.

To retrieve the program size, we can run command `solana program show <PROGRAM_ID>` and it will give the following output:

```bash
Program Id: 7hAWRYH1FuQPsmyVRiJ9JJY6bTkj1JtEVq5THvzGHwLg
Owner: BPFLoaderUpgradeab1e11111111111111111111111
ProgramData Address: DayEtJvJcBoDMHTKJh3pJrPkwkEg452kRB6T9wa8PMyJ
Authority: 2zYNtrtrq5vq76H19q2JDSr7mUCuAAhMA4xC26yjaCMP
Last Deployed In Slot: 4086
Data Length: 35344 (0x8a10) bytes
Balance: 0.24719832 SOL
```

As you can see, the `Data Length` is `35344` bytes which is the total program size.

### Example 2 - Counter

Again, two instructions were given for this program:

- Retrieve the lamports balance of the program. -> `solana balance <PROGRAM_ID>`
- Modify the client to feed an incorrect address for the greeting account of the program.

Normally the program will greet our deterministically created accounts as long as we are using the same address which has been used to deploy the program and thus called as the owner. Hence, the exercise wants us to feed an incorrect address (not owner or not linked to owner) to the program and display [this](https://github.com/0xScratch/SolanaBootcamp/blob/8898b89b3910527b9973a59c9708947edd2c5437/examples_baremetal/example2-counter/rust/src/lib.rs#L38) error.

That's how it has been solved by modifying the client [here](https://github.com/0xScratch/SolanaBootcamp/blob/main/examples_baremetal/example2-counter/client/main.ts). you can easily notice the changes made, especially the variable which has been used a lot inside `deployGreetAccount` asynchronous function - `incorrectGreetedPubkey`.
