const ERROR_MESSAGE: &'static str = "Inputs must start with L/R followed by numerals.";

pub fn count_zeroes_func(input: &Vec<&'static str>) -> i64 {
    input
        .iter()
        .fold((50, 0), |acc: (i64, i64), s: &&str| {
            let step = {
                if let Some(rest) = s.strip_prefix("L") {
                    rest.parse::<i64>().expect(ERROR_MESSAGE) * -1
                } else if let Some(rest) = s.strip_prefix("R") {
                    rest.parse::<i64>().expect(ERROR_MESSAGE)
                } else {
                    unreachable!("{}", ERROR_MESSAGE)
                }
            };

            let tally = acc.0 + step;
            if tally % 100 == 0 {
                (tally, acc.1 + 1)
            } else {
                (tally, acc.1)
            }
        })
        .1
}

pub fn count_zeroes_imp(input: &Vec<&'static str>) -> i64 {
    let mut tally = 50;
    let mut occurrences = 0;

    for s in input {
        if let Some(rest) = s.strip_prefix("L") {
            let step = rest.parse::<i64>().expect(ERROR_MESSAGE) * -1;
            tally += step;
            if tally % 100 == 0 {
                occurrences += 1;
            }
        } else if let Some(rest) = s.strip_prefix("R") {
            let step = rest.parse::<i64>().expect(ERROR_MESSAGE);
            tally += step;
            if tally % 100 == 0 {
                occurrences += 1;
            }
        }
    }
    occurrences
}
