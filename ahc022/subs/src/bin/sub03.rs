// sample-code

#![allow(non_snake_case)]

#[cfg_attr(cargo_equip, cargo_equip::equip)]
use ::subs::{
    consts::*, get_input, judge::Judge,
};

/// # Solver
/// ソルバー
#[derive(Debug)]
struct Solver {
    L: usize,
    N: usize,
    S: usize,
    landing_pos: Vec<Pos>,
    temperature: Vec<Vec<usize>>,
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
            landing_pos,
            temperature: vec![vec![0; L]; L],
        }
    }

    /// ソルバーを実行する
    pub fn solve(&mut self) {
        // 温度盤面の作成
        self.create_temperature();
        // 温度の設定
        Judge::set_temperature(&self.temperature);
        // 予測
        let estimate = self.predict();
        // 出力
        Judge::answer(estimate);
    }

    /// 温度を作成する
    fn create_temperature(&mut self) {
        // set the temperature to i * 10 for i-th position
        for (i, &(r, c)) in self.landing_pos.iter().enumerate() {
            self.temperature[r][c] = i * 10;
        }
    }

    /// 解の予想を行う
    fn predict(&mut self) -> Vec<usize> {
        // 予測結果を保存する配列
        let mut estimate = vec![INF; self.N];

        for i_in in 0..self.N {
            println!("# measure i={} r=0 c=0", i_in);
            // 温度の計測（3回計測したものの中央値）
            let mut measured_values = [
                Judge::measure(i_in, 0, 0),
                Judge::measure(i_in, 0, 0),
                Judge::measure(i_in, 0, 0),
            ];
            measured_values.sort();
            // 中央値
            let mid = measured_values[1];

            // 最も近い温度のセルを答える
            let min_diff = (0..self.N)
                .min_by_key(|&i_out| {
                    let (r, c) = self.landing_pos[i_out];
                    self.temperature[r][c].abs_diff(mid)
                })
                .unwrap();
            estimate[i_in] = min_diff;
        }

        estimate
    }
}

fn main() {
    // ソルバの作成
    let mut solver = Solver::new();
    // ソルバのプレビュー
    // println!("# {:?}", solver);
    // 解の算出
    solver.solve();
}
