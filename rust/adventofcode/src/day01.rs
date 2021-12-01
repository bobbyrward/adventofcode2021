use anyhow::{anyhow, Result};
use clap::Parser;
use tracing::debug;

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

fn find_deltas(measurements: &Vec<i64>) -> Vec<i64> {
    measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .map(|(a, b)| *b - *a)
        .collect::<Vec<_>>()
}

fn find_sliding_deltas(measurements: &Vec<i64>) -> Vec<i64> {
    let sums = measurements
        .iter()
        .zip(measurements.iter().skip(1))
        .zip(measurements.iter().skip(2))
        .map(|((a, b), c)| a + b + c)
        .collect::<Vec<_>>();

    sums.iter()
        .zip(sums.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>()
}

fn part_one() -> Result<String> {
    let measurements = input(crate::Day::day01)
        .lines()
        .map(|s| s.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let deltas = find_deltas(&measurements);

    Ok(deltas.iter().filter(|x| **x > 0).count().to_string())
}

fn part_two() -> Result<String> {
    let measurements = input(crate::Day::day01)
        .lines()
        .map(|s| s.trim().parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let deltas = find_sliding_deltas(&measurements);

    Ok(deltas.iter().filter(|x| **x > 0).count().to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_find_paired() -> Result<()> {
        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        let report = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let deltas = find_deltas(&report);

        assert_eq!(deltas.len(), 9);

        tracing::debug!(deltas=?deltas);

        assert_eq!(deltas.iter().filter(|x| **x > 0).count(), 7);
        assert_eq!(deltas.iter().filter(|x| **x < 0).count(), 2);
        assert_eq!(deltas.iter().filter(|x| **x == 0).count(), 0);

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        let report = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let deltas = find_sliding_deltas(&report);

        assert_eq!(deltas.len(), 7);

        tracing::debug!(deltas=?deltas);

        assert_eq!(deltas.iter().filter(|x| **x > 0).count(), 5);
        assert_eq!(deltas.iter().filter(|x| **x < 0).count(), 1);
        assert_eq!(deltas.iter().filter(|x| **x == 0).count(), 1);

        Ok(())
    }
}
