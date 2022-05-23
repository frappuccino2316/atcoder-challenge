use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let max = a.iter().max().unwrap();
    let mut l = vec![0; max + 1];

    for i in a {
        l[i] += 1;
    }

    println!("{:?}", l);
    let mut ans = (n * (n - 1) * (n - 2)) / 6;

    for j in l {
        if j > 2 {
            ans -= (j * (j - 1) * (j - 2)) / 6;
        }
        if j > 1 {
            ans -= ((j * (j - 1)) / 2) * (n - j);
        }
    }

    println!("{}", ans);
}
