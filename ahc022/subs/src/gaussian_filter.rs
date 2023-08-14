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
    fn convolution(&self, target: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
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

    /// 3x3のフィルターで平滑化
    pub fn smoothing3x3(target: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let filter3x3 = GaussianFilter::new(vec![vec![1, 2, 1], vec![2, 4, 2], vec![1, 2, 1]]);
        filter3x3.convolution(target)
    }

    /// 5x5のフィルターで平滑化
    pub fn smoothing5x5(target: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
        let filter5x5 = GaussianFilter::new(vec![
            vec![1, 4, 6, 4, 1],
            vec![4, 16, 24, 16, 4],
            vec![6, 24, 36, 24, 6],
            vec![4, 16, 24, 16, 4],
            vec![1, 4, 6, 4, 1],
        ]);
        filter5x5.convolution(target)
    }
}


#[cfg(test)]
mod test_gaussian_filter {
    use super::*;

    #[test]
    fn test_gaussian_filter() {
        // フィルター
        let filter = GaussianFilter::new(vec![vec![1, 2, 1], vec![2, 4, 2], vec![1, 2, 1]]);
    
        let target = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 9, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
    
        let res = filter.convolution(&target);
        println!("{:?}", res);
    }
    
    #[test]
    fn test_smoothing3x3() {
        let target = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 16, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
    
        // 平滑化
        let res = GaussianFilter::smoothing3x3(&target);
    
        assert_eq!(
            &res,
            &vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 1, 2, 1, 0],
                vec![0, 2, 4, 2, 0],
                vec![0, 1, 2, 1, 0],
                vec![0, 0, 0, 0, 0]
            ]
        );
    
        // 表示
        println!("{:?}", &res);
    }
    
    #[test]
    fn test_smoothing5x5() {
        let target = vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 256, 0, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0],
        ];
    
        // 平滑化
        let res = GaussianFilter::smoothing5x5(&target);
    
        assert_eq!(
            &res,
            &vec![
                vec![1, 4, 6, 4, 1],
                vec![4, 16, 24, 16, 4],
                vec![6, 24, 36, 24, 6],
                vec![4, 16, 24, 16, 4],
                vec![1, 4, 6, 4, 1],
            ]
        );
    
        // 表示
        println!("{:?}", &res);
    }    
}
