use competitive_programming_unipi::get_input;

fn max_sub_array(arr: &[i64]) -> i64 {
    let mut max: i64 = i64::MIN;
    let mut sum: i64 = 0;
    for val in arr {
        sum += val;
        if sum > max {
            max = sum;
        }
        if sum <= 0 {
            sum = 0
        }
    }
    max

}

fn main() {
    let mut input: Vec<i64> = Vec::new();

    input = get_input(input);
    let res = max_sub_array(&input);
    println!("{res}");
}