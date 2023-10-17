#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use num_traits::NumOps;
use pathfinding::prelude::directions::N;
use std::{collections::{BTreeMap, BTreeSet, VecDeque}, fmt::Display, str::FromStr};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const MOD: usize = 1e9 as usize + 7;
// const MOD: usize = 998244353;
// const MOD: usize = 2147483647;

#[macro_export]
macro_rules! max {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::max($x, max!($( $y ),+))
    }
}
#[macro_export]
macro_rules! min {
    ($x: expr) => ($x);
    ($x: expr, $( $y: expr ),+) => {
        std::cmp::min($x, min!($( $y ),+))
    }
}
#[derive(Default)]
struct Solver {}
impl Solver {
    #[fastout]
    fn solve(&mut self) {
        // let mut stdin = LineSource::new(BufReader::new(io::stdin()));
        // macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            n: u128,
            k: u64,
        }
        let mut n = n;
        for _ in 0..k {
            let mut m = 0;
            let mut i = 0;
            // 8進数として読み取る
            while n > 0 {
                m += (n % 10) * 8u128.pow(i);
                i += 1;
                n /= 10;
            }
            n = 0;
            i = 0;
            // 9進数として書き出す
            // ただし、8を5に置き換える
            while m > 0 {
                if m % 9 == 8 {
                    n += 5 * 10u128.pow(i);
                } else {
                    n += (m % 9) * 10u128.pow(i);
                }
                i += 1;
                m /= 9;
            }
        }
        println!("{}", n);
    }
}

fn main() {
    std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(|| Solver::default().solve())
        .unwrap()
        .join()
        .unwrap();
}
