fn main() {
    let r: i64 = 0;
    let test: i64 = (r - 1).rem_euclid(7);

    println!("TEST: {test}");
}
