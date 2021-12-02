use std::str::FromStr;

use anyhow::{anyhow, Result};
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

enum SubCommand {
    Forward(i64),
    Up(i64),
    Down(i64),
}

impl FromStr for SubCommand {
    type Err = anyhow::Error;

    fn from_str(command: &str) -> Result<Self, <Self as FromStr>::Err> {
        let mut command_parts = command.split(' ');

        match (command_parts.next(), command_parts.next()) {
            (Some("forward"), Some(n)) => Ok(SubCommand::Forward(n.parse()?)),
            (Some("up"), Some(n)) => Ok(SubCommand::Up(n.parse()?)),
            (Some("down"), Some(n)) => Ok(SubCommand::Down(n.parse()?)),
            _ => Err(anyhow!("Unrecognized line: '{:?}'", command)),
        }
    }
}

fn sub_part_one<I, S>(commands: I) -> i64
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let (x, y) = commands
        .into_iter()
        .fold((0, 0), |(mut x, mut y), command| {
            let command = command.as_ref().parse::<SubCommand>().unwrap();

            match command {
                SubCommand::Forward(n) => x += n,
                SubCommand::Up(n) => y -= n,
                SubCommand::Down(n) => y += n,
            };

            (x, y)
        });

    x * y
}

fn sub_part_two<I, S>(commands: I) -> i64
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let (_, x, y) = commands
        .into_iter()
        .fold((0, 0, 0), |(mut aim, mut x, mut y), command| {
            let command = command.as_ref().parse::<SubCommand>().unwrap();

            match command {
                SubCommand::Forward(n) => {
                    x += n;
                    y += aim * n;
                }
                SubCommand::Up(n) => aim -= n,
                SubCommand::Down(n) => aim += n,
            };

            (aim, x, y)
        });

    x * y
}

fn part_one() -> Result<String> {
    Ok(sub_part_one(input(crate::Day::day02).lines()).to_string())
}

fn part_two() -> Result<String> {
    Ok(sub_part_two(input(crate::Day::day02).lines()).to_string())
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing_test::traced_test;

    const TEST_INPUT: &'static [&'static str] = &[
        "forward 5",
        "down 5",
        "forward 8",
        "up 3",
        "down 8",
        "forward 2",
    ];

    #[traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        assert_eq!(sub_part_one(TEST_INPUT), 150);
        assert_eq!(sub_part_one(input(crate::Day::day02).lines()), 2120749);

        Ok(())
    }

    #[traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        assert_eq!(sub_part_two(TEST_INPUT), 900);
        assert_eq!(sub_part_two(input(crate::Day::day02).lines()), 2138382217);
        Ok(())
    }
}
