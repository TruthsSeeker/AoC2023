use crate::utils;

pub fn part1(input_path: &str) {
    let games_raw = utils::load_file_lines(input_path);
    
    let mut games: Vec<Game> = vec![];
    for game_raw in games_raw {
        let game_rounds = parse_rounds(game_raw);
        games.push(game_rounds);
    }
    
    let valid_games = find_valid_games(&games);
    let total = valid_games.iter().sum::<u32>();
    println!("Total: {}", total);
}

pub fn part2(input_path: &str) {
    let games_raw = utils::load_file_lines(input_path);
    
    let mut games: Vec<Game> = vec![];
    for game_raw in games_raw {
        let game_rounds = parse_rounds(game_raw);
        games.push(game_rounds);
    }
    
    let mut total = 0;
    for game in &games {
        let minimum_cubes = find_minimum_cubes(&game);
        total += minimum_cubes.power();
    }
    println!("Total: {}", total);
}

fn parse_rounds(game: String) -> Game {
    let parts: Vec<&str> = game.split(":").collect();
    let game_part: Vec<&str> = parts[0].split_whitespace().collect();
    let game_id = game_part[1].trim().parse::<u32>().unwrap();

    let rounds_str = parts[1].trim();
    let rounds_parts: Vec<&str> = rounds_str.split(';').collect();
    let mut game_rounds: Vec<Round> = vec![];
    for round_str in rounds_parts {
        let mut round = Round { red: 0, blue: 0, green: 0 };
        let color_parts: Vec<&str> = round_str.split(',').collect();
        for color_part in color_parts {
            let color_number: Vec<&str> = color_part.trim().split_whitespace().collect();
            let number = color_number[0].parse::<u32>().unwrap();
            let color = color_number[1];
            match color {
                "red" => round.red = number,
                "blue" => round.blue = number,
                "green" => round.green = number,
                _ => (),
            }
        }
        game_rounds.push(round);
    }
    Game {
        id: game_id,
        rounds: game_rounds,
    }
}

fn find_valid_games(games: &Vec<Game>) -> Vec<u32> {
    let mut valid_games_ids: Vec<u32> = vec![];
    let mut valid = true;

    for game in games {
        valid = true;

        for round in &game.rounds {
            if round.red > 12 || round.green > 13 || round.blue > 14 {
                valid = false;
            }
        }

        if valid {
            valid_games_ids.push(game.id);                
        }
    }

    valid_games_ids
}

fn find_minimum_cubes(game: &Game) -> Round {
    let mut minimum_cubes = Round { red: 0, blue: 0, green: 0 };

    for round in &game.rounds {
        if round.red > minimum_cubes.red {
            minimum_cubes.red = round.red;
        }
        if round.blue > minimum_cubes.blue {
            minimum_cubes.blue = round.blue;
        }
        if round.green > minimum_cubes.green {
            minimum_cubes.green = round.green;
        }
    }

    minimum_cubes
}

#[derive(PartialEq, Debug)]
struct Round {
    red: u32,
    blue: u32,
    green: u32,
}

impl Round {
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(PartialEq, Debug)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_rounds() {
        let game = String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
        let expected_rounds = vec![
            Round { red: 4, blue: 3, green: 0 },
            Round { red: 1, blue: 6, green: 2 },
            Round { red: 0, blue: 0, green: 2 },
        ];
        let expected_game = Game {
            id: 1,
            rounds: expected_rounds,
        };
        assert_eq!(parse_rounds(game), expected_game);
    }

    #[test]
    fn test_find_valid_games() {
        let games = vec![
            Game {
                id: 1,
                rounds: vec![
                    Round { red: 4, blue: 3, green: 0 },
                    Round { red: 1, blue: 6, green: 2 },
                    Round { red: 0, blue: 0, green: 2 },
                ],
            },
            Game {
                id: 2,
                rounds: vec![
                    Round { red: 10, blue: 15, green: 12 },
                    Round { red: 2, blue: 2, green: 2 },
                    ],
                },
                Game {
                    id: 3,
                    rounds: vec![
                    Round { red: 8, blue: 9, green: 14 },
                    Round { red: 3, blue: 3, green: 3 },
                ],
            },
        ];

        let expected_valid_games_ids = vec![1];
        assert_eq!(find_valid_games(&games), expected_valid_games_ids);
    }


    #[test]
    fn test_find_minimum_cubes() {
        let game = Game {
            id: 1,
            rounds: vec![
                Round { red: 5, blue: 3, green: 2 },
                Round { red: 4, blue: 6, green: 1 },
                Round { red: 2, blue: 2, green: 4 },
            ],
        };

        let minimum_cubes = find_minimum_cubes(&game);

        assert_eq!(minimum_cubes.red, 5);
        assert_eq!(minimum_cubes.blue, 6);
        assert_eq!(minimum_cubes.green, 4);
    }
}