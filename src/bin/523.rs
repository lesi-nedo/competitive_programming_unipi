use competitive_programming_unipi::{get_input, get_single_num};
use std::collections::HashMap;

fn check_sub_sum(nums: &[i32], k: i32) -> bool {
    if nums.len() < 2 {
        return false;
    }
    let mut map: HashMap<i32, i32> = HashMap::from([(0, -1)]);
    let mut acc = 0;
    for (ind, num) in nums.iter().enumerate() {
        acc += num;
        let md = acc % k;
        let iind = ind as i32;
        if let Some(&pos) = map.get(&md) {
            if iind - pos > 1 {
                return true;
            }
        } else {
            map.insert(md, iind);
        }
    }
    false
}

fn main() {
    let mut input: Vec<i32> = Vec::new();
    input = get_input(input);
    let k: i32 = get_single_num();

    println!("{}", check_sub_sum(&input, k));
}
