use std::io;

fn to_nums(s: &str) -> Vec<usize> {
    s.split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect()
}

fn maximum_meetings_one_room(s: &[usize], f: &[usize]) -> Vec<usize> {
    let mut res = Vec::new();
    let mut fi: Vec<(usize, usize)> = f.iter().enumerate().map(|(i, &el)| (el, i + 1)).collect();
    if fi.is_empty() {
        return res;
    }
    fi.sort_by_key(|x| x.0);
    res.push(fi[0].1);
    let mut lv = fi[0].0;
    for &(val, ind) in fi.iter().skip(1) {
        if lv < s[ind - 1] {
            res.push(ind);
            lv = val;
        }
    }
    res.sort_unstable();
    res
}

fn main() {
    let mut lines = io::stdin().lines();
    loop {
        let s_arr: Vec<usize>;
        match lines.next() {
            Some(Ok(line)) => {
                s_arr = to_nums(&line);
            }
            Some(Err(err)) => {
                eprintln!("Error reading input s: {err}");
                break;
            }
            None => {
                break;
            }
        }
        let f_arr: Vec<usize>;
        match lines.next() {
            Some(Ok(line)) => {
                f_arr = to_nums(&line);
            }
            Some(Err(e)) => {
                eprintln!("Error reading f: {e}");
                break;
            }
            None => break,
        }
        println!("{:?}", maximum_meetings_one_room(&s_arr, &f_arr));
    }
}
