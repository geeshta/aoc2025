const ERROR_MESSAGE: &'static str = "Inputs must start with L/R followed by numerals.";

fn parse_step(s: &str) -> i64 {
    if let Some(rest) = s.strip_prefix("L") {
        -rest.parse::<i64>().expect(ERROR_MESSAGE)
    } else if let Some(rest) = s.strip_prefix("R") {
        rest.parse::<i64>().expect(ERROR_MESSAGE)
    } else {
        panic!("{}", ERROR_MESSAGE)
    }
}

pub fn day1_part1_func(input: &[&str]) -> i64 {
    input
        .iter()
        .fold((50, 0), |acc: (i64, i64), s: &&str| {
            let step = parse_step(s);

            let tally = acc.0 + step;
            if tally.rem_euclid(100) == 0 {
                (tally, acc.1 + 1)
            } else {
                (tally, acc.1)
            }
        })
        .1
}

pub fn day1_part1_imp(input: &[&str]) -> i64 {
    let mut tally = 50;
    let mut zeroes = 0;

    for s in input {
        let step = parse_step(s);

        tally += step;
        if tally.rem_euclid(100) == 0 {
            zeroes += 1;
        }
    }
    zeroes
}
