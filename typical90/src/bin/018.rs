#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
use alga::general::RealField;
use itertools::Itertools;
use num_integer::Roots;
use std::{collections::{BTreeMap, BTreeSet, VecDeque}, f64::consts::PI};

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
    fn get_yz(e: f64, t: f64, l: f64) -> (f64, f64) {
        let rad = e / t * 2.0 * PI;
        (rad.sin() * -l/2.0, -rad.cos() * l / 2.0 + l / 2.0)
    }

    #[fastout]
    fn solve(&mut self) {
        // let mut stdin = LineSource::new(BufReader::new(io::stdin()));
        // macro_rules! input(($($tt:tt)*) => (proconio::input!(from &mut stdin, $($tt)*)));
        input! {
            t: f64,
            l: f64,
            x: f64,
            y: f64,
            q: usize,
            e: [f64; q],
        }

        for i in 0..q {
            let (ey, ez) = Self::get_yz(e[i], t, l);
            let dy = (ey - y).abs();
            let p2 = x * x + dy * dy;
            let p = p2.sqrt();
            let q = (p2 + ez * ez).sqrt();
            let rad = (p / q).acos();
            let deg = rad * 180.0 / PI;
            println!("{}", deg);
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
