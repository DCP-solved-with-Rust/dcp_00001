/*
 * Copyright (c) 2018 Erik Nordstrøm <erik@nordstroem.no>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

use std::env;

use std::vec::Vec;

use std::io;
use std::io::prelude::*;

use std::collections::HashSet;

use std::process::exit;

fn main ()
{
    let args: Vec<String> = env::args().collect();

    /*
     * We take a single, mandatory argument; k.
     * This is the same k given in the problem
     * that we are looking to sum to.
     */

    let k =
    {
        if args.len() == 2
        {
            args[1].parse::<i32>().unwrap()
        }
        else
        {
            panic!(format!("Usage: {:?} <k>", args[0]));
        }
    };

    /*
     * We read from stdin the list of numbers that
     * we are going to look through for any two numbers
     * that will sum to k.
     */

    let stdin = io::stdin();

    let mut nums = Vec::new();

    for line in stdin.lock().lines()
    {
        let mut curr_inputs: Vec<i32> = line.unwrap().split(" ")
            .map(|x| x.parse().expect("Inputs must be integers!"))
            .collect();

        nums.append(&mut curr_inputs);
    }

    exit((!any_two_in_list_sum_to_k(nums, k)) as i32);
}

fn any_two_in_list_sum_to_k (nums: Vec<i32>, k: i32) -> bool
{
    if nums.len() < 2
    {
        panic!("Need at least two numbers!");
    }

    let mut unique_nums_seen = HashSet::new();

    for i in nums
    {
        let j = k - i;

        if unique_nums_seen.contains(&j)
        {
            println!("Yes! {} + {} = {}", j, i, k);

            return true;
        }

        unique_nums_seen.insert(i);
    }

    println!("Nope!");
    return false;
}

#[test]
fn given_example ()
{
    let k = 17;
    let nums = vec![10, 15, 3, 7];
    assert!(any_two_in_list_sum_to_k(nums, k));
}
