use competitive_programming_unipi::get_input;


fn leaders(els: &[i64]) -> Vec<i64> {
    let n = els.len();

    let mut stack = Vec::with_capacity(n);
    stack.push(els[0]);
    for val in els.iter().skip(1) {
        while !stack.is_empty() && val > stack.last().unwrap() {
            stack.pop();
        }
        stack.push(*val);
    }
    stack
}

fn main() {
    let mut  input: Vec<i64> = Vec::new();
    input = get_input(input);

    let leaders = leaders(&input);
    println!("{:?}", leaders);

}

