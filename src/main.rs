use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    }

    // 行ごとの合計
    let mut hs = vec![0; h];
    // 列ごとの合計
    let mut ws = vec![0; w];

    for (i, s) in a.iter().enumerate() {
        for (j, e) in s.iter().enumerate() {
            hs[i] += e;
            ws[j] += e;
        }
    }

    let mut r = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            r[i][j] = hs[i] + ws[j] - a[i][j];
        }
    }

    for i in r {
        for j in &i {
            if *j == i.len() - 1 {
                print!("{}", j);
            } else {
                print!("{} ", j);
            }
        }
        println!();
    }
}
