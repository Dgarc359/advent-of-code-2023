use regex::Regex;

fn main() {
    let input = include_str!("./p1.txt");
    let output = get_sum(input);
    dbg!(output);
}

fn get_sum(input: &str) -> usize {
    let mut sum = 0_usize;

    //let symbol_coords = find_symbols(input);

    let valid_nums = find_valid_nums(input);

    for num in valid_nums.iter() {
        sum = sum.saturating_add(*num);
    }

    sum
}

fn find_valid_nums(input: &str) -> Vec<usize> {
    let re = Regex::new(r"[0-9]").unwrap();
    let mut valid_nums = vec![];

    let rows = input.split(0xA as char);

    let mut num_str = String::from("");

    for (y, row) in rows.enumerate() {
        dbg!(&row);
        for (x, c) in row.chars().enumerate() {
            dbg!(c);
            dbg!(has_adjacent_symbols(input, x, y, row.len()));

            /*
             * keep track of # as it's being read
             * keep a separate flag that is whether or not that number is valid for the usm
             * default is false.
             * check cardinals and diagonals
             * only check that flag and push to valid_nums when that flag has been set to true
             */
            if has_adjacent_symbols(input, x, y, row.len()) {
                num_str = String::from("");
                continue;
            }

            if re.is_match(String::from(c).as_str()) {
                num_str.push(c);
                dbg!(&num_str, &c);
            } else {
                match num_str.as_str().parse::<usize>() {
                    Ok(i) => {
                        valid_nums.push(i);
                    }
                    Err(_) => {}
                }

                num_str = String::from("");
            }
        }
    }

    dbg!(&valid_nums);

    valid_nums
}

fn has_adjacent_symbols(input: &str, x: usize, y: usize, padding: usize) -> bool {
    cardinals_have_symbols(input, x, y, padding) && diagonals_have_symbols(input, x, y, padding)
}

fn cardinals_have_symbols(input: &str, x: usize, y: usize, padding: usize) -> bool {
    dbg!("checking cardinals", );
    // what is this wizardry? am I destructuring the array?? idk.
    // see reference: https://users.rust-lang.org/t/fast-removing-chars-from-string/24554
    let filtered_input = input.replace(&['\n'][..], "");
    let re = &Regex::new(r"[$+#*]").unwrap();
    let north = padding.saturating_mul(y.saturating_sub(1)) + x;
    let south = (padding
        .saturating_mul(y.saturating_add(1))
        .saturating_add(x)
        .saturating_add(1))
    .clamp(0, filtered_input.len() - 1);
    let east = padding.saturating_mul(y).saturating_add(x).saturating_add(1).clamp(0, filtered_input.len() - 1);
    let west =  padding.saturating_mul(y).saturating_add(x).saturating_sub(1).clamp(0, filtered_input.len() - 1);

    dbg!(north, south, east, west, input, padding, &filtered_input);

    let n_s: bool = match filtered_input.chars().nth(north) {
        Some(i) => {
            dbg!(north, &filtered_input.chars().nth(north));
            re.is_match(String::from(i).as_str())
        }
        None => panic!("unable to index north symbol"),
    };

    let s_s: bool = match filtered_input.chars().nth(south) {
        Some(i) => {
            dbg!(south, &filtered_input.chars().nth(south));
            re.is_match(String::from(i).as_str())
        }
        None => panic!("unable to index north symbol"),
    };

    let e_s = filtered_input.as_bytes()[east] as char;
    let w_s = filtered_input.as_bytes()[west] as char;

    dbg!(n_s, s_s, e_s, w_s);

    n_s || s_s || re.is_match(e_s.to_string().as_str()) || re.is_match(w_s.to_string().as_str())
}

fn diagonals_have_symbols(input: &str, x: usize, y: usize, padding: usize) -> bool {
    dbg!("checking diagonals");
    let filtered_input = input.replace(&['\n'][..], "");
    let re = &Regex::new(r"[$+#*]").unwrap();
    let north_east = padding
        .saturating_mul(y.saturating_sub(1))
        .saturating_add(x)
        .saturating_add(1);
    let north_west = padding
        .saturating_mul(y.saturating_sub(1))
        .saturating_add(x)
        .saturating_sub(1);
    let south_east = padding
        .saturating_mul(y.saturating_add(1))
        .saturating_add(x)
        .saturating_add(1).clamp(0, filtered_input.len() -1);
    let south_west = padding
        .saturating_mul(y.saturating_add(1))
        .saturating_add(x)
        .saturating_sub(1).clamp(0, filtered_input.len() - 1);

    dbg!(
        north_east,
        north_west,
        south_east,
        south_west,
        padding,
        &filtered_input
    );

    let ne_s = filtered_input.as_bytes()[north_east] as char;
    let nw_s = filtered_input.as_bytes()[north_west] as char;
    let se_s = filtered_input.as_bytes()[south_east] as char;
    let sw_s = filtered_input.as_bytes()[south_west] as char;

    dbg!(ne_s, nw_s, se_s, sw_s);

    re.is_match(ne_s.to_string().as_str())
        || re.is_match(se_s.to_string().as_str())
        || re.is_match(nw_s.to_string().as_str())
        || re.is_match(sw_s.to_string().as_str())
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

        assert_eq!(4361_usize, output);
    }
}
