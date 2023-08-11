// attributes
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// imports
use lazy_static::lazy_static;
use rand;
use rand::prelude::*;
use rand::{rngs::StdRng, SeedableRng};
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
const SEED: u64 = 20021213; // 乱数のシード値
const SAMPLING_THRESHOLD: usize = 20; // サンプリングを行うときの最小

// 実行時定数
lazy_static! {
    static ref _INPUT: (usize, usize, usize, usize) = get!(usize, usize, usize, usize);
    static ref N: usize = _INPUT.0;
    static ref W: usize = _INPUT.1;
    static ref K: usize = _INPUT.2;
    static ref C: usize = _INPUT.3;
    // powerの設定
    static ref POWER: usize = 50.max(2 * *C);
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
    sources: Vec<Pos>,
}

impl Field {
    fn new(SOURCES: &Vec<Pos>, HOUSES: &Vec<Pos>) -> Self {
        Self {
            state: vec![vec![State::Rock(0); *N]; *N],
            sources: SOURCES.iter().cloned().collect(),
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
                        // self.show();
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
        loop {
            match self.state[r][c] {
                State::Rock(_) => {
                    self.damage(pos, *POWER);
                }
                State::Destroyed(strength) => {
                    break strength;
                }
            }
        }
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
                return;
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

    /// ## sampling
    /// 長方形領域をランダムに試し掘りし、最も小さい位置座標を返す
    /// ### 入力
    /// - `d1`：長方形の対角成分1
    /// - `d2`：長方形の対角成分2
    /// - `rng`：乱数生成器
    /// ### 出力
    /// `Pos`：最も値の低い座標を返す
    fn sampling(&mut self, d1: Pos, d2: Pos, rng: &mut StdRng) -> Pos {
        let (rstart, rend) = (d1.0.min(d2.0), d1.0.max(d2.0));
        let (cstart, cend) = (d1.1.min(d2.1), d1.1.max(d2.1));
        let height = rend - rstart;  // 長方形の高さ
        let width  = cend - cstart;  // 長方形の幅
        let sample_count = (height / SAMPLING_THRESHOLD) * (width / SAMPLING_THRESHOLD);

        // サンプリング
        let (mut min_idx, mut min_cost) = (d1, INF);  // 答えとなる座標、掘るのにかかるコスト
        for _ in 0..sample_count {
            let r = rng.gen_range(rstart, rend);
            let c = rng.gen_range(cstart, cend);
            // 破壊するまでにかかるコストを調べる
            let cost = match self.state[r][c] {
                State::Rock(_) => self.destroy((r, c)),
                State::Destroyed(c) => c,
            };
            if chmin!(min_cost, cost) {
                min_idx = (r, c);
            }
        }
        min_idx
    }

    /// ## tunnel_auto
    /// 再帰的にサンプリングを行いながら、最も重みの小さい最短経路を通るように掘り進める
    fn tunnel_auto(&mut self, start: Pos, end: Pos, rng: &mut StdRng) {
        if start.dist(&end) < SAMPLING_THRESHOLD || self.sampling(start, end, rng) == start {
            // 直接掘り進める
            self.tunnel(start, end);
        }
        else {
            // サンプリングし、経由地を求める
            let waypoint = self.sampling(start, end, rng);

            // 開始地からから経由地まで
            self.tunnel_auto(start, waypoint, rng);

            // 経由地から目的地まで
            self.tunnel_auto(waypoint, end, rng);
        }
    }

    /// ## find_source
    /// `(r, c)`から最も近い水源を見つける
    /// ### 出力
    /// - 最も近い水源の座標
    fn find_source(&self, pos: Pos) -> Pos {
        let (mut idx, mut dist) = (0, INF);
        for (i, src) in self.sources.iter().enumerate() {
            if chmin!(dist, pos.dist(src)) {
                idx = i;
            }
        }
        self.sources[idx]
    }

    fn show(&self) {
        for r in 0..*N {
            for c in 0..*N {
                match self.state[r][c] {
                    State::Rock(_) => eprint!(" "),
                    State::Destroyed(strength) => {
                        match strength {
                            0..=625 => eprint!("▁"),
                            626..=1250 => eprint!("▂"),
                            1251..=1875 => eprint!("▃"),
                            1876..=2500 => eprint!("▄"),
                            2501..=3175 => eprint!("▅"),
                            3126..=3750 => eprint!("▆"),
                            3751..=4375 => eprint!("▇"),
                            4376..=5000 => eprint!("█"),
                            _ => eprint!("█"),
                        };
                    }
                }
            }
            eprintln!();
        }
    }
}

// solve
fn main() {
    // 入力受け取り
    lazy_static::initialize(&_INPUT); // N, W, K, C の受け取り
    let SOURCES: Vec<(usize, usize)> = (0..*W).map(|_| get!(usize, usize)).collect();
    let HOUSES: Vec<(usize, usize)> = (0..*K).map(|_| get!(usize, usize)).collect();

    // 乱数生成器の初期化
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(SEED);

    let mut field = Field::new(&SOURCES, &HOUSES);

    // 全ての家に関して、最も近い水源までを直線距離で掘る
    for &house in &HOUSES {
        // 最も近い水源を特定する
        let src = field.find_source(house);

        // 自動で掘り進める
        field.tunnel_auto(src, house, &mut rng);
    }
}
