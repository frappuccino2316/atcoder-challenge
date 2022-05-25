use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [[usize; 2]; m],
    }

    let mut l = vec![vec![]; n + 1];
    for abi in ab {
        l[abi[0]].push(abi[1]);
        l[abi[1]].push(abi[0]);
    }

    let mut cnt = 0;
    for (m, v) in l.iter().enumerate() {
        let mut over_cnt = 0;
        for i in v {
            if *i < m {
                over_cnt += 1;
            }
        }
        if over_cnt == 1 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
