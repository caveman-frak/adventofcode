use {
    anyhow::Result,
    common::input::{from_path, inputs},
};

fn main() -> Result<()> {
    let inputs = inputs(to_usize, from_path("day8/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(inputs: &[usize]) -> Result<()> {
    println!("Day 7 Part 1 => {:?}", inputs.iter().sum::<usize>());

    Ok(())
}

fn part2(_inputs: &[usize]) -> Result<()> {
    println!("Day 8 Part 2 => ");

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Digit {
    Unknown,
    One,
    Four,
    Seven,
    Eight,
}

impl Digit {
    fn unique(count: u32) -> Option<Digit> {
        match count {
            2 => Some(Digit::One),
            3 => Some(Digit::Seven),
            4 => Some(Digit::Four),
            7 => Some(Digit::Eight),
            _ => None,
        }
    }
}

impl Default for Digit {
    fn default() -> Self {
        Digit::Unknown
    }
}

fn to_usize(s: String) -> Option<usize> {
    //None
    s.split('|').skip(1).next().map(|s| {
        s.trim()
            .split(' ')
            .map(|s| Digit::unique(s.trim().len() as u32))
            .filter(|v| v.is_some())
            .count()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_digit() {
        assert_eq!(Digit::default(), Digit::Unknown);
        assert_eq!(Digit::unique(1), None);
        assert_eq!(Digit::unique(2), Some(Digit::One));
        assert_eq!(Digit::unique(4), Some(Digit::Four));
        assert_eq!(Digit::unique(3), Some(Digit::Seven));
        assert_eq!(Digit::unique(7), Some(Digit::Eight));
    }

    #[test]
    fn check_to_u32() {
        let s =
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc";
        assert_eq!(to_usize(s.to_string()), Some(3));
    }
}
