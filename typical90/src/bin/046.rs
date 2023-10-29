#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

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
            n: usize,
            a: [u64; n],
            b: [u64; n],
            c: [u64; n],
        }
        const MOD: usize = 46;
        let mut a_count = [0u64; MOD];
        let mut b_count = [0u64; MOD];
        let mut c_count = [0u64; MOD];
        for i in 0..n {
            a_count[(a[i] % (MOD as u64)) as usize] += 1;
            b_count[(b[i] % (MOD as u64)) as usize] += 1;
            c_count[(c[i] % (MOD as u64)) as usize] += 1;
        }
        let ans: u64 = (0..MOD).flat_map(|i| {
            (0..MOD).map(move |j| {
                let k = (2 * MOD - i - j) % MOD;
                a_count[i] * b_count[j] * c_count[k]
            })
        }).sum();
        println!("{}", ans);
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
