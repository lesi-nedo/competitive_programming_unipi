use std::collections::HashMap;
use std::io::{self};

fn find_indexes(n: usize, k: i64, arr: &[usize]) -> (usize, usize) {
    let mut result: (usize, usize) = (0, 0);
    let mut els: HashMap<usize, i32> = HashMap::with_capacity(n);
    let mut max: i64 = 0;
    let mut left: usize = 0;
    let mut diff: i64 = 0;
    let mut tot: i64 = 0;
    for (ind, el) in arr.iter().enumerate() {
        let el = *el;
        *els.entry(el).or_insert(0) += 1;
        tot += 1;
        if let Some(&value) = els.get(&el)
            && value == 1
        {
            diff += 1;
        }
        if diff <= k && tot > max {
            result = (left + 1, ind + 1);
            max = tot;
        }

        if diff > k {
            while diff > k {
                if let Some(value) = els.get_mut(&arr[left]) {
                    *value -= 1;
                    tot -= 1;
                    if *value == 0 {
                        diff -= 1;
                    }
                }
                left += 1;
            }
        }
    }
    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // Split by whitespace
    let mut it = input.split_whitespace();

    // Parse first line
    let n: usize = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();
    input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    it = input.split_whitespace();

    // Parse array
    let arr: Vec<usize> = (0..n)
        .map(|_| it.next().unwrap().parse().unwrap())
        .collect();
    let res = find_indexes(n, k, &arr);
    println!("{} {}", res.0, res.1);
}
