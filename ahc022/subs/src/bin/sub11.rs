#![allow(non_snake_case)]

#[cfg_attr(cargo_equip, cargo_equip::equip)]
use ::subs::{get_input, judge::Judge, temperature_field::TemperatureField};

/// パターンの1辺
const PATTERN_LEN: usize = 5;
const HALF: isize = (PATTERN_LEN / 2) as isize;
const FILL: usize = 500;

/// # Solver
/// ソルバー
pub struct Solver {
    L: usize,
    N: usize,
    S: usize,
    temperature: TemperatureField,
}

impl Solver {
    pub fn new() -> Self {
        // 初期値の取得
        let (L, N, S) = get_input!(usize, usize, usize);
        let landing_pos = get_input!(usize, usize; N);
        Self {
            L,
            N,
            S,
            temperature: TemperatureField::new(L, landing_pos),
        }
    }

    /// ソルバーを実行する
    pub fn solve(&mut self) {
        // 温度盤面の作成
        self.create_temperature();
        // 温度の設定
        Judge::set_temperature(&self.temperature.field);
        // 予測
        let estimate = self.predict();
        // 出力
        Judge::answer(estimate);
    }

    /// 温度を作成する
    fn create_temperature(&mut self) {
        // ランダムな盤面を生成
        self.temperature.generate_random_field();

        // maskを生成
        let mut mask = vec![vec![false; self.L]; self.L];
        for &(r, c) in &self.temperature.landing_pos {
            for dr in -HALF..=HALF {
                for dc in -HALF..=HALF {
                    let i = ((r as isize) + dr) as usize % self.L;
                    let j = ((c as isize) + dc) as usize % self.L;
                    mask[i][j] = true;
                }
            }
        }

        // maskされていない部分をfill
        for r in 0..self.L {
            for c in 0..self.L {
                if !mask[r][c] {
                    self.temperature.field[r][c] = FILL;
                }
            }
        }

        if self.S < 100 {
            // 5x5ガウシアンフィルタで平滑化
            self.temperature.smoothing5x5();
        }
    }

    /// 複数回計測し、中央値を取る
    fn measure_acculate(&self, i: usize, r: isize, c: isize) -> usize {
        Judge::measure(i, r, c)
    }

    /// i番目の出口の座標を特定する
    /// - 出口を中心とした5x5のパターンを取得する
    fn get_5x5_pattern(&self, idx: usize) -> usize {
        // パターン
        let mut pattern = vec![vec![0; PATTERN_LEN]; PATTERN_LEN];
        // 計測
        for r in -HALF..=HALF {
            for c in -HALF..=HALF {
                let i = (r + HALF) as usize;
                let j = (c + HALF) as usize;
                pattern[i][j] = self.measure_acculate(idx, r, c)
            }
        }
        // 最も近いパターンを特定
        self.temperature.find_nearest_pattern(&pattern)
    }

    /// 解の予想を行う
    fn predict(&mut self) -> Vec<usize> {
        // 予測結果を保存する配列
        (0..self.N).map(|i| self.get_5x5_pattern(i)).collect()
    }
}

fn main() {
    // ソルバの作成
    let mut solver = Solver::new();

    // 解の算出
    solver.solve();
}
