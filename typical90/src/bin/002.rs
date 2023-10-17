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
    fn rec(n: usize, depth: usize, curr: &mut String, result: &mut Vec<String>) { 
        if n < depth {
            return;
        }
        if n == 0 {
            if depth == 0 {
                result.push(curr.clone());
            }
            return;
        }
        curr.push('(');
        Self::rec(n - 1, depth + 1, curr, result);
        curr.pop();
        if depth > 0 {
            curr.push(')');
            Self::rec(n - 1, depth - 1, curr, result);
            curr.pop();
        }
    }

    #[fastout]
    fn solve(&mut self) {
        // let mut stdin = LineSource::new(BufReader::new(io::stdin()));
        // macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            n: usize,
        }
        let mut result: Vec<String> = Vec::new();
        let mut curr = String::new();
        Self::rec(n, 0, &mut curr, &mut result);
        if !result.is_empty() {
            println!("{}", result.join("\n"));
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
