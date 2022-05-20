use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    }

    let mut sum = 0;
    for i in 1..=n {
        let mut s = 0;
        for j in i.to_string().chars() {
            s += (j as i64) - 48;
        }
        if a <= s && s <= b {
            sum += i;
        }
    }
    println!("{}", sum);
}
