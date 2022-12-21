#![allow(unused_imports)]
use std::{
    collections::{BTreeMap, HashSet},
    ops::Add,
};

use itertools::Itertools;
use shared::puzzle_input;

fn dst_idx(src_idx: i64, value: i64, len: i64) -> usize {
    (src_idx + value).rem_euclid(len - 1) as usize
}

fn mix(original: &[i64], times: u8) -> Vec<i64> {
    let mut idx_to_orig = (0..original.len()).collect_vec();
    let mut orig_to_idx = idx_to_orig.clone();
    for _ in 0..times {
        for (orig_idx, value) in original.iter().enumerate() {
            let src_idx = orig_to_idx[orig_idx];
            let dst_idx = dst_idx(src_idx as i64, *value, original.len() as i64);
            match src_idx.cmp(&dst_idx) {
                std::cmp::Ordering::Less => {
                    for i in src_idx + 1..=dst_idx {
                        let orig2 = idx_to_orig[i];
                        orig_to_idx[orig2] = i - 1;
                        idx_to_orig[i - 1] = orig2;
                    }
                    orig_to_idx[orig_idx] = dst_idx;
                    idx_to_orig[dst_idx] = orig_idx;
                }
                std::cmp::Ordering::Equal => (),
                std::cmp::Ordering::Greater => {
                    for i in (dst_idx..src_idx).rev() {
                        let orig2 = idx_to_orig[i];
                        orig_to_idx[orig2] = i + 1;
                        idx_to_orig[i + 1] = orig2;
                    }
                    orig_to_idx[orig_idx] = dst_idx;
                    idx_to_orig[dst_idx] = orig_idx;
                }
            }
        }
    }
    idx_to_orig.into_iter().map(|i| original[i]).collect_vec()
}

pub fn main() {
    let input = puzzle_input!();
    let original = input
        .split('\n')
        .map(|line| line.parse::<i64>().unwrap())
        .collect_vec();
    let p1: i64 = {
        let new_order = mix(&original, 1);
        let zero_idx = new_order.iter().find_position(|x| **x == 0).unwrap().0;
        [1000, 2000, 3000]
            .iter()
            .map(|i| new_order[(i + zero_idx) % original.len()])
            .sum()
    };
    let p2: i64 = {
        let decrypted = original
            .iter()
            .map(|x| (*x as i64) * 811589153i64)
            .collect_vec();
        let new_order = mix(&decrypted, 10);
        let zero_idx = new_order.iter().find_position(|x| **x == 0).unwrap().0;
        [1000, 2000, 3000]
            .iter()
            .map(|i| new_order[(i + zero_idx) % original.len()] as i64)
            .sum()
    };

    // 9866, 12374299815791
    println!("part 1: {}\npart 2: {}", p1, p2);
}
