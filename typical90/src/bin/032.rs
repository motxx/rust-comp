#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use proconio::{
    fastout, input,
    marker::{Chars, Usize1},
};

const MOD: usize = 1e9 as usize + 7;
// const MOD: usize = 998244353;
// const MOD: usize = 2147483647;
const INF: i64 = 1e12 as i64;

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
            a: [[i64; n]; n],
            m: usize,
            xys: [(usize, usize); m],
        }
        let mut mp: HashMap<usize, HashSet<usize>> = HashMap::new();
        for (x, y) in xys {
            mp.entry(x - 1).or_default().insert(y - 1);
            mp.entry(y - 1).or_default().insert(x - 1);
        }

        let mut dp = vec![vec![vec![INF; n]; n]; 1 << n];
        for i in 0..n {
            dp[1 << i][0][i] = a[i][0];
//            println!("{}", dp[1 << i][0][i]);
        }
        for bit in 0..(1 << n) {
            for i in 1..n {
                for j in 0..n {
                    for k in 0..n {
                        if  j != k &&
                            dp[bit][i - 1][j] < INF &&
                            bit & (1 << k) == 0 &&
                            mp.get(&j).map_or(true, |st| !st.contains(&k))
                        {
                            dp[bit | (1 << k)][i][k] = min!(dp[bit | (1 << k)][i][k], dp[bit][i - 1][j] + a[k][i]);
//                            println!("{} {} {} {} {}", bit | (1 << k), i, k, a[k][i], dp[bit | (1 << k)][i][k]);
                        }
                    }
                }
            }
        }
        let mut ans = INF;
        for i in 0..n {
            ans = min!(ans, dp[(1 << n) - 1][n - 1][i]);
        }
        println!("{}", if ans < INF { ans } else { -1 });
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
