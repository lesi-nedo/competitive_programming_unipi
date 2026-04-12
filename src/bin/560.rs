use competitive_programming_unipi::{get_single_num, get_input};
use std::collections::HashMap;


fn sub_sum_k(els: &[i32], k: i32) -> usize {
    let mut map: HashMap<i32, usize> = HashMap::from([(0,1)]);
    let mut res = 0usize;

    els.iter().fold(
        0, |sum, &el| {
            let itr_sum = sum + el;
            let key = itr_sum - k;
            if let Some(c) = map.get(&key){
                res += c;
            }
            *map.entry(itr_sum).or_insert(0) += 1;
            itr_sum
        }
    );

    res
}

fn main() {
    let mut input : Vec<i32> = Vec::new();
    input = get_input(input).unwrap();
    let k: i32 = get_single_num();
    println!("{}", sub_sum_k(&input, k));

}