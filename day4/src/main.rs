use std::ops::RangeInclusive;

use itertools::Itertools;

fn main() {
    let parsed_pairs = include_str!("input.txt")
    .lines()
    .map(|line| {
        line.split(",")
        .map(|pair| {
            pair.split("-")
            .map(|range| range.parse().expect("Must be u32"))
            .collect_tuple::<(u32, u32)>()
            .map(|(start, end)| start..=end)
            .expect("Must have ranges")
        })
        .collect_tuple::<(_, _)>()
        .expect("Must be paired")
    });

    let contained = parsed_pairs
    .clone()
    .filter(|(a, b)| a.contains_or_is_contains(b))
    .count();

    let overlapped = parsed_pairs
    .filter(|(a, b)| a.overlaps_or_is_overlapped(b))
    .count();

    dbg!(contained);
    dbg!(overlapped);
}

trait InclusiveRangeExtension {
    fn contains_range(&self, other: &Self) -> bool;

    fn contains_or_is_contains(&self, other: &Self) -> bool {
        self.contains_range(other) || other.contains_range(self)
    }

    fn overlaps(&self, other: &Self) -> bool;

    fn overlaps_or_is_overlapped(&self, other: &Self) -> bool {
        self.overlaps(other) || other.overlaps(self)
    }
}

impl<T> InclusiveRangeExtension for RangeInclusive<T>
    where
        T: PartialOrd,
        {
            fn contains_range(&self, other: &Self) -> bool {
                self.contains(other.start()) && self.contains(other.end())
            }

            fn overlaps(&self, other: &Self) -> bool {
                self.contains(other.start()) || self.contains(other.end())
            }
        }