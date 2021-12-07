use std::num::ParseIntError;

use anyhow::Result;
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

const fn bit(bits: usize, idx: usize) -> usize {
    bits - idx - 1
}

const fn sized_inverse(bits: usize, n: u64) -> u64 {
    n << (64 - bits) >> (64 - bits)
}

#[tracing::instrument(level = "debug", skip(items))]
fn find_most_common_bits<'a, I>(items: I, bits: usize) -> u64
where
    I: IntoIterator<Item = &'a u64>,
{
    let mut counts = vec![0; bits];
    let mut length = 0u64;

    for item in items.into_iter() {
        length += 1;

        for (idx, count) in counts.iter_mut().enumerate() {
            if item & (1 << bit(bits, idx)) == 1 << bit(bits, idx) {
                *count += 1;
            }
        }
    }

    let mut result = 0u64;
    let half = length / 2 + length % 2;

    for (idx, count) in counts.iter().enumerate() {
        result |= if *count >= half { 1 } else { 0 } << (bits - 1 - idx);
    }

    result
}

#[tracing::instrument(level = "debug", skip(items))]
fn find_rating<'a, I>(items: I, bits: usize, use_most: bool) -> u64
where
    I: IntoIterator<Item = &'a u64>,
{
    let mut items = items.into_iter().copied().collect::<Vec<_>>();

    for idx in 0..bits {
        let needle = {
            let mcb = find_most_common_bits(&items, bits);

            if use_most {
                mcb
            } else {
                sized_inverse(bits, !mcb)
            }
        };

        items = items
            .into_iter()
            .filter(|n| *n & (1 << bit(bits, idx)) == needle & (1 << bit(bits, idx)))
            .collect();

        if items.len() == 1 {
            return items[0];
        }
    }

    panic!("Should never reach here");
}

#[tracing::instrument(level = "debug")]
fn part_one() -> Result<String> {
    let mcb = find_most_common_bits(
        &input(crate::Day::day03)
            .lines()
            .map(|l| u64::from_str_radix(l, 2))
            .collect::<Result<Vec<_>, ParseIntError>>()?,
        12,
    );

    Ok((mcb * sized_inverse(12, !mcb)).to_string())
}

#[tracing::instrument(level = "debug")]
fn part_two() -> Result<String> {
    let items = input(crate::Day::day03)
        .lines()
        .map(|l| u64::from_str_radix(l, 2))
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    Ok((find_rating(&items, 12, true) * find_rating(&items, 12, false)).to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: [u64; 12] = [
        0b00100, 0b11110, 0b10110, 0b10111, 0b10101, 0b01111, 0b00111, 0b11100, 0b10000, 0b11001,
        0b00010, 0b01010,
    ];

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(find_most_common_bits(&TEST_INPUT, 5), 22);
        assert_eq!(sized_inverse(5, !find_most_common_bits(&TEST_INPUT, 5)), 9);
        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(find_rating(&TEST_INPUT, 5, true), 23);
        assert_eq!(find_rating(&TEST_INPUT, 5, false), 10);
        Ok(())
    }
}
