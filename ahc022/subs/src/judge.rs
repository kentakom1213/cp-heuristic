use itertools::Itertools;

use crate::get_input;

/// # Judge
/// 入出力を行う
pub struct Judge;

impl Judge {
    /// 温度を設定する
    pub fn set_temperature(temperature: &Vec<Vec<usize>>) {
        for row in temperature {
            println!("{}", row.iter().join(" "));
        }
    }

    /// 温度を計測する
    pub fn measure(i: usize, y: isize, x: isize) -> usize {
        println!("{} {} {}", i, x, y);
        let v = get_input!(isize);
        if v >= 0 {
            v as usize
        } else {
            println!("something went wrong. i={} y={} x={}", i, x, y);
            std::process::exit(1);
        }
    }

    /// 答えを出力する
    pub fn answer(estimate: Vec<usize>) {
        println!("-1 -1 -1");
        println!("{}", estimate.iter().join("\n"));
    }
}

impl std::fmt::Debug for Judge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Judge")
    }
}
