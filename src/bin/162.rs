use competitive_programming_unipi::get_input;

fn find_pick_element(nums: &[i64]) -> usize {
    let mut low: usize = 0;
    let mut high = nums.len();

    let len = nums.len();
    while low < high {
        let middle = low + (high - low) / 2;
        let left = std::cmp::max(0, (low as i64) - 1) as usize;
        let right = std::cmp::min(middle + 1, len - 1);
        if nums[left] <= nums[middle] && nums[middle] >= nums[right] {
            return middle;
        } else if nums[middle] < nums[right] {
            low = middle + 1;
        } else if nums[middle] < nums[left] {
            high = middle;
        }
    }
    0
}

fn main() {
    let mut input: Vec<i64> = Vec::new();

    input = get_input(input).unwrap();

    let res = find_pick_element(&input);

    println!("{res}");
}
