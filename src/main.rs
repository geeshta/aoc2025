mod secret_entrance;
use secret_entrance::{count_zeroes_func, count_zeroes_imp};

const DAY1_INPUT: &str = include_str!("../inputs/day1.csv");

fn main() {
    let day1_input: Vec<&'static str> = DAY1_INPUT.lines().collect();

    let password_func = count_zeroes_func(&day1_input);
    println!("{:?}", password_func);

    let password_imp = count_zeroes_imp(&day1_input);
    println!("{:?}", password_imp);
}
