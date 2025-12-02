const DAY1_ERR_MSG: &str = "Inputs must start with L/R followed by numerals.";

/// Strip prefix and convert to an dial instruction as a signed number
/// Panics on invalid inputs
fn parse_step(s: &str) -> isize {
    if let Some(rest) = s.strip_prefix("L") {
        -rest.parse::<isize>().expect(DAY1_ERR_MSG)
    } else if let Some(rest) = s.strip_prefix("R") {
        rest.parse::<isize>().expect(DAY1_ERR_MSG)
    } else {
        panic!("{}", DAY1_ERR_MSG)
    }
}

/// The number of times dial lands exactly on 0. Pure functional solution.
pub fn day1_part1_func(input: &[&str]) -> isize {
    input
        .iter()
        .fold((50, 0), |acc: (isize, isize), s: &&str| {
            let step = parse_step(s);
            let tally_mod = (acc.0 + step).rem_euclid(100);
            let inc = (tally_mod == 0) as isize;

            (tally_mod, acc.1 + inc)
        })
        .1
}

/// The number of times dial lands exactly on 0. Idiomatic imperative solution.
pub fn day1_part1_imp(input: &[&str]) -> isize {
    let mut tally = 50;
    let mut zeroes = 0;

    for s in input {
        let step = parse_step(s);
        tally += step;

        if tally.rem_euclid(100) == 0 {
            zeroes += 1;
        }

        tally = tally.rem_euclid(100);
    }
    zeroes
}

/// Count how many times 0 has been crossed by comparing whether after a step, a value becomes
/// negative (crossed to the left) or over a 100 (crossed from the right)
fn zero_crosses(step: isize, tally: isize) -> isize {
    let hundreds = (tally / 100).abs();

    let adj = (tally - step == 0) as isize;

    match step {
        st if st > 0 => hundreds,
        st if st < 0 => (tally < 1).then_some(hundreds + 1 - adj).unwrap_or(0),
        _ => 0,
    }
}

/// The number of times dial crosses 0. Pure functional solution.
pub fn day1_part2_func(input: &[&str]) -> isize {
    input
        .iter()
        .fold((50, 0), |acc: (isize, isize), s: &&str| {
            let step = parse_step(s);
            let tally = acc.0 + step;
            let inc = zero_crosses(step, tally) as isize;

            (tally.rem_euclid(100), acc.1 + inc)
        })
        .1
}

/// The number of times dial crosses 0. Idiomatic imperative solution.
pub fn day1_part2_imp(input: &[&str]) -> isize {
    let mut tally = 50;
    let mut zeroes = 0;

    for s in input {
        let step = parse_step(s);
        tally += step;

        zeroes += zero_crosses(step, tally);

        tally = tally.rem_euclid(100);
    }
    zeroes
}
