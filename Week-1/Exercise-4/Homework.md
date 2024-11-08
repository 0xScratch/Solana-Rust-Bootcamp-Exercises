# Rust Programs

## Fizz Buzz Program

1. Create a program called **bootcamp** using cargo.
2. The main function should print a welcome message.
3. Write a 'fizz buzz' function that should be called from your main function.
    - The function should have a loop counting up to 301
    - If the count is divisible by 3, print "fizz"
    - If the count is divisible by 5, print "buzz"
    - If the count is divisible by 3 and 5, print "fizz buzz"
    - At the end, print the number of time "fizz buzz" occurred.

## Two Sum

We have Vectors of integers called nums and a target integer. Return the two indices that add up to the target value.

Rules:

    - There's always one unique solution for each list.
    - You can't use the same number twice.

### Example 1

    ```bash
        Input: nums = [2, 7, 11, 15], target = 9
        Output: [0, 1]
        Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
    ```

### Example 2

    ```bash
        Input: nums = [3, 2, 4], target = 6
        Output: [1, 2]
    ```

### Example 3

    ```bash
        Input: nums = [3, 3], target = 6
        Output: [0, 1]
    ```

### Use this boilerplate code

    ```rust
        fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            // your code goes here
        }

        fn main() {
            println!("{:?}", two_sum(vec![2, 3, 4, 5,], 9));
        }
    ```

There's a brute force solution that's a bit easier to figure out, but see if you can also use a HashMap for a more efficient solution.

## Solutions

1. [Fizz Buzz Program](./bootcamp/src/main.rs)
2. [Two Sum](./two-sum/src/main.rs)
