use std::io;
use std::io::Read;
use std::cmp::{Reverse, max};

fn design_course(n: usize, arr: &mut Vec<[i32; 2]>) -> usize {
    if arr.is_empty() {
        return 0;
    }

    arr.sort_by_key(|bd| (bd[0], Reverse(bd[1])));
    let mut d = vec![i32::MAX; n+1];
    d[0] = i32::MIN;
    let mut res = 0;

    for ind in 0..n {
        let cd = arr[ind][1];
        let l = d.partition_point(|&x| x <= cd);
        if d[l.saturating_sub(1)] < cd && cd < d[l] {
            d[l] = cd;
            res = max(res, l);
        }
    }

    res
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let n: usize = lines.next().unwrap().split_whitespace().next().unwrap().parse().unwrap();
    let mut arr: Vec<[i32; 2]> = lines
        .map(|line| {
            let mut parts = line.split_whitespace();

            [
                parts.next().and_then(|s| s.parse().ok()).unwrap(),
                parts.next().and_then(|s| s.parse().ok()).unwrap()
            ]
        })
        .collect();
    println!("{}", design_course(n, &mut arr));
}