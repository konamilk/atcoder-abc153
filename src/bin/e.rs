use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    let source = AutoSource::from("9 3
    8 3
    4 2
    2 1");
    input!{
        // from source,
        h: usize,
        n: usize,
        ab: [(usize, i64);n],
    };

    let mut dp = vec![vec![0i64;h+1]; n];

    for i in 0..n{
        let (a, b) = ab[i];
        for j in 1..=h{
            if i == 0 {
                if j > a {
                    dp[i][j] = dp[i][j - a] + b;
                }
                else {
                    dp[i][j] = b;
                }
            }
            else {
                if j > a {
                    dp[i][j] = min(dp[i-1][j], min(dp[i][j - a] + b, dp[i][j - a] + b));
                }
                else {
                    dp[i][j] = min(dp[i-1][j], b)
                }
            }
        }
    }

    // println!("{:?}", dp);
    println!("{}", dp[n-1][h])
}
