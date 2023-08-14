#![allow(non_snake_case)]

use rand::prelude::*;

use crate::{chmin, consts::*, gaussian_filter::GaussianFilter, usize_tools::UsizeTools};

/// # TemperatureField
/// 温度のフィールド
#[derive(Debug)]
pub struct TemperatureField {
    L: usize, // 1辺
    landing_pos: Vec<Pos>,
    pub field: Vec<Vec<usize>>, // フィールド
}

impl TemperatureField {
    pub fn new(L: usize, landing_pos: Vec<Pos>) -> Self {
        Self {
            L,
            landing_pos,
            field: vec![vec![0; L]; L],
        }
    }

    /// 0~1000までの値で埋められたランダムなフィールドを生成する
    pub fn generate_random_field(&mut self) {
        // 乱数生成器の初期化
        let mut rng: SmallRng = rand::SeedableRng::seed_from_u64(20021213);
        // ランダムな値で埋める
        for r in 0..self.L {
            for c in 0..self.L {
                self.field[r][c] = rng.gen_range(0, 1000);
            }
        }
    }

    /// フィールドを3x3のガウシアンフィルタで平滑化する
    pub fn smoothing3x3(&mut self) {
        self.field = GaussianFilter::smoothing3x3(&self.field);
    }

    /// フィールドを3x3のガウシアンフィルタで平滑化する
    pub fn smoothing5x5(&mut self) {
        self.field = GaussianFilter::smoothing5x5(&self.field);
    }

    /// 最も近いパターンを持つセルを見つける
    pub fn find_nearest_pattern(&self, pattern: &Vec<Vec<usize>>) -> usize {
        // パターンのサイズ
        let (row, col) = (pattern.len(), pattern[0].len());
        // 最もパターンが近いセル
        let mut nearest = 0;
        let mut score = std::usize::MAX;
        // セルを全探索
        for (idx, &(r, c)) in self.landing_pos.iter().enumerate() {
            let mut tmp = 0; // 一致率
            for i in 0..row {
                for j in 0..col {
                    // マップする位置
                    let new_r = (self.L - row / 2 + r + i) % self.L;
                    let new_c = (self.L - col / 2 + c + j) % self.L;
                    tmp += self.field[new_r][new_c].sq_err(pattern[i][j]);
                }
            }
            // 更新
            if chmin!(score, tmp) {
                nearest = idx;
            }
        }
        nearest
    }

    /// 最も近いパターンを持つセルを見つける
    pub fn find_nearest_pattern_flexible(&self, pattern: &Vec<Vec<Option<usize>>>) -> usize {
        // パターンのサイズ
        let (row, col) = (pattern.len(), pattern[0].len());
        // 最もパターンが近いセル
        let mut nearest = 0;
        let mut score = std::usize::MAX;
        // セルを全探索
        for (idx, &(r, c)) in self.landing_pos.iter().enumerate() {
            let mut tmp = 0; // 一致率
            for i in 0..row {
                for j in 0..col {
                    if let Some(val) = pattern[i][j] {
                        // マップする位置
                        let new_r = (self.L - row / 2 + r + i) % self.L;
                        let new_c = (self.L - col / 2 + c + j) % self.L;
                        tmp += self.field[new_r][new_c].sq_err(val);
                    }
                }
            }
            // 更新
            if chmin!(score, tmp) {
                nearest = idx;
            }
        }
        nearest
    }
}

#[cfg(test)]
mod test_temperature_field {
    use super::*;

    #[test]
    fn test_nearest_pattern() {
        // 検索対象のフィールド
        let field = TemperatureField {
            L: 5,
            landing_pos: vec![(1, 1), (2, 1), (2, 2), (3, 4), (4, 4)],
            field: vec![
                vec![0, 3, 2, 1, 3],
                vec![6, 4, 8, 9, 0],
                vec![3, 3, 4, 2, 1],
                vec![6, 7, 8, 8, 0],
                vec![1, 2, 7, 6, 0],
            ],
        };

        assert_eq!(
            field.find_nearest_pattern(&vec![vec![0, 3, 2], vec![6, 4, 8], vec![3, 3, 4]]),
            0 // (1, 1)
        );

        assert_eq!(
            field.find_nearest_pattern(&vec![vec![8, 0, 6], vec![6, 0, 1], vec![1, 3, 0]]),
            4 // (4, 4)
        );

        assert_eq!(
            field.find_nearest_pattern(&vec![vec![4, 7, 8], vec![3, 3, 3], vec![8, 8, 8]]),
            2 // (2, 2)
        );

        assert_eq!(
            field.find_nearest_pattern(&vec![vec![3, 3, 3]]),
            1 // (2, 1)
        );

        assert_eq!(
            field.find_nearest_pattern(&vec![vec![1], vec![0], vec![0]]),
            3 // (3, 4)
        );
    }
}
