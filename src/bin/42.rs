use competitive_programming_unipi::get_input;

fn trap(arr: &mut [i64]) -> i64 {
    let mut res: i64 = 0;
    let mut fst: usize = 0;
    for ind in 0..arr.len()-1 {
        let scd = ind + 1;
        let diff = std::cmp::max(0, arr[fst] - arr[scd]);
        let mut b_ind = ind;
        let min_w = std::cmp::min(arr[scd], arr[fst]);
        while arr[b_ind] < arr[scd] && b_ind > fst {
            let wat = min_w - arr[b_ind];
            arr[b_ind] += wat;
            res += wat;
            b_ind = b_ind.saturating_sub(1);
        }


        if diff == 0 {
            fst = scd;
        }

    }
    res

}

fn main() {
    let mut input: Vec<i64> = Vec::new();

    input = get_input(input);
    let res = trap(&mut input);

    println!("{res}");

}