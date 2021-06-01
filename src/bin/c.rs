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
    // let source = AutoSource::from("0");
    input!{
        // from source,
        n: usize,
        k: usize,
        mut h: [i64;n]
    };

    h.sort();

    if k >= n{
        println!("0");
        return
    }

    let mut ans = 0i64;

    for i in 0..n-k{
        ans += h[i];
    }

    println!("{}", ans);
}
