// attributes
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// imports
use lazy_static::lazy_static;
use rand;
use rand::prelude::*;
use std::collections::BTreeSet;
use std::{thread, time};

// 入力マクロ
macro_rules! get {
    ( $($t:ty), * ) => {{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut iter = line.trim().split_whitespace();
        (
            $( iter.next().unwrap().parse::<$t>().unwrap() ), *
        )
    }};
}

/// `a > b` のとき、`a`を`b`に置き換え、trueを返す
macro_rules! chmin {
    ( $a:expr, $b:expr $(,)* ) => {{
        if $a > $b {
            $a = $b;
            true
        } else {
            false
        }
    }};
}

// 定数
const INF: usize = 1 << 60;
const SEED: u64 = 20021213;

// 実行時定数
lazy_static! {
    static ref _INPUT: (usize, usize, usize, usize) = get!(usize, usize, usize, usize);
    static ref N: usize = _INPUT.0;
    static ref W: usize = _INPUT.1;
    static ref K: usize = _INPUT.2;
    static ref C: usize = _INPUT.3;
    // powerの設定
    static ref P_INIT: usize = match *C {
        1 => 20,
        2 => 25,
        4 => 30,
        8 => 40,
        16 => 50,
        32 => 64,
        64 => 128,
        128 => 256,
        _ => unreachable!(),
    };
    // 階差
    static ref P_DELTA: usize = match *C {
        1 => 0,
        2 => 0,
        4 => 2,
        8 => 10,
        16 => 10,
        32 => 32,
        64 => 50,
        128 => 64,
        _ => unreachable!(),
    };
}

// abs_diffの実装
trait UsizeTools {
    fn abs_diff(&self, other: Self) -> usize;
}
impl UsizeTools for usize {
    fn abs_diff(&self, other: Self) -> usize {
        if *self < other {
            other - *self
        } else {
            *self - other
        }
    }
}

// Structs
type Pos = (usize, usize);

trait Vec2 {
    fn dist(&self, other: &Self) -> usize;
}
impl Vec2 for Pos {
    /// マンハッタン距離を求める
    fn dist(&self, other: &Self) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

/// セルの状態
#[derive(Debug, Clone, Copy)]
enum State {
    Rock(usize),      // 岩盤(与えたダメージの累計)
    Destroyed(usize), // 破壊された岩盤(与えたダメージの累計)
}

struct Field {
    state: Vec<Vec<State>>,
    sources: BTreeSet<Pos>,
    houses: BTreeSet<Pos>,
}

impl Field {
    fn new(SOURCES: &Vec<Pos>, HOUSES: &Vec<Pos>) -> Self {
        Self {
            state: vec![vec![State::Rock(0); *N]; *N],
            sources: SOURCES.iter().cloned().collect(),
            houses: HOUSES.iter().cloned().collect(),
        }
    }

    /// ## damage
    /// `(r, c)`にダメージ`x`を与える。
    /// ### 出力
    /// - 対象が`Rock`である場合、`Ok`
    ///   - 破壊できた場合、`Ok(true)`
    ///   - 破壊できなかった場合、`Ok(false)`
    /// - 対象が`Rock`以外の場合、`Err`
    fn damage(&mut self, pos: Pos, x: usize) -> bool {
        let (r, c) = pos;
        match self.state[r][c] {
            State::Rock(d) => {
                // 掘削の指示
                println!("{} {} {}", r, c, x);
                // 結果の受け取り
                match get!(isize) {
                    0 => {
                        self.state[r][c] = State::Rock(d + x); // ダメージの追加
                        false
                    }
                    1 => {
                        self.state[r][c] = State::Destroyed(d + x);
                        true
                    }
                    _ => {
                        std::process::exit(0);
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    /// ## destroy
    /// `(r, c)`を破壊するまで掘り進める
    /// ### 出力
    /// - 破壊するまでにかかったコスト
    fn destroy(&mut self, pos: Pos) -> usize {
        let (r, c) = pos;
        let mut POWER = *P_INIT;
        loop {
            match self.state[r][c] {
                State::Rock(_) => {
                    self.damage(pos, POWER);
                    POWER += *P_DELTA;
                }
                State::Destroyed(strength) => {
                    self.sources.insert(pos);
                    break strength;
                }
            }
        }
    }

    /// ## predict_dist
    /// 2点間の、岩盤の強さを考慮した到達コストを予測する。
    /// ↓下の3点をサンプリングし破壊するまでのコストの和を、距離に掛けた値を返す
    /// ```not-rust
    ///     [sc]        [ec]
    /// [sr]  s-----------+
    ///       |           |
    ///       x           |
    ///       |           |
    /// [er]  x-----x-----g
    /// ```
    fn predict_dist(&mut self, start: Pos, end: Pos) -> usize {
        let (sr, sc) = start;
        let (er, ec) = end;
        // サンプリングする地点
        let samples = [
            ((sr + er) / 2, ec),
            (er, sc),
            (er, (sc + ec) / 2)
        ];
        let weight_sum = samples
            .iter()
            .map(|&pos| {
                self.destroy(pos)
            })
            .sum::<usize>();
        let dist = start.dist(&end);

        weight_sum * dist
    }

    /// ## tunnel_h
    /// 水平に掘り進める
    fn tunnel_h(&mut self, r: usize, cstart: usize, cend: usize) {
        let mut c = cstart;
        // 横に掘削
        loop {
            self.destroy((r, c));
            if c == cend {
                break;
            } else if cend < c {
                c -= 1;
            } else {
                c += 1;
            }
        }
    }

    /// ## tunnel_v
    /// 鉛直に掘り進める
    fn tunnel_v(&mut self, c: usize, rstart: usize, rend: usize) {
        let mut r = rstart;
        // 縦に掘削
        loop {
            self.destroy((r, c));
            if r == rend {
                break;
            } else if rend < r {
                r -= 1;
            } else {
                r += 1;
            }
        }
    }

    /// ## tunnel
    /// `start=(sr,sc)`から`end=(er,ec)`まで鉛直→水平の順で掘り進める
    fn tunnel(&mut self, start: Pos, end: Pos) {
        let (sr, sc) = start;
        let (er, ec) = end;
        // 鉛直に掘り進める
        self.tunnel_v(sc, sr, er);
        // 水平に掘り進める
        self.tunnel_h(er, sc, ec);
    }

    /// ## find_source
    /// `(r, c)`から最も近い水源を見つける
    /// ### 出力
    /// - 最も近い水源の座標
    fn find_source(&self, pos: Pos) -> Pos {
        let (mut res, mut dist) = (pos, INF);
        for (i, src) in self.sources.iter().enumerate() {
            if chmin!(dist, pos.dist(src)) {
                res = *src;
            }
        }
        res
    }

    /// ## get_nearest_pair
    /// 最も近い水源と家のペアを取得する
    /// その際、家はリストから削除する
    /// ### 出力
    /// - `(source, house)`
    fn get_nearest_pair(&mut self) -> Option<(Pos, Pos)> {
        if self.houses.is_empty() {
            return None;
        }

        let mut res = (
            *self.sources.iter().next().unwrap(),
            *self.houses.iter().next().unwrap(),
        );
        let mut min_dist = INF;
        let houses = self.houses.clone();
        let sources = self.sources.clone();

        for &house in &houses {
            for &src in &sources {
                if chmin!(min_dist, self.predict_dist(house, src)) {
                    res = (src, house);
                }
            }
        }
        // 見つけたペアに含まれる家を削除
        self.houses.remove(&res.1);

        Some(res)
    }
}

// solve
fn main() {
    // 乱数生成器の初期化
    let mut rng: rand::rngs::StdRng = rand::SeedableRng::seed_from_u64(SEED);

    // 入力受け取り
    lazy_static::initialize(&_INPUT); // N, W, K, C の受け取り
    let SOURCES: Vec<(usize, usize)> = (0..*W).map(|_| get!(usize, usize)).collect();
    let HOUSES: Vec<(usize, usize)> = (0..*K).map(|_| get!(usize, usize)).collect();

    let mut field = Field::new(&SOURCES, &HOUSES);

    // 全ての家に関して、最も近い水源までを直線距離で掘る
    while let Some((src, house)) = field.get_nearest_pair() {
        // 水源から家まで掘り進める
        field.tunnel(src, house);
    }
}
