use item::Item;
use im::HashSet;
use itertools::Itertools;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let rugsacks = include_str!("input.txt");

    let sum = rugsacks
    .lines()
    .map(|line| -> color_eyre::Result<_> {
        let (first, second) = line.split_at(line.len() / 2);

        let first_half: HashSet<Item> = first.bytes().map(Item::try_from).collect::<Result<HashSet<_>, _>>()?;

        itertools::process_results(second
            .bytes()
            .map(Item::try_from), |mut it| {
                it.find(|&item| first_half.contains(&item))
                .map(|item| item.priority())
                .ok_or_else(|| color_eyre::eyre::eyre!("The 2 compartments have no items in common"))
            })?
    })
    .sum::<color_eyre::Result<usize>>()?;
    
    dbg!(sum);

    let group_sum: usize = rugsacks
    .lines()
    .map(|line| {
        line.bytes().map(|b| b.try_into().unwrap()).collect::<HashSet<Item>>()
    })
    .chunks(3)
    .into_iter()
    .map(|chunks| {
        chunks
        .reduce(|a, b| a.intersection(b))
        .expect("We must always have 3 chunks")
        .iter()
        .next()
        .expect("Too many item types in common")
        .priority()
    })
    .sum();

    dbg!(group_sum);

    Ok(())
}

mod item {
    #[repr(transparent)]
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl TryFrom<u8> for Item {
        type Error = color_eyre::Report;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a' ..=b'z' | b'A' ..=b'Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!(
                    "{} is not a valid item",
                    value as char,
                ))
            }
        }
    }

    impl std::fmt::Debug for Item {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }

    impl Item {
        pub(crate) fn priority(self) -> usize {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }
}