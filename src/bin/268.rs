use competitive_programming_unipi::get_input;

fn missing_number(arr: Vec<i64>) -> usize {

    let mut cont: Box<[i64]> = vec![0; arr.len()+1].into_boxed_slice();
    for el in arr {
        cont[el as usize] = 1;
    }
    let res: usize = 0;

    for (ind, val) in cont.iter().enumerate() {
        if *val == 0 {
            return ind;
        }
    }


    res

}

fn main() {
    let mut input: Vec<i64> = Vec::new();

    input = get_input(input);

    println!("{:?}", missing_number(input));

}