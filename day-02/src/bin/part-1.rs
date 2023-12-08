use regex::Regex;

fn main() {
    let input = include_str!("./p1.txt");
    let game = GameConfig {
        red: 12,
        green: 13,
        blue: 14,
    };
    let output = game.get_possible_game_id_sum(input);
    dbg!(output);
}

struct GameConfig {
    red: u32,
    blue: u32,
    green: u32,
}

impl GameConfig {
    fn get_possible_game_id_sum(&self, input: &str) -> u32 {
        let mut output = 0_u32;
        let mut game_num = 1_u32;

        let games = input.split(0xA as char);

        for (_, c) in games.enumerate() {
            if c == "" {
                continue;
            }

            // we have a game row
            let game_rounds = c.split(":").nth(1).unwrap();
            let round_results = game_rounds.split(";");
            let r =
                Regex::new(r"(?P<red>\d+)? red|(?P<blue>\d+)? blue|(?P<green>\d+)? green").unwrap();
            let mut blue = 0_u32;
            let mut red = 0_u32;
            let mut green = 0_u32;

            for (_, round_str) in round_results.enumerate() {
                dbg!(round_str);
                for caps in r.captures_iter(round_str) {
                    match caps.name("blue") {
                        Some(m) => {
                            let num = m.as_str().parse::<u32>().unwrap_or(0);
                            if num > blue {
                                blue = num;
                            }
                        }
                        None => {}
                    }

                    match caps.name("green") {
                        Some(m) => {
                            let num = m.as_str().parse::<u32>().unwrap_or(0);
                            if num > green {
                                green = num;
                            }
                        }
                        None => {}
                    }

                    match caps.name("red") {
                        Some(m) => {
                            let num = m.as_str().parse::<u32>().unwrap_or(0);
                            if num > red {
                                red = num;
                            }
                        }
                        None => {}
                    }
                }

                dbg!(red, blue, green);
            }
            if red <= self.red && blue <= self.blue && green <= self.green {
                dbg!("adding to output");
                output += game_num;
            }
            game_num += 1;
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(test)]
    fn part_1() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let game = GameConfig {
            red: 12,
            green: 13,
            blue: 14,
        };
        let output = game.get_possible_game_id_sum(input);

        assert_eq!(8, output)
    }
}
