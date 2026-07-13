use competitive_programming_unipi::get_input;
fn solve(arr: &[i32]) -> i32 {
    let n = arr.len() as i32;
    if n < 2 {
        return 0;
    }

    if arr[0] == 0 && n > 1 {
        return -1;
    }
    let mut jmps = 0i32;
    let mut r = 0i32;
    let mut sr = 0i32;
    while r < n - 1 {
        let mut fi = sr;
        let mut nsr = 0;
        for c in sr..=r {
            let ci = c as usize;
            if fi < c + arr[ci] {
                nsr = r + 1;
                fi = c + arr[ci];
            }
        }
        if fi == r {
            return -1;
        }
        sr = nsr;
        r = fi;
        jmps += 1;
    }
    jmps
}
fn main() {
    let mut input: Vec<i32> = Vec::new();
    input = get_input(input);
    println!("{}", solve(&input));
}
