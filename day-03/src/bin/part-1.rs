
fn main() {
    let input = include_str!("./p1.txt");
    let output = get_sum(input);
    dbg!(output);
}


fn get_sum(input: &str) -> u32 {
    let sum = 0_u32;

    for (i, c) in input.chars().enumerate() {

    }

    sum
}


mod tests {
    use super::*;

    #[cfg(test)]
    fn example_test() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let output = get_sum(input);

        assert_eq!(4361_u32, output);
    }
}
