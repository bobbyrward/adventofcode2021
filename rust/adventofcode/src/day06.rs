use std::collections::HashMap;

use anyhow::{Context, Result};
use clap::Parser;

use crate::{input, Command};

#[derive(Debug, Parser)]
pub enum Args {
    Part1,
    Part2,
}

impl Command for Args {
    fn execute(&self) -> Result<String> {
        match self {
            Self::Part1 => part_one(),
            Self::Part2 => part_two(),
        }
    }
}

#[tracing::instrument(level = "debug", skip(fish))]
fn lantern_fish(_day: usize, fish: HashMap<u8, u64>) -> HashMap<u8, u64> {
    let mut output = HashMap::new();

    for (k, v) in fish.iter() {
        if *k == 0 {
            *output.entry(8).or_default() += v;
            *output.entry(6).or_default() += v;
        } else {
            *output.entry(k - 1).or_default() += v;
        }
    }

    output
}

#[tracing::instrument(level = "debug", skip(fish))]
fn iterate_lantern_fish<I>(fish: I, days: usize) -> u64
where
    I: IntoIterator<Item = u8>,
{
    let mut counts: HashMap<u8, u64> = HashMap::new();

    for n in fish {
        *counts.entry(n).or_default() += 1;
    }

    for day in 0..days {
        counts = lantern_fish(day, counts)
    }

    counts.values().sum()
}

fn part_one() -> Result<String> {
    Ok(iterate_lantern_fish(
        input(crate::Day::day06)
            .trim()
            .split(',')
            .map(|l| {
                l.parse::<u8>()
                    .map_err(anyhow::Error::from)
                    .with_context(|| format!("Invalid digit: '{}'", l))
            })
            .collect::<Result<Vec<_>>>()?,
        80,
    )
    .to_string())
}

fn part_two() -> Result<String> {
    Ok(iterate_lantern_fish(
        input(crate::Day::day06)
            .trim()
            .split(',')
            .map(|l| {
                l.parse::<u8>()
                    .map_err(anyhow::Error::from)
                    .with_context(|| format!("Invalid digit: '{}'", l))
            })
            .collect::<Result<Vec<_>>>()?,
        256,
    )
    .to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: [u8; 5] = [3, 4, 3, 1, 2];

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(iterate_lantern_fish(TEST_INPUT.to_vec(), 18), 26);
        assert_eq!(iterate_lantern_fish(TEST_INPUT.to_vec(), 80), 5934);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(
            iterate_lantern_fish(TEST_INPUT.to_vec(), 256),
            26_984_457_539
        );
        Ok(())
    }
}
