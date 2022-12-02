/*
--- Part Two ---

The Elf finishes helping with the tent and sneaks back over to you. "Anyway, the second column says how the round needs to end: X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win. Good luck!"

The total score is still calculated in the same way, but now you need to figure out what shape to choose so the round ends as indicated. The example above now goes like this:

    In the first round, your opponent will choose Rock (A), and you need the round to end in a draw (Y), so you also choose Rock. This gives you a score of 1 + 3 = 4.
    In the second round, your opponent will choose Paper (B), and you choose Rock so you lose (X) with a score of 1 + 0 = 1.
    In the third round, you will defeat your opponent's Scissors with Rock for a score of 1 + 6 = 7.

Now that you're correctly decrypting the ultra top secret strategy guide, you would get a total score of 12.

Following the Elf's instructions for the second column, what would your total score be if everything goes exactly according to your strategy guide?
*/
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/day2.txt").unwrap();
    let total_score: u32 = input.lines()
        .map(|line| parse_game(line.trim()))
        .map(|game| game.outcome())
        .sum();

    println!("Solution: {total_score}");
}

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl std::str::FromStr for Move {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(format!("unknown move '{}'", s)),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum SuggestedOutcome {
    Lose,
    Draw,
    Win,
}

impl std::str::FromStr for SuggestedOutcome {
    type Err = String;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(SuggestedOutcome::Lose),
            "Y" => Ok(SuggestedOutcome::Draw),
            "Z" => Ok(SuggestedOutcome::Win),
            _ => Err(format!("unknown suggestoutcome '{}'", s)),
        }
    }
}

struct RockPaperScissorsGame {
    opponent_move: Move,
    suggested_outcome: SuggestedOutcome,
}

impl RockPaperScissorsGame {
    fn outcome(&self) -> u32 {
        let player_score: u32 = match self.opponent_move {
            Move::Rock => match self.suggested_outcome {
                SuggestedOutcome::Lose => 3,
                SuggestedOutcome::Draw => 1,
                SuggestedOutcome::Win => 2,
            },
            Move::Paper => match self.suggested_outcome {
                SuggestedOutcome::Lose => 1,
                SuggestedOutcome::Draw => 2,
                SuggestedOutcome::Win => 3,
            },
            Move::Scissors => match self.suggested_outcome {
                SuggestedOutcome::Lose => 2,
                SuggestedOutcome::Draw => 3,
                SuggestedOutcome::Win => 1,
            },
        };
        let score = match self.suggested_outcome {
            SuggestedOutcome::Lose => 0,
            SuggestedOutcome::Draw => 3,
            SuggestedOutcome::Win => 6,
        };
        player_score + score
    }
}

fn parse_game(line: &str) -> RockPaperScissorsGame {
    let mut parts = line.split(' ');
    RockPaperScissorsGame { 
        opponent_move: parts.next().unwrap().to_string().parse().unwrap(),
        suggested_outcome: parts.next().unwrap().to_string().parse().unwrap(),
    }
}
