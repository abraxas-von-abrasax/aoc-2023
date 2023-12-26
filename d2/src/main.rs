use std::fs;

#[derive(Debug)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSet {
    fn new(red: u32, green: u32, blue: u32) -> Self {
        Self {
            red,
            green,
            blue,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    max: CubeSet,
    picks: Vec<CubeSet>,
}

impl Game {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split(": ").collect();

        let game_part = parts[0];
        let pick_part = parts[1];

        let mut picks: Vec<CubeSet> = Vec::new();

        let picks_str: Vec<&str> = pick_part.split("; ").collect();

        for pick in picks_str {
            let colors: Vec<&str> = pick.split(", ").collect();

            let mut picked_red: u32 = 0;
            let mut picked_green: u32 = 0;
            let mut picked_blue: u32 = 0;

            fn parse_num(num_str: &str) -> u32 {
                num_str.parse().expect("Could not extract color number")
            }

            for color in colors {
                let num_with_col: Vec<&str> = color.split(' ').collect();

                match num_with_col[1] {
                    "red" => picked_red = parse_num(num_with_col[0]),
                    "green" => picked_green = parse_num(num_with_col[0]),
                    "blue" => picked_blue = parse_num(num_with_col[0]),
                    _ => {}
                }

                picks.push(CubeSet::new(picked_red, picked_green, picked_blue));
            }
        }

        Self {
            id: game_part
                .split(' ')
                .collect::<Vec<_>>()[1]
                .parse()
                .expect("Could not parse game ID"),
            max: CubeSet::new(12, 13, 14),
            picks,
        }
    }

    fn calculate_game_sum(self: &Self) -> u32 {
        for pick in &self.picks {
            if pick.red > self.max.red || pick.green > self.max.green || pick.blue > self.max.blue {
                return 0;
            }
        }

        self.id
    }

    fn calculate_cube_power(self: &Self) -> u32 {
        let mut min_red: u32 = 0;
        let mut min_green: u32 = 0;
        let mut min_blue: u32 = 0;

        for pick in &self.picks {
            if pick.red > min_red {
                min_red = pick.red;
            }

            if pick.green > min_green {
                min_green = pick.green;
            }

            if pick.blue > min_blue {
                min_blue = pick.blue;
            }
        }

        min_red * min_green * min_blue
    }
}

fn main() {
    let content = get_content();
    let mut sum: u32 = 0;

    for line in &content {
        let game = Game::from_line(&line);
        sum += game.calculate_game_sum();
    }

    println!("[Day 2] Part 1: {}", sum);

    sum = 0;

    for line in &content {
        let game = Game::from_line(&line);
        sum += game.calculate_cube_power();
    }

    println!("[Day 2] Part 2: {}", sum);
}


fn get_content() -> Vec<String> {
    fs::read_to_string("data/input.txt")
        .expect("Could not read input file")
        .split("\n")
        .map(|s| String::from(s))
        .collect::<Vec<String>>()
}