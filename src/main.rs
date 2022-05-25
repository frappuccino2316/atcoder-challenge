use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [[usize; 2]; m],
    }

    let mut l = vec![vec![]; n + 1];
    for abi in ab {
        if abi[0] > abi[1] {
            l[abi[0]].push(abi[1])
        } else {
            l[abi[1]].push(abi[0]);
        }
    }

    let mut cnt = 0;
    for m in l {
        if m.len() == 1 {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
