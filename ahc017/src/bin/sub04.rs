// sub03

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
const REP_TIME: usize = 10000;
const TRY_TIME: usize = 5;

/// # Edge
/// グラフのノードを表す
#[derive(Debug, Clone, Copy, PartialEq)]
struct Edge {
    i: usize,    // 辺の番号
    u: usize,    // 頂点1
    v: usize,    // 頂点2
    w: usize,    // 重み
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
    adjacent: Vec<Vec<Vec<Option<Edge>>>>,  // [日付][u][v] := 辺が存在するか
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
            adjacent: vec![vec![vec![None; n]; n]; d+1],
            cnt: vec![0; n],
        }
    }

    /// 辺の追加
    fn append_edge(&mut self, u: usize, v: usize, w: usize, day: usize) {
        let i = self.M;
        self.edges.push(
            Edge { i, u, v, w, day }
        );
        self.adjacent[day][u][v] = Some(
            Edge { i, u, v, w, day }
        );
        self.cnt[day] += 1;
        self.M += 1;
    }
    
    /// 辺を工事する日付を変更
    /// → 成功したかどうか
    fn change_day(&mut self, i: usize, new_day: usize) -> bool {
        if self.cnt[new_day] + 1 > self.K {
            return false;
        }

        let Edge { i: _, u, v, w, day: old_day } = self.edges[i];

        self.edges[i].day = new_day;
        self.cnt[old_day] -= 1;
        self.cnt[new_day] += 1;

        // adjacent
        self.adjacent[old_day][u][v] = None;
        self.adjacent[old_day][u][v] = None;
        self.adjacent[new_day][u][v] = Some(self.edges[i]);
        self.adjacent[new_day][v][u] = Some(self.edges[i]);

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
    eprintln!("REP_TIME={}", REP_TIME);
    eprintln!("TRY_TIME={}", TRY_TIME);
    
    // --- 入力受け取り ---
    let (N, M, D, K) = get!(usize, usize, usize, usize);
    let UVW: Vec<(usize, usize, usize)> = get!(usize, usize, usize; M)
        .iter()
        .map(|&(u, v, w)| (u - 1, v - 1, w))
        .collect();
    let XY = get!(usize, usize; N);
    // --------------------

    // --- グラフの構築 ---
    let mut D_MAX = 0;
    let mut graph = Graph::new(N, D, K);
    for (i, &(u, v, w)) in UVW.iter().enumerate() {
        let day = i / K + 1;
        D_MAX = day;
        graph.append_edge(u, v, w, day);
    }
    D_MAX += 5;
    if D_MAX > D { D_MAX = D }
    // --------------------

    // --- 乱数の初期化 ---
    let mut rng = SmallRng::seed_from_u64(SEED);
    // --------------------

    // -- 最適化 ---
    // ランダムに頂点を選択、日付が被っている辺に関して辺の日付を変更
    let mut change_count = 0;

    for _ in 0..REP_TIME {
        let e = rng.gen_range(0, M);
        let edge = graph.edges[e];
        let i = edge.u;
        
        let mut cnt_day = vec![vec![]; D+1];
        
        for d in 1..=D {
            for j in 0..N {
                if let Some(edge) = &graph.adjacent[d][i][j] {
                    cnt_day[d].push(
                        edge.i
                    );
                }
            }
        }

        // 重複した日程をばらけさせる
        for old_day in 1..=D {
            if cnt_day[old_day].len() >= 2 {
                let choose_edge = cnt_day[old_day][0];
                // TRY_TIME回繰り返す、その際に変更されない場合は無視
                for _ in 0..TRY_TIME {
                    let new_day = rng.gen_range(1, D_MAX+1);
                    if cnt_day[new_day].len() + 1 <= cnt_day[old_day].len() - 1 && graph.change_day(choose_edge, new_day) {
                        change_count += 1;
                        break;
                    }
                }
            }
        }

        // let mut cnt_day_after = vec![vec![]; D+1];
        // for d in 1..=D {
        //     for j in 0..N {
        //         if let Some(edge) = graph.adjacent[d][i][j] {
        //             cnt_day_after[d].push(
        //                 edge
        //             );
        //         }
        //     }
        // }

        // if cnt_day != cnt_day_after {
        //     change_count += 1;
        //     eprintln!("Before: {:?}", cnt_day);
        //     eprintln!("After : {:?}", cnt_day_after);
        // }
    }

    eprintln!("changed: {}", change_count);
    // -------------

    // --- 出力 ---
    graph.show();
    // ------------
}
