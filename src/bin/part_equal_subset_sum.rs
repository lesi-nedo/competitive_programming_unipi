use std::io::{self, Read};

fn part_equal_subset_sum(arr: &[usize]) -> bool {
    let n = arr.len();
    if n < 2 {
        return false;
    }
    let sum: usize = arr.iter().sum();
    if sum % 2 == 1 {
        return false;
    }
    let target = sum / 2;
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &x in arr {
        if x > target {
            return false;
        }
        for ind in (x..=target).rev() {
            dp[ind] = dp[ind] || dp[ind - x];
        }
        if dp[target] {
            break;
        }
    }
    dp[target]
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    for line in input.lines() {
        let mut arr: Vec<usize> = Vec::new();
        let itr = line.split_whitespace();
        for el in itr {
            arr.push(el.parse().unwrap());
        }
        println!("{}", part_equal_subset_sum(&arr));
    }
}
