# Exercise 9

## CPI Example

There wasn't much to do in here other than just understand how's the code working and what it is doing which I will explain right in here.

1. First thing comes up, what is CPI? CPI stands for [Cross Program Invocation](https://solana.com/docs/core/cpi#cross-program-invocations). It is just a way to call another program from the current program. That's it, nothing fancy (however, solana do kept this name fancy and made it sound ultra technical for just a simple thing 😂).

2. So, using this `example4-cpi` program we are calling the program which resides in `example1-helloworld`. Here the main code is present in `entrypoint.rs` file and `lib.rs` contains entrypoint module.

3. If you paid attention to the previous examples, most of the things are crystal clear. The only thing which is new here how a new instruction is assembled at line [27](https://github.com/0xScratch/SolanaBootcamp/blob/8898b89b3910527b9973a59c9708947edd2c5437/examples_baremetal/example4-cpi/rust/src/entrypoint.rs#L27) and then it got invoked on line [32](https://github.com/0xScratch/SolanaBootcamp/blob/8898b89b3910527b9973a59c9708947edd2c5437/examples_baremetal/example4-cpi/rust/src/entrypoint.rs#L32) using [invoke](https://docs.rs/solana-program/latest/src/solana_program/program.rs.html#26-28) function.

4. Just for your information, `new_with_bincode` is a method that creates a new instruction using Bincode serialization for the instruction data and takes 3 parameters i.e. program_id, accounts and data. More about this method can be found [here](https://docs.rs/solana-program/latest/solana_program/instruction/struct.Instruction.html#method.new_with_bincode)

5. Even client side is almost the same. The only 2 lines which you can be intrigued about is line [77](https://github.com/0xScratch/SolanaBootcamp/blob/8898b89b3910527b9973a59c9708947edd2c5437/examples_baremetal/example4-cpi/client/main.ts#L77) where we are getting the key for `helloworld` program. And on line [83](https://github.com/0xScratch/SolanaBootcamp/blob/8898b89b3910527b9973a59c9708947edd2c5437/examples_baremetal/example4-cpi/client/main.ts#L83) we created the desired instruction to be passed to the program.

6. And that's it. Just deploy the program, call it and you will see the output.

**Note:** Make sure to deploy the `example1-helloworld` program first before deploying this program. Otherwise, you will get an error.

## Compute Example

This program was all about computing some prime numbers. The main code lets the user type a number and calculate each prime number upto that count. For example, let's say I entered 20, it will give me first 20 prime numbers. That's it.

Not much to explain here as two helper functions do most of the jobs. One is [is_prime](https://github.com/0xScratch/SolanaBootcamp/blob/64db77be11eb6c5139f1f326fa0eb90076627883/examples_baremetal/example5-compute/rust/src/entrypoint.rs#L30) which checks if the number is prime or not. And the other one is [division_based](https://github.com/0xScratch/SolanaBootcamp/blob/64db77be11eb6c5139f1f326fa0eb90076627883/examples_baremetal/example5-compute/rust/src/entrypoint.rs#L41) which calculates the prime numbers.

There's this interesting code snippet at line 17:

```rust
let (prime_count, _) = instruction_data
    .split_first()
    .ok_or(ProgramError::InvalidArgument)?;
```

This line extracts out the first element from the instruction given (which was basically the number entered by the user) and storing it in `prime_count` variable. This is a very handy way to extract out the data from the instruction.

Then this get feeded into the helpers functions, and the output is returned back to the client.
