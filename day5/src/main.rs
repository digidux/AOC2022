use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_while1},
    combinator::{all_consuming, map, opt, map_res},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};
use itertools::Itertools;
use smallvec::SmallVec;

fn main() {
    let mut lines = include_str!("input.txt").lines();

    let crate_lines: Vec<_> = (&mut lines).map_while(|line| {
        all_consuming(parse_crate_line)(line)
            .finish()
            .ok()
            .map(|(_, line)| line)
    })
    .collect();

    let mut piles = Piles(transpose_reverse(crate_lines));
    println!("{piles:?}");

    assert!(lines.next().unwrap().is_empty());

    for instruction in lines.map(|line| all_consuming(parse_instruction)(line).finish().unwrap().1) {
        println!("{instruction:?}");
        piles.apply(instruction); 
        println!("{piles:?}");
    }

    println!(
        "answer = {}",
        piles.0.iter().map(|pile| pile.last().unwrap()).join("")
    );
}

impl std::fmt::Display for Crate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, Copy)]
struct Crate(char);

fn parse_crate(i: &str) ->IResult<&str, Crate> {
    let first_char = | s: &str| Crate(s.chars().next().unwrap());
    let f = delimited(tag("["), take(1_usize), tag("]"));
    map(f, first_char)(i)
}

fn parse_hole(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn parse_crate_or_hole(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_hole, |_| None)))(i)
}

fn parse_crate_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let (mut i, c) = parse_crate_or_hole(i)?;
    let mut v = vec![c];

    loop {
        let (next_i, maybe_c) = opt(preceded(tag(" "), parse_crate_or_hole))(i)?;

        match maybe_c {
            Some(c) => v.push(c),
            None => break,
        }

        i = next_i
    }

    Ok((i, v))
}

fn transpose_reverse<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            let mut vec = Vec::with_capacity(256);
            vec.extend(
                iters
                .iter_mut()
                .rev()
                .filter_map(|n| n.next().unwrap())
            );
            vec
        })
        .collect()
}

fn parse_number(i: &str) -> IResult<&str, usize> {
    map_res(take_while1(|c: char| c.is_ascii_digit()), |s: &str| {
        s.parse::<usize>()
    })(i)
}

fn parse_pile_number(i: &str) -> IResult<&str, usize> {
    map(parse_number, |i| i - 1)(i)
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    src: usize,
    destination: usize,
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
    map(
        tuple((
            preceded(tag( "move "), parse_number),
            preceded(tag( " from "), parse_pile_number),
            preceded(tag( " to "), parse_pile_number),
        )),
        |(quantity, src, destination)| Instruction { quantity, src, destination } 
    )(i)
}

struct Piles(Vec<Vec<Crate>>);

impl std::fmt::Debug for Piles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, pile) in self.0.iter().enumerate() {
            writeln!(f, "Pile {}: {:?}", i, pile)?;
        }
        Ok(())
    }
}

impl Piles {
    fn apply(&mut self, ins: Instruction) {
        for krate in (0..ins.quantity).map(|_| self.0[ins.src].pop().unwrap()).collect::<SmallVec<[_; 256]>>().into_iter().rev() {
            self.0[ins.destination].push(krate);
        }
    }
}