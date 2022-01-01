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
                Syntax::Corrupted(_) => Some(s.score()),
                _ => None,
            })
            .sum::<usize>()
    );

    Ok(())
}

fn part2(inputs: &[Syntax]) -> Result<()> {
    let mut incomplete = inputs
        .iter()
        .filter_map(|s| match s {
            Syntax::Incomplete(_) => Some(s.score()),
            _ => None,
        })
        .collect::<Vec<usize>>();

    incomplete.sort_unstable();

    println!("Day 10 Part 2 => {}", incomplete[incomplete.len() / 2]);

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Bracket {
    Round,
    Square,
    Curly,
    Angle,
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
    Incomplete(Vec<Bracket>),
    Corrupted(Bracket),
}

impl Syntax {
    fn score(&self) -> usize {
        match self {
            Syntax::Complete => 0,
            Syntax::Corrupted(bracket) => Syntax::corrupted_score(bracket),
            Syntax::Incomplete(brackets) => brackets
                .iter()
                .rev()
                .fold(0, |acc, b| acc * 5 + Syntax::incomplete_score(b)),
        }
    }

    fn corrupted_score(bracket: &Bracket) -> usize {
        match bracket {
            Bracket::Round => 3,
            Bracket::Square => 57,
            Bracket::Curly => 1197,
            Bracket::Angle => 25137,
        }
    }

    fn incomplete_score(bracket: &Bracket) -> usize {
        match bracket {
            Bracket::Round => 1,
            Bracket::Square => 2,
            Bracket::Curly => 3,
            Bracket::Angle => 4,
        }
    }
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
            Ok(Syntax::Incomplete(stack))
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
            inputs
                .iter()
                .filter(|s| matches!(s, &&Syntax::Incomplete(_)))
                .count(),
            5
        );
        assert_eq!(
            inputs
                .iter()
                .filter(|s| matches!(s, &&Syntax::Corrupted(_)))
                .count(),
            5
        );

        Ok(())
    }

    #[test]
    fn check_scores() -> Result<()> {
        let inputs = inputs(to_syntax, from_path("test/test.txt")?);

        assert_eq!(
            inputs
                .iter()
                .filter_map(|s| match s {
                    Syntax::Corrupted(_) => Some(s.score()),
                    _ => None,
                })
                .sum::<usize>(),
            26397
        );

        let mut incomplete = inputs
            .iter()
            .filter_map(|s| match s {
                Syntax::Incomplete(_) => Some(s.score()),
                _ => None,
            })
            .collect::<Vec<usize>>();
        assert_eq!(incomplete, vec![288957, 5566, 1480781, 995444, 294]);

        incomplete.sort_unstable();
        assert_eq!(incomplete[incomplete.len() / 2], 288957);
        Ok(())
    }

    #[test]
    fn check_parse_syntax() -> Result<()> {
        let s = "[<>({}){}[([])<>]]".to_string();
        let syntax: Syntax = s.parse()?;
        assert_eq!(syntax, Syntax::Complete);

        Ok(())
    }

    #[test]
    fn check_incomplete_score() -> Result<()> {
        let s = "<{([{{}}[<[[[<>{}]]]>[]]".to_string();
        let syntax: Syntax = s.parse()?;

        assert!(matches!(&syntax, Syntax::Incomplete(_)));
        if let Syntax::Incomplete(brackets) = &syntax {
            assert_eq!(
                brackets,
                &vec![
                    Bracket::Angle,
                    Bracket::Curly,
                    Bracket::Round,
                    Bracket::Square,
                ]
            );
        }
        assert_eq!(syntax.score(), 294);

        Ok(())
    }
}
