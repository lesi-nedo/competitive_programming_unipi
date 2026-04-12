use competitive_programming_unipi::get_line;

fn min_partitions(n: String) -> i32 {
    let mut r: i32 = 0;
    for c in n.chars() {
        let t = c.to_digit(10).unwrap().cast_signed();
        if t > r {
            r = t;
        }
    }
    r
}

fn main() {
    let n = get_line().unwrap();
    let r = min_partitions(n);
    println!("{r}");
}
