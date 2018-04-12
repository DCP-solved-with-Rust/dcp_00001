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

use std::process::exit;

fn main ()
{
    let args: Vec<String> = env::args().collect();

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

    let stdin = io::stdin();

    let mut nums = Vec::new();

    let mut a = 0;
    let mut b = 0;
    let mut summed = false;

    'outer: for line in stdin.lock().lines()
    {
        let curr_inputs: Vec<i32> = line.unwrap().split(" ")
            .map(|x| x.parse().expect("Not an integer!"))
            .collect();

        for j in curr_inputs
        {
            if nums.iter().any(|x| *x == j) && 2 * j == k
            {
                a = j;
                b = j;
                summed = true;
                break 'outer;
            }
            else
            {
                for i in nums.iter()
                {
                    if i + j == k
                    {
                        a = *i;
                        b = j;
                        summed = true;
                        break 'outer;
                    }
                }

                nums.push(j);
            }
        }
    }

    if summed
    {
        println!("Yes! {} + {} = {}", a, b, k);
    }
    else
    {
        println!("Nope!");
        exit(1);
    }
}