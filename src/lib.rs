pub trait Fillable {
    type Input;

    fn fill_me(self, item: Self::Input) -> Self;
}

impl<T> Fillable for Vec<T> {
    type Input = T;

    fn fill_me(mut self, item: T) -> Self {
        self.push(item);
        self
    }
}

pub fn get_single_num<T> () -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    input.trim().parse::<T>().unwrap()
}


pub fn get_input<T: Fillable>(mut list: T) -> T
where
    T::Input: std::str::FromStr,
    <T::Input as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Step 1: Replace common delimiters with spaces to make split_whitespace work
    let standardized = input.replace('[', " ")
        .replace(']', " ")
        .replace(',', " ");

    // Step 2: Now split_whitespace() will find "1", "2", "3", "4" individually
    let it = standardized.split_whitespace();

    for token in it {
        // No need for complex char filtering anymore, split_whitespace handled it
        let parsed_value = token.parse::<T::Input>().unwrap();

        list = list.fill_me(parsed_value);
    }

    list
}

pub fn get_matrix<T>() -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let mut matrix = Vec::new();
    let mut row = Vec::new();
    let mut token = String::new();
    let mut depth = 0usize;

    for ch in input.chars() {
        match ch {
            '[' => {
                depth += 1;
                if depth == 2 {
                    row = Vec::new();
                }
            }
            ']' => {
                if depth == 2 {
                    if !token.trim().is_empty() {
                        row.push(token.trim().parse::<T>().unwrap());
                        token.clear();
                    }
                    matrix.push(std::mem::take(&mut row));
                }
                depth = depth.saturating_sub(1);
            }
            ',' => {
                if depth == 2 && !token.trim().is_empty() {
                    row.push(token.trim().parse::<T>().unwrap());
                    token.clear();
                }
            }
            _ => {
                if depth == 2 && !ch.is_whitespace() {
                    token.push(ch);
                }
            }
        }
    }

    matrix
}
