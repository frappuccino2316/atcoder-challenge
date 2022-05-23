use proconio::input;

fn main() {
    input! {
        n: usize,
        cp: [[usize; 2]; n],
        q: usize,
        lr: [[usize; 2]; q],
    }

    for lri in lr {
        let mut first_sum = 0;
        let mut second_sum = 0;
        let cp_block = &cp[lri[0] - 1..lri[1]];

        for cpi in cp_block {
            if cpi[0] == 1 {
                first_sum += cpi[1];
            } else {
                second_sum += cpi[1];
            }
        }

        println!("{} {}", first_sum, second_sum);
    }
}
