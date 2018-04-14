# Daily Coding Problem: Problem #1

[![Build Status](https://travis-ci.org/DCP-solved-with-Rust/dcp_00001.svg?branch=master)](https://travis-ci.org/DCP-solved-with-Rust/dcp_00001?branch=master)

> Good morning. Here's your coding interview problem for today.
>
> Given a list of numbers, return whether any two sums to k.
>
> For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.
>
> Bonus: Can you do this in one pass?

## Solution

Solved with Rust 1.27.0 nightly-2018-04-07. https://rustup.rs/

After solving this I looked at how others had done it and I saw that
https://github.com/GideonShils/Daily-Coding-Problem/blob/57072b0adfec692dcda0f9aaf9ec74d0d6625226/Day1.java
was a better solution than my own original solution so I then adapted
my solution accordingly.

If you are interested in having a look at my original solution, see
[main.rs at revision 22f10c3](https://github.com/DCP-solved-with-Rust/dcp_00001/blob/22f10c3b1e23a01563686f41b9c04f667e2196be/src/main.rs).

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

Exits with non-zero if any errors occur.

Exits with zero if any two of the numbers sum to k. Exits with one if not.
