enum Play {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Loose,
}
struct Game {
    you: Play,
    opponent: Play,
}

struct Strategy {
    opponent: Play,
    outcome: Outcome,
}

#[derive(Debug)]
struct InvalidPlayIndexError;
#[derive(Debug)]
struct InvalidOutcomeIndexError;

impl Play {
    fn new_you(you: &str) -> Result<Play, InvalidPlayIndexError> {
        match you {
            "X" => Ok(Play::Rock),
            "Y" => Ok(Play::Paper),
            "Z" => Ok(Play::Scissors),
            _ => Err(InvalidPlayIndexError),
        }
    }

    fn new_opponent(opponnent: &str) -> Result<Play, InvalidPlayIndexError> {
        match opponnent {
            "A" => Ok(Play::Rock),
            "B" => Ok(Play::Paper),
            "C" => Ok(Play::Scissors),
            _ => Err(InvalidPlayIndexError),
        }
    }
}

impl Outcome {
    fn new(outcome: &str) -> Result<Outcome, InvalidOutcomeIndexError> {
        match outcome {
            "X" => Ok(Outcome::Loose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(InvalidOutcomeIndexError),
        }
    }
}

impl Game {
    fn outcome(&self) -> Outcome {
        match *self {
            Game {
                you: Play::Rock,
                opponent: Play::Rock,
            } => Outcome::Draw,
            Game {
                you: Play::Rock,
                opponent: Play::Paper,
            } => Outcome::Loose,
            Game {
                you: Play::Rock,
                opponent: Play::Scissors,
            } => Outcome::Win,
            Game {
                you: Play::Paper,
                opponent: Play::Rock,
            } => Outcome::Win,
            Game {
                you: Play::Paper,
                opponent: Play::Paper,
            } => Outcome::Draw,
            Game {
                you: Play::Paper,
                opponent: Play::Scissors,
            } => Outcome::Loose,
            Game {
                you: Play::Scissors,
                opponent: Play::Rock,
            } => Outcome::Loose,
            Game {
                you: Play::Scissors,
                opponent: Play::Paper,
            } => Outcome::Win,
            Game {
                you: Play::Scissors,
                opponent: Play::Scissors,
            } => Outcome::Draw,
        }
    }

    fn play(&self) -> u32 {
        let base_points = match self.you {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };

        let outcome_points: u32 = match self.outcome() {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loose => 0,
        };

        base_points + outcome_points
    }
}

impl Strategy {
    fn you(&self) -> Play {
        match *self {
            Strategy {
                opponent: Play::Rock,
                outcome: Outcome::Draw,
            } => Play::Rock,
            Strategy {
                opponent: Play::Rock,
                outcome: Outcome::Loose,
            } => Play::Scissors,
            Strategy {
                opponent: Play::Rock,
                outcome: Outcome::Win,
            } => Play::Paper,
            Strategy {
                opponent: Play::Paper,
                outcome: Outcome::Draw,
            } => Play::Paper,
            Strategy {
                opponent: Play::Paper,
                outcome: Outcome::Loose,
            } => Play::Rock,
            Strategy {
                opponent: Play::Paper,
                outcome: Outcome::Win,
            } => Play::Scissors,
            Strategy {
                opponent: Play::Scissors,
                outcome: Outcome::Draw,
            } => Play::Scissors,
            Strategy {
                opponent: Play::Scissors,
                outcome: Outcome::Loose,
            } => Play::Paper,
            Strategy {
                opponent: Play::Scissors,
                outcome: Outcome::Win,
            } => Play::Rock,
        }
    }
    fn play(&self) -> u32 {
        let base_points: u32 = match self.you() {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        };

        let outcome_points: u32 = match self.outcome {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loose => 0,
        };

        base_points + outcome_points
    }
}

fn parse_game(line: &str) -> Game {
    let game_vec = line.split(" ").collect::<Vec<&str>>();
    Game {
        you: Play::new_you(game_vec[1]).unwrap(),
        opponent: Play::new_opponent(game_vec[0]).unwrap(),
    }
}

fn parse_strategy(line: &str) -> Strategy {
    let strat_vec = line.split(" ").collect::<Vec<&str>>();

    Strategy {
        opponent: Play::new_opponent(strat_vec[0]).unwrap(),
        outcome: Outcome::new(strat_vec[1]).unwrap(),
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(parse_game).collect::<Vec<Game>>();
    Some(games.iter().map(|g| g.play()).sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let strategies = input.lines().map(parse_strategy).collect::<Vec<Strategy>>();
    Some(strategies.iter().map(|s| s.play()).sum::<u32>())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
