use std::io::{self, Read};


pub trait Fillable {
    type Input;

    fn fill_me(self, item: Self::Input) -> Self;
}

impl Fillable for Vec<i64> {
    type Input = i64;

    fn fill_me(mut self, item: i64) -> Self {
        self.push(item);
        self
    }
}

impl Fillable for Vec<f64> {
    type Input = f64;

    fn fill_me(mut self, item: f64) -> Self {
        self.push(item);
        self
    }
}

impl Fillable for Vec<char> {
    type Input = char;

    fn fill_me(mut self, item: char) -> Self {
        self.push(item);
        self
    }
}

impl Fillable for Vec<String> {
    type Input = String;

    fn fill_me(mut self, item: String) -> Self {
        self.push(item);
        self
    }
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