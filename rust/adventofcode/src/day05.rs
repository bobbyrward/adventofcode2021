use std::collections::HashMap;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use clap::Parser;
use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::{input, Command};
use crate::{Dimension, DimensionedValue, Point};

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

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
struct LineSegment {
    start: Point<i64>,
    end: Point<i64>,
}

fn gen_points(start: Point<i64>, end: Point<i64>, dimension: Dimension) -> Vec<Point<i64>> {
    let min = std::cmp::min(start.get(dimension), end.get(dimension));
    let max = std::cmp::max(start.get(dimension), end.get(dimension));
    let default = DimensionedValue::new(dimension.other(), start.get(dimension.other()));

    let points = (min..=max)
        .map(|n| Point::from_dimensioned_values(default, DimensionedValue::new(dimension, n)))
        .collect::<Vec<Point<i64>>>();

    tracing::debug!(start=?start, end=?end, points=?points, range=?(min..=max).collect::<Vec<_>>(), dimension=?dimension);

    points
}

struct RangeCanBeNegativeInclusive {
    start: i64,
    end: i64,
    current: i64,
    step: i64,
}

impl RangeCanBeNegativeInclusive {
    fn new(start: i64, end: i64) -> Self {
        if start < end {
            Self {
                start,
                end,
                current: start - 1,
                step: 1,
            }
        } else {
            Self {
                start,
                end,
                current: start + 1,
                step: -1,
            }
        }
    }
}

impl Iterator for RangeCanBeNegativeInclusive {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            self.current += self.step;
            tracing::debug!(
                step = self.step,
                start = self.start,
                end = self.end,
                current = self.current,
                "step"
            );
            Some(self.current)
        }
    }
}

fn gen_points_diagonal(start: Point<i64>, end: Point<i64>) -> Vec<Point<i64>> {
    let points = RangeCanBeNegativeInclusive::new(start.x, end.x)
        .zip(RangeCanBeNegativeInclusive::new(start.y, end.y))
        .map(|(x, y)| Point::new(x, y))
        .collect::<Vec<_>>();

    tracing::debug!(
        start=?start,
        end=?end,
        points=?points,
        d=?end-start,
        x_range=?RangeCanBeNegativeInclusive::new(start.x, end.x).collect::<Vec<_>>(),
        y_range=?RangeCanBeNegativeInclusive::new(start.y, end.y).collect::<Vec<_>>(),
    );

    points
}

impl LineSegment {
    fn new(start: Point<i64>, end: Point<i64>) -> Self {
        LineSegment { start, end }
    }

    fn points(&self, include_diagonal: bool) -> Vec<Point<i64>> {
        match (self.end.x - self.start.x, self.end.y - self.start.y) {
            (_, 0) => gen_points(self.start, self.end, Dimension::X),
            (0, _) => gen_points(self.start, self.end, Dimension::Y),
            _ => {
                if !include_diagonal {
                    tracing::debug!(start=?self.start, end=?self.end, dx=?self.end.x - self.start.x, dy=?self.end.y-self.start.y, "Diagonal segment");
                    Vec::new()
                } else {
                    gen_points_diagonal(self.start, self.end)
                }
            }
        }
    }
}

static LINE_SEGMENT_REGEX: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"^(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)$").unwrap()
});

impl FromStr for LineSegment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(captures) = LINE_SEGMENT_REGEX.captures(s) {
            let segment = LineSegment::new(
                Point::new(
                    captures
                        .name("x1")
                        .ok_or_else(|| anyhow!("x1 missing"))?
                        .as_str()
                        .parse()?,
                    captures
                        .name("y1")
                        .ok_or_else(|| anyhow!("y1 missing"))?
                        .as_str()
                        .parse()?,
                ),
                Point::new(
                    captures
                        .name("x2")
                        .ok_or_else(|| anyhow!("x2 missing"))?
                        .as_str()
                        .parse()?,
                    captures
                        .name("y2")
                        .ok_or_else(|| anyhow!("y2 missing"))?
                        .as_str()
                        .parse()?,
                ),
            );

            Ok(segment)
        } else {
            Err(anyhow!("Invalid line segment: '{}'", s))
        }
    }
}

fn map_intersections<I>(segments: I, include_diagonal: bool) -> Vec<Point<i64>>
where
    I: IntoIterator<Item = LineSegment>,
{
    let mut map: HashMap<Point<i64>, i64> = HashMap::new();

    for segment in segments {
        for point in segment.points(include_diagonal) {
            *map.entry(point).or_default() += 1;
        }
    }

    map.into_iter()
        .filter_map(|(point, count)| if count > 1 { Some(point) } else { None })
        .collect::<Vec<_>>()
}

fn part_one() -> Result<String> {
    let segments = input(crate::Day::day05)
        .lines()
        .map(|s| s.parse::<LineSegment>())
        .collect::<Result<Vec<_>>>()?;

    // display_points(segments, 1024, 1024);

    let intersections = map_intersections(segments, false);

    Ok(intersections.len().to_string())
}

fn part_two() -> Result<String> {
    let segments = input(crate::Day::day05)
        .lines()
        .map(|s| s.parse::<LineSegment>())
        .collect::<Result<Vec<_>>>()?;

    // display_points(segments, 1024, 1024);

    let intersections = map_intersections(segments, true);

    Ok(intersections.len().to_string())
}

#[allow(dead_code)]
fn display_points<I>(segments: I, width: i64, height: i64, include_diagonal: bool)
where
    I: IntoIterator<Item = LineSegment>,
{
    let mut buffer = vec![0; (width * height) as usize];

    for segment in segments {
        for point in segment.points(include_diagonal) {
            buffer[(point.y * width + point.x) as usize] += 1;
        }
    }

    for row in buffer.into_iter().chunks(width as usize).into_iter() {
        println!("{}", row.map(|n| n.to_string()).join(""));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: [&'static str; 10] = [
        "0,9 -> 5,9",
        "8,0 -> 0,8",
        "9,4 -> 3,4",
        "2,2 -> 2,1",
        "7,0 -> 7,4",
        "6,4 -> 2,0",
        "0,9 -> 2,9",
        "3,4 -> 1,4",
        "0,0 -> 8,8",
        "5,5 -> 8,2",
    ];

    #[tracing_test::traced_test]
    #[test]
    fn test_part_one() -> Result<()> {
        let segments = TEST_INPUT
            .iter()
            .map(|s| s.parse::<LineSegment>())
            .collect::<Result<Vec<_>>>()?;

        assert_eq!(
            segments[0],
            LineSegment::new(Point::new(0, 9), Point::new(5, 9))
        );

        assert_eq!(
            LineSegment::new(Point::new(0, 9), Point::new(5, 9)).points(false),
            vec![
                Point::new(0, 9),
                Point::new(1, 9),
                Point::new(2, 9),
                Point::new(3, 9),
                Point::new(4, 9),
                Point::new(5, 9),
            ]
        );

        let intersections = map_intersections(segments.clone(), false);
        tracing::debug!(intersections=?intersections);

        display_points(segments, 10, 10, false);

        assert_eq!(intersections.len(), 5);

        Ok(())
    }

    #[tracing_test::traced_test]
    #[test]
    fn test_part_two() -> Result<()> {
        let segments = TEST_INPUT
            .iter()
            .map(|s| s.parse::<LineSegment>())
            .collect::<Result<Vec<_>>>()?;

        assert_eq!(
            segments[0],
            LineSegment::new(Point::new(0, 9), Point::new(5, 9))
        );

        assert_eq!(
            LineSegment::new(Point::new(1, 1), Point::new(3, 3)).points(true),
            vec![Point::new(1, 1), Point::new(2, 2), Point::new(3, 3),]
        );
        assert_eq!(
            LineSegment::new(Point::new(9, 7), Point::new(7, 9)).points(true),
            vec![Point::new(9, 7), Point::new(8, 8), Point::new(7, 9),]
        );

        let intersections = map_intersections(segments.clone(), true);
        tracing::debug!(intersections=?intersections);

        display_points(segments, 10, 10, true);

        assert_eq!(intersections.len(), 12);

        Ok(())
    }
}
