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
    fn gcd(mut a: u64, mut b: u64) -> u64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    fn tgcd(v: Vec<u64>) -> u64 {
        let mut res = std::u64::MAX;
        let g = Self::gcd(v[0], v[1]);
        if v[2] % g == 0 {
            res = (v[0] / g - 1) + (v[1] / g - 1) + (v[2] / g - 1);
        }
        let g = Self::gcd(v[1], v[2]);
        if v[0] % g == 0 {
            res = std::cmp::min(res, (v[0] / g - 1) + (v[1] / g - 1) + (v[2] / g - 1));
        }
        let g = Self::gcd(v[0], v[2]);
        if v[1] % g == 0 {
          res = std::cmp::min(res, (v[0] / g - 1) + (v[1] / g - 1) + (v[2] / g - 1));
        }
        res
    }

    #[fastout]
    fn solve(&mut self) {
        // let mut stdin = LineSource::new(BufReader::new(io::stdin()));
        // macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            a: u64,
            b: u64,
            c: u64,
        }
        println!("{}", Self::tgcd(vec![a, b, c]))
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
