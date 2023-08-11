// attributes
#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]

// imports
use lazy_static::lazy_static;

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

// 定数
const INF: usize = 1 << 60;
const POWER: usize = 50;

// 実行時定数
lazy_static! {
    static ref _INPUT: (usize, usize, usize, usize) = { get!(usize, usize, usize, usize) };
    static ref N: usize = _INPUT.0;
    static ref W: usize = _INPUT.1;
    static ref K: usize = _INPUT.2;
    static ref C: usize = _INPUT.3;
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
    fn sub(&self, other: &Self) -> Self;
    fn dist(&self, other: &Self) -> usize;
}

impl Vec2 for Pos {
    fn sub(&self, other: &Self) -> Self {
        (self.0 - other.0, self.1 - other.1)
    }
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
}

impl Field {
    fn new(sources: &Vec<Pos>, houses: &Vec<Pos>) -> Self {
        Self {
            state: vec![vec![State::Rock(0); *N]; *N],
        }
    }

    /// ## damage
    /// `(r, c)`にダメージ`x`を与える。
    /// ### 出力
    /// - 対象が`Rock`である場合、`Ok`
    ///   - 破壊できた場合、`Ok(true)`
    ///   - 破壊できなかった場合、`Ok(false)`
    /// - 対象が`Rock`以外の場合、`Err`
    fn damage(&mut self, r: usize, c: usize, x: usize) -> bool {
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

    fn show(&self) {
        for r in 0..*N {
            for c in 0..*N {
                match self.state[r][c] {
                    State::Rock(_) => eprint!("*"),
                    State::Destroyed(_) => eprint!("."),
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
    let sources: Vec<(usize, usize)> = (0..*W).map(|_| get!(usize, usize)).collect();
    let houses: Vec<(usize, usize)> = (0..*K).map(|_| get!(usize, usize)).collect();

    let mut field = Field::new(&sources, &houses);

    // 全ての家に関して、最も近い水源までを直線距離で掘る
    for &house in &houses {
        // 最も近い水源を調べる
        let mut tmp = (0, INF);
        for (i, source) in sources.iter().enumerate() {
            if tmp.1 > house.dist(source) {
                tmp = (i, house.dist(source));
            }
        }
        let src = sources[tmp.0];

        let (house_r, house_c) = house; // 家の位置
        let (mut cur_r, mut cur_c) = src; // 現在位置

        // 縦に掘削
        loop {
            let can_destroy = match field.state[cur_r][cur_c] {
                State::Rock(_) => field.damage(cur_r, cur_c, POWER),
                _ => true,
            };
            if can_destroy {
                if cur_r == house_r {
                    break;
                } else if house_r < cur_r {
                    cur_r -= 1;
                } else {
                    cur_r += 1;
                }
            }
        }

        // 横に掘削
        loop {
            let can_destroy = match field.state[cur_r][cur_c] {
                State::Rock(_) => field.damage(cur_r, cur_c, POWER),
                _ => true,
            };
            if can_destroy {
                if house_c == cur_c {
                    break;
                } else if house_c < cur_c {
                    cur_c -= 1;
                } else {
                    cur_c += 1;
                }
            }
        }
    }
}
