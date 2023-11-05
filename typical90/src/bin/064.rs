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
    #[fastout]
    fn solve(&mut self) {
        // let mut stdin = LineSource::new(BufReader::new(io::stdin()));
        // macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            n: usize,
            q: usize,
            a: [i64; n],
            lrv: [(usize, usize, i64); q],
        }

        let mut b = vec![0i64; n];
        let mut ans = 0i64;
        for i in 0..a.len() {
            if i > 0 {
                b[i] = a[i] - a[i - 1];
                ans += b[i].abs();
            }
        }
        //println!("a: {:?}", a);
        //println!("b: {:?}", b);
        for (l, r, v) in lrv {
            if l > 1 {
                let prev = b[l - 1];
                b[l - 1] += v;
                let curr = b[l - 1];
                ans += curr.abs() - prev.abs();
            }

            if r < n {
                let v = -v;
                let prev = b[r];
                b[r] += v;
                let curr = b[r];
                ans += curr.abs() - prev.abs();
            }

            println!("{}", ans);
        }
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
