// sample-code

#![allow(non_snake_case)]

use subs::get_input;
#[cfg_attr(cargo_equip, cargo_equip::equip)]
use ::subs::{consts::*, judge::Judge};

/// # ガウシアンフィルター
pub struct GaussianFilter {
    row: usize,
    col: usize,
    sum: usize,              // フィルタの数の合計
    filter: Vec<Vec<usize>>, // フィルタ
}

impl GaussianFilter {
    pub fn new(filter: Vec<Vec<usize>>) -> Self {
        Self {
            row: filter.len(),
            col: filter[0].len(),
            sum: filter.iter().map(|v| v.iter().sum::<usize>()).sum(),
            filter,
        }
    }

    /// 畳み込みを行う（対象の二次元配列はトーラスとみなす）
    pub fn convolution(&self, target: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        // 対象となる配列のサイズ
        let (row, col) = (target.len(), target[0].len());
        // 結果を格納する配列
        let mut result = vec![vec![0; col]; row];

        // 畳み込み
        for r in 0..row {
            for c in 0..col {
                for i in 0..self.row {
                    for j in 0..self.col {
                        // マップする位置
                        let new_r = (row - self.row / 2 + r + i) % row;
                        let new_c = (col - self.col / 2 + c + j) % col;
                        result[new_r][new_c] += target[r][c] * self.filter[i][j];
                    }
                }
            }
        }

        // 平均を取る
        for r in 0..row {
            for c in 0..col {
                // 情報が減るのを防ぐため、切り上げする
                result[r][c] += self.sum - 1;
                result[r][c] /= self.sum;
            }
        }

        result
    }
}

/// # Solver
/// ソルバー
#[derive(Debug)]
pub struct Solver {
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
            temperature: vec![vec![10 * (N / 2); L]; L],
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
        // 温度はデフォルトで最小値と最大値の中間に設定してある
        // set the temperature to i * 10 for i-th position
        for (i, &(r, c)) in self.landing_pos.iter().enumerate() {
            self.temperature[r][c] = i * 10;
        }
        // 5x5のガウシアンフィルタをかける
        let filter = GaussianFilter::new(vec![
            // vec![1, 4, 6, 4, 1],
            // vec![4, 16, 24, 16, 4],
            // vec![6, 24, 36, 24, 6],
            // vec![4, 16, 24, 16, 4],
            // vec![1, 4, 6, 4, 1],
            vec![1, 5, 8, 5, 1],
            vec![5, 18, 27, 18, 5],
            vec![8, 27, 0, 27, 8],
            vec![5, 18, 27, 18, 5],
            vec![1, 5, 8, 5, 1],
        ]);
        self.temperature = filter.convolution(&self.temperature);
        // 再度温度を設定
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

    // 解の算出
    solver.solve();
}
