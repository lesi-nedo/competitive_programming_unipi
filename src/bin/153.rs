use competitive_programming_unipi::get_input;

fn get_prev_pos(curr: usize, len: usize) -> usize {
    ((curr as i64) - 1).rem_euclid(len as i64) as usize
}

fn get_min(nums: &[i64]) -> i64 {
    let mut low: usize = 0;
    let mut high = nums.len();
    let len = nums.len();

    while low < high {
        let middle = low + (high - low) / 2;

        let prev_pos = get_prev_pos(middle, len);
        if nums[prev_pos] > nums[middle] {
            return nums[middle];
        } else if nums[middle] > nums[high - 1] {
            low = middle + 1;
        } else {
            high = middle;
        }
    }

    nums[0]
}

fn main() {
    let mut input: Vec<i64> = Vec::new();
    input = get_input(input).unwrap();
    let res = get_min(&input);
    println!("{res}");
}
