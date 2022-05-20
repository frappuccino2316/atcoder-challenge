use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    }

    let mut c = vec![];
    for mut i in a {
        let mut cnt = 0;
        while i % 2 == 0 {
            cnt += 1;
            i /= 2;
        }
        c.push(cnt);
    }

    println!("{}", c.iter().min().unwrap());
}
