/// abs_diffに対応
pub trait UsizeTools {
    fn abs_diff(&self, other: Self) -> Self;
    fn sq_err(&self, other: Self) -> Self;
}

impl UsizeTools for usize {
    /// 差の絶対値を求める
    fn abs_diff(&self, other: Self) -> Self {
        if *self > other {
            *self - other
        } else {
            other - *self
        }
    }

    /// 差の2乗を求める
    fn sq_err(&self, other: Self) -> Self {
        let d = self.abs_diff(other);
        d * d
    }
}
