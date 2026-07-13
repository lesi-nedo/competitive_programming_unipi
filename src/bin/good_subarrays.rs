use std::collections::HashMap;
use std::io::{self, Read};

fn good_subarray(nums: &[i32]) -> usize {
    let mut map: HashMap<i32, usize> = HashMap::from([(0, 1)]);
    let mut r = 0usize;
    nums.iter().fold((0, 1usize), |(sum, len), e| {
        let c = sum + e;
        let d = c - (len as i32);
        if let Some(d) = map.get(&d) {
            r += d;
        }
        *map.entry(d).or_insert(0) += 1;
        (c, len + 1)
    });

    r
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut itr = input.split_whitespace();
    let n: usize = itr.next().unwrap().parse().unwrap();
    for _ in 0..n {
        let _: usize = itr.next().unwrap().parse().unwrap();
        let nums: Vec<i32> = itr
            .next()
            .unwrap()
            .chars()
            .filter_map(|c| Some(c.to_digit(10).unwrap() as i32))
            .collect();
        println!("{}", good_subarray(&nums));
    }
}
