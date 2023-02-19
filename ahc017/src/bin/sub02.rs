// sub02

// attributes
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_macros)]

// imports
use itertools::{join, Itertools};
use rand::Rng;
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use rand::{SeedableRng, rngs::SmallRng};

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
}

// constant
const MOD1: usize = 1_000_000_007;
const MOD9: usize = 998_244_353;
const INF: usize = 1001001001001001001;
const NEG1: usize = 1_usize.wrapping_neg();
const SEED: u64 = 20021213;
const REP_TIME: usize = 1_000_000;

/// # Edge
/// グラフのノードを表す
#[derive(Debug, Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
    w: usize,
    day: usize,  // 工事を行う日付
}

/// # Graph
/// グラフ
#[derive(Debug)]
struct Graph {
    N: usize,          // ノードの数
    M: usize,          // エッジの数
    D: usize,          // 工事を行える期間
    K: usize,          // 1日に行える工事の最大値
    edges: Vec<Edge>,  // 辺の集合
    cnt: Vec<usize>,   // cnt[d] := d日に工事している辺の個数
}

impl Graph {
    fn new(n: usize, d: usize, k: usize) -> Self {
        Graph {
            N: n,
            M: 0,
            D: d,
            K: k,
            edges: vec![],
            cnt: vec![0; n],
        }
    }

    /// 辺の追加
    fn append_edge(&mut self, u: usize, v: usize, w: usize, day: usize) {
        self.edges.push(
            Edge { u, v, w, day }
        );
        self.cnt[day] += 1;
        self.M += 1;
    }

    /// 辺を工事する日付を変更
    /// → 成功したかどうか
    fn change_day(&mut self, edge: usize, new_day: usize) -> bool {
        if self.cnt[new_day] + 1 > self.K {
            return false;
        }
        let old_day = self.edges[edge].day;
        self.edges[edge].day = new_day;
        self.cnt[old_day] -= 1;
        self.cnt[new_day] += 1;
        true
    }

    // /// グラフの不満度を計算する
    // /// - ランダムに選んだ日付の、ランダムに選んだ頂点の組で計算
    // /// - O(N^2)
    // fn calc_f(&self) -> usize {
    //     let mut sum = 0;
    //     for i in 0..self.N {
    //         for j in i+1..self.N {

    //         }
    //     }
    //     sum / (self.N * (self.N - 1))
    // }

    /// 各辺を工事する日付を表示する
    fn show(&self) {
        for edge in self.edges.iter() {
            print!("{} ", edge.day);
        }
        println!();
    }
}

/// # 方針
/// - 愚直に前からK本ずつ出力する
fn main() {
    // --- 入力受け取り ---
    let (N, M, D, K) = get!(usize, usize, usize, usize);
    let UVW: Vec<(usize, usize, usize)> = get!(usize, usize, usize; M)
        .iter()
        .map(|&(u, v, w)| (u - 1, v - 1, w))
        .collect();
    let XY = get!(usize, usize; N);
    // --------------------

    // --- グラフの構築 ---
    let mut graph = Graph::new(N, D, K);
    for (i, &(u, v, w)) in UVW.iter().enumerate() {
        let day = i / K + 1;
        graph.append_edge(u, v, w, day);
    }
    // --------------------

    // --- 乱数の初期化 ---
    let mut rng = SmallRng::seed_from_u64(SEED);
    // --------------------

    // -- 最適化 ---
    // ランダムに辺を選び、日付を変更する
    for _ in 0..REP_TIME {
        let edge = rng.gen_range(0, M);
        let new_day = rng.gen_range(1, D+1);

        graph.change_day(edge, new_day);
    }
    // -------------

    // --- 出力 ---
    graph.show();
    // ------------
}
