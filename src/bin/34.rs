use competitive_programming_unipi::{get_input, get_single_num};
use std::cmp::Ordering;
use std::net::Shutdown::Read;

fn search_range(arr: &[i64], num: i64) -> [i64; 2] {
    let mut low: usize = 0;
    let mut high: usize = arr.len();
    let mut res: [i64; 2] = [-1, -1];

    while low < high {
        let middle = low + (high - low) / 2;

        match num.cmp(&arr[middle]) {
            Ordering::Less => high = middle,
            Ordering::Equal => {
                if middle == 0 || arr[middle-1] != num {
                    res[0] = middle as i64;
                    break;
                } else {
                    high = middle;
                }
            },
            Ordering::Greater => {
                low = middle + 1;
            }
        }
    }
    low =  0;
    high = arr.len();
    while low < high {
        let middle = low + (high - low) / 2;
        match num.cmp(&arr[middle]) {
            Ordering::Greater => low = middle+1,
            Ordering::Equal => {
                if middle == arr.len()-1 || arr[middle+1] != num {
                    res[1] = middle as i64;
                    break;
                } else {
                    low = middle + 1;
                }
            },
            Ordering::Less => {
                high = middle;
            }
        }

    }
    res
}

fn main() {
    let mut input: Vec<i64> = Vec::new();
    input = get_input(input);
    let num = get_single_num();

    let res = search_range(&input, num);
    println!("{:?}", res);
}