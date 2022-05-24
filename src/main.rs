use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    let sa: i64 = a.iter().sum();
    let sb: i64 = b.iter().sum();

    if (sa + k) % 2 == sb % 2 && (sb - sa).abs() <= k {
        println!("Yes");
    } else {
        println!("No");
    }
}
