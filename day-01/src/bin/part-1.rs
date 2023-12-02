use std::iter::once;

fn main() {
    let input = include_str!("./p1.txt");
    let answer = findSumOfCalibrationValues(input);
    dbg!(answer);
}

/*
 * We're going to do this naively, in the name of completion
 *
 * Approach
 * Iterate through each char in string arr
 * collect first "number" char
 * continue iterating, anytime a new number char is encountered, set last_number char to that
 * when \n is hit, concat the two "number" chars together and cast to u32
 * keep a running_sum var to add to after we cast
 * return running sum when input is terminated
 */
fn findSumOfCalibrationValues(input: &str) -> u32 {
    let mut running_sum: u32 = 0;
    let mut first_num: u32 = 0;
    let mut last_num: u32 = 0;
    let mut first_last_flag: bool = true;

    for (i, c) in input.chars().enumerate() {
        if c == 0xA as char || input.chars().nth(i + 1).is_none() {
            let additive: u32 = format!("{}{}", first_num, last_num)
                .to_string()
                .parse::<u32>()
                .unwrap();

            running_sum = running_sum.saturating_add(additive);

            first_num = 0;
            last_num = 0;
            first_last_flag = !first_last_flag;
        } else {
            if c.is_numeric() {
                match first_last_flag {
                    true => {
                        first_num = c.to_digit(10).unwrap();
                        last_num = c.to_digit(10).unwrap();
                        first_last_flag = !first_last_flag;
                    }
                    false => {
                        last_num = c.to_digit(10).unwrap();
                    }
                }
            }
        }
    }

    running_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let output = findSumOfCalibrationValues(input);
        assert_eq!(142_u32, output);
    }
}
