use regex::Regex;

fn main() {
    let input = include_str!("./p1.txt");
    let output = get_sum(input);
    dbg!(output);
}

fn get_sum(input: &str) -> u32 {
    let sum = 0_u32;

    let symbol_coords = find_symbols(input);

    let valid_nums = find_valid_nums(input, symbol_coords);
    sum
}

fn find_valid_nums(input: &str, symbol_coords: Vec<[usize; 2]>) -> Vec<usize> {
    let re = &Regex::new(r"[0-9]").unwrap();
    let mut valid_nums = vec![];

    let rows = input.split(0xA as char);

    let mut num_str = String::from("");

    let mut num_has_adjacent_symbol = false;

    for (y, row) in rows.enumerate() {
        for (x, c) in row.chars().enumerate() {
            if re.is_match(String::from(c).as_str()) {
            }
        }
    }

    valid_nums
}

fn check_cardinals_for_symbols(symbol_coords: Vec<[usize; 2]>, x: usize, y: usize) {

}

fn find_symbols(input: &str) -> Vec<[usize; 2]> {
    let re = &Regex::new(r"[$+#*]").unwrap();
    let mut coords = vec![];

    let rows = input.split(0xA as char);

    for (y, row) in rows.enumerate() {
        for (x, c) in row.chars().enumerate() {
            if re.is_match(String::from(c).as_str()) {
                coords.push([x, y]);
            }
        }
    }

    coords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
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
