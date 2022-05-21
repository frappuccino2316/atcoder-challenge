use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut s = Vec::new();
    for _ in 0..n {
        input! {
            si: String,
        }
        let b: Vec<char> = si.chars().collect();
        s.push(b);
    }

    let mut d = Vec::new();
    for i in 0..=9 {
        let mut second = 0;
        let mut max_index = 0;

        for (j, v) in s.iter().enumerate() {
            for (k, e) in v.iter().enumerate() {
                if i == (*e as i32) - 48 && max_index < k {
                    max_index = k;
                } else if i == (*e as i32) - 48 && max_index == k && j != 0 {
                    second += 10;
                }
            }
        }
        d.push(second + max_index);
    }

    let mut m = 100;
    for i in d {
        if i < m {
            m = i;
        }
    }

    println!("{}", m);
}
