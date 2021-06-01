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
        mut h: i64
    };

    let mut n = 1i64;

    let mut ans = 0i64;

    while h > 1{
        h = h / 2;
        ans += n;
        n = n * 2;
    }

    ans += n;

    println!("{}", ans)
}
