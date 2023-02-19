// sub01

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use itertools::{join, Itertools};
use rand::{SeedableRng, Rng};

// [Rustで競技プログラミング スターターキット](https://qiita.com/hatoo@github/items/fa14ad36a1b568d14f3e)
macro_rules! get {
    ($t:ty) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.trim().parse::<$t>().unwrap()
        }
    };
    ($($t:ty),*) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            (
                $(iter.next().unwrap().parse::<$t>().unwrap(),)*
            )
        }
    };
    ($t:ty ; $n:expr) => {
        (0..$n).map(|_|
            get!($t)
        ).collect::<Vec<_>>()
    };
    ($($t:ty),* ; $n:expr) => {
        (0..$n).map(|_|
            get!($($t),*)
        ).collect::<Vec<_>>()
    };
    ($t:ty ;;) => {
        {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            line.split_whitespace()
                .map(|t| t.parse::<$t>().unwrap())
                .collect::<Vec<_>>()
        }
    };
    ($t:ty ;; $n:expr) => {
        (0..$n).map(|_|
            get!($t ;;)
        ).collect::<Vec<_>>()
    };
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();

#[derive(Debug, Clone, Copy)]
struct Node {
    nxt: usize,
    wgt: usize,
}

/// # 方針
/// - 愚直に前からK本ずつ出力する
fn main() {
    // --- 入力受け取り ---
    let (N, M, D, K) = get!(usize, usize, usize, usize);
    let UVW: Vec<(usize, usize, usize)> = get!(usize, usize, usize; M)
        .iter()
        .map(|&(u, v, w)| (u-1, v-1, w))
        .collect();
    let XY = get!(usize, usize; N);
    // --------------------

    // --- グラフの構築 ---
    let graph = {
        let mut g = vec![vec![]; N];
        for &(u, v, w) in &UVW {
            g[u].push( Node{ nxt: v, wgt: w } );
            g[v].push( Node{ nxt: u, wgt: w } );
        }
        g
    };

    // for i in 0..N {
    //     for node in &graph[i] {
    //         eprintln!("{:?} ", node);
    //     }
    //     println!();
    // }


    for i in 0..M {
        print!("{} ", i/K + 1);
    }
    println!();
}
