mod day1;
use day1::{day1_part1_func, day1_part1_imp};

const DAY1_INPUT: &str = include_str!("../inputs/day1_part1.csv");

fn main() {
    let day1_input: Vec<&'static str> = DAY1_INPUT.lines().collect();

    let password_func = day1_part1_func(&day1_input);
    println!("{:?}", password_func);

    let password_imp = day1_part1_imp(&day1_input);
    println!("{:?}", password_imp);
}
