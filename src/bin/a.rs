#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        x: isize,
        y: isize,
    }
    let mut ans = (y.abs() - x.abs()).abs();
    if x + ans == y {
        ans += 0;
    } else if -x + ans == y {
        ans += 1;
    } else if -x - ans == y {
        ans += 1;
    } else if x - ans == y {
        ans += 2;
    }

    println!("{}", ans);
}
