use {
    anyhow::{anyhow, Error, Result},
    common::input::{from_path, inputs},
    std::{convert::TryFrom, str::FromStr},
};

fn main() -> Result<()> {
    let inputs = inputs(to_syntax, from_path("day10/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(inputs: &[Syntax]) -> Result<()> {
    println!(
        "Day 10 Part 1 => {:?}",
        inputs
            .iter()
            .filter_map(|s| match s {
                Syntax::Corrupted(bracket) => Some(bracket.score()),
                _ => None,
            })
            .sum::<u32>()
    );

    Ok(())
}

fn part2(_inputs: &[Syntax]) -> Result<()> {
    println!("Day 10 Part 1 => ");

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
}

impl Bracket {
    fn score(&self) -> u32 {
        match self {
            Bracket::Round => 3,
            Bracket::Square => 57,
            Bracket::Curly => 1197,
            Bracket::Angle => 25137,
        }
    }
}

#[derive(Debug, PartialEq)]
enum Chunk {
    Open(Bracket),
    Close(Bracket),
}

impl TryFrom<char> for Chunk {
    type Error = Error;

    fn try_from(ch: char) -> Result<Self> {
        match ch {
            '(' => Ok(Chunk::Open(Bracket::Round)),
            '[' => Ok(Chunk::Open(Bracket::Square)),
            '{' => Ok(Chunk::Open(Bracket::Curly)),
            '<' => Ok(Chunk::Open(Bracket::Angle)),
            ')' => Ok(Chunk::Close(Bracket::Round)),
            ']' => Ok(Chunk::Close(Bracket::Square)),
            '}' => Ok(Chunk::Close(Bracket::Curly)),
            '>' => Ok(Chunk::Close(Bracket::Angle)),
            _ => Err(anyhow!("invalid token {}", ch)),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Syntax {
    Complete,
    Incomplete,
    Corrupted(Bracket),
}

impl FromStr for Syntax {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut stack: Vec<Bracket> = Vec::new();

        for ch in s.chars() {
            let chunk = Chunk::try_from(ch)?;
            match chunk {
                Chunk::Open(bracket) => {
                    stack.push(bracket);
                }
                Chunk::Close(bracket) => match stack.pop() {
                    Some(opening) if opening == bracket => (),
                    _ => {
                        return Ok(Syntax::Corrupted(bracket));
                    }
                },
            }
        }

        if stack.is_empty() {
            Ok(Syntax::Complete)
        } else {
            Ok(Syntax::Incomplete)
        }
    }
}

fn to_syntax(s: String) -> Option<Syntax> {
    s.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_inputs() -> Result<()> {
        let inputs = inputs(to_syntax, from_path("test/test.txt")?);

        assert_eq!(inputs.len(), 10);
        assert_eq!(inputs.iter().filter(|s| s == &&Syntax::Complete).count(), 0);
        assert_eq!(
            inputs.iter().filter(|s| s == &&Syntax::Incomplete).count(),
            5
        );
        assert_eq!(
            inputs
                .iter()
                .filter(|s| matches!(s, &&Syntax::Corrupted(_)))
                .count(),
            5
        );
        assert_eq!(
            inputs
                .iter()
                .filter_map(|s| match s {
                    Syntax::Corrupted(bracket) => Some(bracket.score()),
                    _ => None,
                })
                .sum::<u32>(),
            26397
        );

        Ok(())
    }

    #[test]
    fn check_parse_syntax() -> Result<()> {
        let s = "[<>({}){}[([])<>]]".to_string();
        let syntax: Syntax = s.parse()?;
        assert_eq!(syntax, Syntax::Complete);

        Ok(())
    }
}
