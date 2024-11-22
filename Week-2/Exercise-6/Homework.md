# Exercise 6

Here we gotta complete the `homework 6` section.

All complete solutions are present in my [forked-repo](https://github.com/0xScratch/SolanaBootcamp/tree/main/homeworks_rust/homeworks/homework6)

## Coding Challenge

Given a vector `nums`, create a new vector called `runningSum` where each element at index i is the sum of all elements from the beginning of the vector up to index i.

Return the running sum of `nums`.

### Example 1

```typescript
Input: nums = [1, 2, 3, 4]
Output: [1, 3, 6, 10]
Explanation: The running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4]
```

### Example 2

```typescript
Input: nums = [1, 1, 1, 1, 1]
Output: [1, 2, 3, 4, 5]
Explanation: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1]
```

### Example 3

```typescript
Input: nums = [3, 1, 2, 10, 1]
Output: [3, 4, 6, 16, 17]
```

See if you can figure out a "Rusty" way to do it using Rust's iterators and methods.

Here's the [Solution](../Exercise-6/running-sum/src/main.rs)
