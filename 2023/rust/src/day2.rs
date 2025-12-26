fn is_valid(draws: Vec<(&str, u8)>) -> bool {
    let mut bag: [u8; 3] = [12, 13, 14];

    for (color, n) in draws {
        let bag_index = match color {
            "red" => 0,
            "green" => 1,
            _ => 2,
        };

        if n > bag[bag_index] {
            return false;
        }
    }

    return true;
}

fn extract_draws(game: &str) -> Vec<(String, u8)> {
    game.trim()
        .split(":")
        .last()
        .unwrap()
        .trim()
        .split(|c| (c == ',') || (c == ';'))
        .map(|draw| draw.trim().split(" "))
        .map(|Some((n, color))| (n.parse::<u8>().unwrape(), color))
        .collect::<Vec<(String, u8)>>()
}

pub fn n_valid_games(games: &Vec<String>) -> u32 {
    games
        .into_iter()
        .enumerate()
        .flat_map(|(i, g)| (extract_draws(g)))
        .filter(|(_, g)| is_valid(g))
        .map(|(i, _)| (i as u32) + 1)
        .sum::<u32>()
}
#[test]
fn test_sum_valid_games() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
                Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
                Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
                Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
                Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    assert_eq!(
        n_valid_games(
            &input
                .split("\n")
                .map(|l| l.to_owned())
                .collect::<Vec<String>>()
        ),
        8
    )
}
