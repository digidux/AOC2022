use std::cmp::Reverse;

use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let max = include_str!("input.txt")
        .lines()
        .map(|v| v.parse::<u64>().ok())
        .batching(|it| {
            let mut sum = None;
            while let Some(Some(v)) = it.next() {
                sum = Some(sum.unwrap_or(0) + v);
            }
            sum
        })
        .max()
        .unwrap_or_default();
    
    println!("{max:?}");


    let answer = include_str!("input.txt")
    .lines()
    .map(|v| v.parse::<u64>().ok())
    .batching(|it| it.map_while(|x| x).sum1::<u64>())
    .map(Reverse)
    .k_smallest(3)
    .map(|x| x.0)
    .sum::<u64>();

    println!("{answer:?}");


    Ok(())
}
