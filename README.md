# Daily Coding Problem: Problem #1

> Good morning. Here's your coding interview problem for today.
>
> Given a list of numbers, return whether any two sums to k.
>
> For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
>
> Bonus: Can you do this in one pass?

## Solution

Solved with Rust 1.27.0 nightly-2018-04-07. https://rustup.rs/

### Usage

`cargo run <k>` and provide the list of numbers on stdin separated by whitespace.

Example:

```
cargo run 17 <<EOF
10 15 3 7
EOF
```

Output:

```
Yes! 10 + 7 = 17
```

Exits with zero if any two of the numbers sum to k. Exits with one if not.
