use std::convert::TryFrom;
use std::convert::TryInto;
use std::str::FromStr;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use once_cell::sync::Lazy;

use crate::{input, Command};

#[derive(Debug, Parser)]
pub enum Args {
    Part1,
    Part2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BingoCellStatus {
    Marked,
    Unmarked,
}

impl Default for BingoCellStatus {
    fn default() -> Self {
        BingoCellStatus::Unmarked
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BingoCell {
    value: u64,
    status: BingoCellStatus,
}

impl BingoCell {
    fn value(&self) -> u64 {
        self.value
    }

    fn status(&self) -> BingoCellStatus {
        self.status
    }

    fn mark(&mut self) {
        self.status = BingoCellStatus::Marked
    }
}

impl FromStr for BingoCell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BingoCell {
            value: s.trim().parse()?,
            ..Default::default()
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum BingoCardStatus {
    Unsolved,
    Solved { call: u64, sum: u64 },
}

impl Default for BingoCardStatus {
    fn default() -> Self {
        BingoCardStatus::Unsolved
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct BingoCard {
    cells: Vec<Vec<BingoCell>>,
    //[[BingoCell; 5]; 5],
    status: BingoCardStatus,
}

impl BingoCard {
    fn status(&self) -> BingoCardStatus {
        self.status
    }

    fn unmarked_sum(&self) -> u64 {
        self.cells
            .iter()
            .map::<u64, _>(|row| {
                row.iter()
                    .filter_map(|c| {
                        if matches!(c.status(), BingoCellStatus::Unmarked) {
                            Some(c.value())
                        } else {
                            None
                        }
                    })
                    .sum()
            })
            .sum()
    }

    fn mark_value(&mut self, value: u64) -> BingoCardStatus {
        let mut marked: Option<(usize, usize)> = None;

        for (y, row) in self.cells.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                if cell.value() == value {
                    cell.mark();

                    marked = Some((x, y));
                    break;
                }
            }

            if marked.is_some() {
                break;
            }
        }

        if let Some((x, y)) = marked {
            if self.cells[y]
                .iter()
                .all(|c| c.status() == BingoCellStatus::Marked)
                || self
                    .cells
                    .iter()
                    .all(|r| r[x].status() == BingoCellStatus::Marked)
            {
                self.status = BingoCardStatus::Solved {
                    call: value,
                    sum: self.unmarked_sum(),
                };
            }
        }

        self.status
    }
}

static ROW_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"^([ \d]{2}) ([ \d]{2}) ([ \d]{2}) ([ \d]{2}) ([ \d]{2})$").unwrap()
});

impl FromStr for BingoCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BingoCard {
            cells: s
                .lines()
                .take(5)
                .filter_map(|l| {
                    ROW_REGEX.captures(l).map(|captures| {
                        captures
                            .iter()
                            .skip(1)
                            .map(|c| {
                                c.ok_or_else(|| anyhow!("Capture failed"))
                                    .and_then(|c| c.as_str().parse::<BingoCell>())
                            })
                            .collect::<Result<Vec<_>>>()
                    })
                })
                .collect::<Result<Vec<Vec<_>>>>()?,
            status: BingoCardStatus::Unsolved,
        })
    }
}

#[derive(Debug, Default)]
struct BingoGame {
    calls: Vec<u64>,
    cards: Vec<BingoCard>,
}

impl BingoGame {
    fn find_winning_call(&mut self) -> BingoCardStatus {
        for call in &self.calls {
            for card in self.cards.iter_mut() {
                if let BingoCardStatus::Solved { call, sum } = card.mark_value(*call) {
                    return BingoCardStatus::Solved { call, sum };
                }
            }
        }

        BingoCardStatus::Unsolved
    }

    fn find_last_winner(&mut self) -> Option<BingoCardStatus> {
        let mut win_count = 0;
        let mut last_win = None;
        let card_count = self.cards.len();

        for call in &self.calls {
            for card in self.cards.iter_mut() {
                if matches!(card.status(), BingoCardStatus::Unsolved) {
                    if let BingoCardStatus::Solved { call, sum } = card.mark_value(*call) {
                        last_win = Some(BingoCardStatus::Solved { call, sum });

                        win_count += 1;

                        if win_count == card_count {
                            return last_win;
                        }
                    }
                }
            }
        }

        last_win
    }
}

enum BingoGameParserState {
    WaitingForCalls,
    Calls(Vec<u64>),
    Boards(Vec<u64>, Vec<BingoCard>),
    Error(anyhow::Error),
}

impl TryFrom<BingoGameParserState> for BingoGame {
    type Error = anyhow::Error;

    fn try_from(state: BingoGameParserState) -> Result<Self, Self::Error> {
        match state {
            BingoGameParserState::Boards(calls, cards) => Ok(BingoGame { calls, cards }),
            BingoGameParserState::Error(e) => Err(e.context("Unable to parse game")),
            _ => Err(anyhow!("Unable to parse game: Unknown error")),
        }
    }
}

impl FromStr for BingoGame {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split("\n\n")
            .fold(
                BingoGameParserState::WaitingForCalls,
                |state, chunk| match state {
                    BingoGameParserState::WaitingForCalls => {
                        let calls = chunk
                            .split(',')
                            .map(|c| {
                                c.parse()
                                    .map_err(anyhow::Error::from)
                                    .with_context(|| format!("Invalid call value: '{}'", c))
                            })
                            .collect::<Result<Vec<_>>>();

                        match calls {
                            Ok(calls) => BingoGameParserState::Calls(calls),
                            Err(e) => BingoGameParserState::Error(e),
                        }
                    }
                    BingoGameParserState::Calls(calls) => match chunk.parse::<BingoCard>() {
                        Ok(card) => BingoGameParserState::Boards(calls, vec![card]),
                        Err(e) => BingoGameParserState::Error(e),
                    },
                    BingoGameParserState::Boards(calls, mut cards) => {
                        match chunk.parse::<BingoCard>() {
                            Ok(card) => {
                                cards.push(card);
                                BingoGameParserState::Boards(calls, cards)
                            }
                            Err(e) => BingoGameParserState::Error(e),
                        }
                    }
                    _ => state,
                },
            )
            .try_into()
    }
}

impl Command for Args {
    fn execute(&self) -> Result<String> {
        match self {
            Self::Part1 => part_one(),
            Self::Part2 => part_two(),
        }
    }
}

fn part_one() -> Result<String> {
    if let BingoCardStatus::Solved { call, sum } = input(crate::Day::day04)
        .parse::<BingoGame>()?
        .find_winning_call()
    {
        Ok((call * sum).to_string())
    } else {
        Err(anyhow!("No winning call"))
    }
}

fn part_two() -> Result<String> {
    if let Some(BingoCardStatus::Solved { call, sum }) = input(crate::Day::day04)
        .parse::<BingoGame>()?
        .find_last_winner()
    {
        Ok((call * sum).to_string())
    } else {
        Err(anyhow!("No winning call"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[tracing_test::traced_test]
    #[test]
    fn test_cards() -> Result<()> {
        let card_values = "14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

        let mut card = card_values.parse::<BingoCard>()?;

        assert_eq!(card.mark_value(7), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(4), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(9), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(5), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(11), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(17), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(23), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(2), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(0), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(14), BingoCardStatus::Unsolved);
        assert_eq!(card.mark_value(21), BingoCardStatus::Unsolved);
        assert_eq!(
            card.mark_value(24),
            BingoCardStatus::Solved { call: 24, sum: 188 }
        );

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        let mut game = TEST_INPUT.parse::<BingoGame>()?;

        assert_eq!(
            game.calls,
            vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1
            ]
        );

        assert_eq!(game.cards.len(), 3);
        assert_eq!(
            game.find_winning_call(),
            BingoCardStatus::Solved { call: 24, sum: 188 }
        );
        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        let mut game = TEST_INPUT.parse::<BingoGame>()?;
        assert_eq!(
            game.find_last_winner(),
            Some(BingoCardStatus::Solved { call: 13, sum: 148 })
        );
        Ok(())
    }
}
