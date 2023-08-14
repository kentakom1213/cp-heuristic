#![allow(non_snake_case)]

#[cfg_attr(cargo_equip, cargo_equip::equip)]
use ::subs::{get_input, judge::Judge, temperature_field::TemperatureField};

/// # Solver
/// ソルバー
#[derive(Debug)]
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
        // 3x3ガウシアンフィルタで平滑化
        self.temperature.smoothing5x5();
    }

    /// i番目の出口の座標を特定する
    /// - 出口を中心とした3x5のパターンを取得する
    fn get_3x5_pattern(&self, i: usize) -> usize {
        let pattern = vec![
            vec![
                Judge::measure(i, -2, -1),
                Judge::measure(i, -1, -1),
                Judge::measure(i, 0, -1),
                Judge::measure(i, 1, -1),
                Judge::measure(i, 2, -1),
            ],
            vec![
                Judge::measure(i, -2, 0),
                Judge::measure(i, -1, 0),
                Judge::measure(i, 0, 0),
                Judge::measure(i, 1, 0),
                Judge::measure(i, 2, 0),
            ],
            vec![
                Judge::measure(i, -2, 1),
                Judge::measure(i, -1, 1),
                Judge::measure(i, 0, 1),
                Judge::measure(i, 1, 1),
                Judge::measure(i, 2, 1),
            ],
        ];
        // 最も近いパターンを特定
        self.temperature.find_nearest_pattern(&pattern)
    }

    /// 解の予想を行う
    fn predict(&mut self) -> Vec<usize> {
        // 予測結果を保存する配列
        (0..self.N).map(|i| self.get_3x5_pattern(i)).collect()
    }
}

fn main() {
    // ソルバの作成
    let mut solver = Solver::new();

    // 解の算出
    solver.solve();
}
