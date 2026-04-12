use competitive_programming_unipi::{Matrix, get_input, get_single_num};

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Event {
    Begin,
    End,
}

fn is_covered(ranges: Vec<Vec<i32>>, mut left: i32, right: i32) -> bool {
    let mut pairs: Vec<_> = ranges
        .iter()
        .flat_map(|els| [(els[0], Event::Begin), (els[1], Event::End)])
        .collect();
    pairs.sort_unstable();
    let mut r = 0;
    let mut found = false;
    let _result: Vec<_> = pairs
        .iter()
        .take_while(|x| {
            if x.1 == Event::Begin {
                if x.0 > left && r == 0 {
                    return false;
                }
                r += 1;
            } else {
                if x.0 >= left {
                    left = x.0 + 1;
                }
                r -= 1;
            }
            if left > right {
                found = true;
                return false;
            }
            true
        })
        .collect();
    found
}

fn main() {
    let mut ranges: Matrix<i32> = Matrix(Vec::new());
    ranges = get_input(ranges).unwrap();
    let left: i32 = get_single_num();
    let right: i32 = get_single_num();
    let res = is_covered(ranges.0, left, right);
    println!("{res}");
}
