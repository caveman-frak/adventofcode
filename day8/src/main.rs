use {
    anyhow::Result,
    common::input::{from_path, inputs},
    std::collections::{HashMap, HashSet},
};

fn main() -> Result<()> {
    part1("day8/data/input.txt")?;
    part2("day8/data/input.txt")?;

    Ok(())
}

fn part1(path: &str) -> Result<()> {
    let inputs = inputs(to_usize, from_path(path)?);
    println!("Day 8 Part 1 => {:?}", inputs.iter().sum::<usize>());

    Ok(())
}

fn part2(path: &str) -> Result<()> {
    let inputs = inputs(to_usize2, from_path(path)?);
    println!("Day 8 Part 2 => {:?}", inputs.iter().sum::<usize>());

    Ok(())
}

#[derive(Debug, PartialEq)]
enum Digit {
    Unknown,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
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

    fn decode<T: Into<String>>(segment: &Segment, s: T) -> Option<Digit> {
        let s = sort(s);

        if s == segment.zero {
            Some(Digit::Zero)
        } else if s == segment.one {
            Some(Digit::One)
        } else if s == segment.two {
            Some(Digit::Two)
        } else if s == segment.three {
            Some(Digit::Three)
        } else if s == segment.four {
            Some(Digit::Four)
        } else if s == segment.five {
            Some(Digit::Five)
        } else if s == segment.six {
            Some(Digit::Six)
        } else if s == segment.seven {
            Some(Digit::Seven)
        } else if s == segment.eight {
            Some(Digit::Eight)
        } else if s == segment.nine {
            Some(Digit::Nine)
        } else {
            None
        }
    }

    fn value(&self) -> usize {
        match self {
            Digit::Zero => 0,
            Digit::One => 1,
            Digit::Two => 2,
            Digit::Three => 3,
            Digit::Four => 4,
            Digit::Five => 5,
            Digit::Six => 6,
            Digit::Seven => 7,
            Digit::Eight => 8,
            Digit::Nine => 9,
            _ => panic!(),
        }
    }
}

impl Default for Digit {
    fn default() -> Self {
        Digit::Unknown
    }
}

fn sort<T: Into<String>>(s: T) -> String {
    let s = s.into();
    let mut v: Vec<char> = s.trim().chars().collect();
    v.sort_unstable();
    v.iter().collect::<String>()
}

#[derive(Debug)]
struct Segment {
    zero: String,
    one: String,
    two: String,
    three: String,
    four: String,
    five: String,
    six: String,
    seven: String,
    eight: String,
    nine: String,
}

impl Segment {
    #[allow(clippy::too_many_arguments)]
    fn new<T: Into<String>>(
        zero: T,
        one: T,
        two: T,
        three: T,
        four: T,
        five: T,
        six: T,
        seven: T,
        eight: T,
        nine: T,
    ) -> Self {
        Self {
            zero: sort(zero),
            one: sort(one),
            two: sort(two),
            three: sort(three),
            four: sort(four),
            five: sort(five),
            six: sort(six),
            seven: sort(seven),
            eight: sort(eight),
            nine: sort(nine),
        }
    }

    fn solve(inputs: &[&str]) -> Self {
        let mut zero: Option<String> = None;
        let mut one: Option<String> = None;
        let mut two: Option<String> = None;
        let mut three: Option<String> = None;
        let mut four: Option<String> = None;
        let mut five: Option<String> = None;
        let mut six: Option<String> = None;
        let mut seven: Option<String> = None;
        let mut eight: Option<String> = None;
        let mut nine: Option<String> = None;

        let mut map: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();

        for s in inputs {
            match s.len() {
                2 => one = Some(s.to_string()),
                3 => seven = Some(s.to_string()),
                4 => four = Some(s.to_string()),
                7 => eight = Some(s.to_string()),
                _ => map
                    .entry(s.len())
                    .or_default()
                    .push(s.chars().collect::<HashSet<char>>()),
            }
        }
        let one_set: HashSet<char> = one.as_ref().unwrap().chars().collect();
        let four_minus_one_set: HashSet<char> = four
            .as_ref()
            .unwrap()
            .chars()
            .collect::<HashSet<char>>()
            .difference(&one_set)
            .copied()
            .collect();
        let mut v = map.get(&5).unwrap().clone();
        while let Some(s) = v.pop() {
            // 2,3 or 5
            if three.is_none() && s.is_superset(&one_set) {
                // & 1 = 3
                three = Some(s.iter().collect());
            } else if five.is_none() && s.is_superset(&four_minus_one_set) {
                // & 4 - 1 = 5
                five = Some(s.iter().collect());
            } else if three.is_some() && five.is_some() {
                // = 2
                two = Some(s.iter().collect());
            } else {
                v.insert(0, s);
            }
        }
        let three_set: HashSet<char> = three.as_ref().unwrap().chars().collect();
        let mut v = map.get(&6).unwrap().clone();
        while let Some(s) = v.pop() {
            // 0, 6 or 9
            if nine.is_none() && s.is_superset(&three_set) {
                // & 3 = 9
                nine = Some(s.iter().collect());
            } else if zero.is_none() && s.is_superset(&one_set) {
                // & 1 = 0
                zero = Some(s.iter().collect());
            } else if nine.is_some() && zero.is_some() {
                // = 6
                six = Some(s.iter().collect());
            } else {
                v.insert(0, s);
            }
        }

        Self::new(
            zero.unwrap(),
            one.unwrap(),
            two.unwrap(),
            three.unwrap(),
            four.unwrap(),
            five.unwrap(),
            six.unwrap(),
            seven.unwrap(),
            eight.unwrap(),
            nine.unwrap(),
        )
    }
}

impl Default for Segment {
    fn default() -> Self {
        Self::new(
            "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
        )
    }
}

fn to_usize(s: String) -> Option<usize> {
    s.split('|').nth(1).map(|s| {
        s.trim()
            .split(' ')
            .filter_map(|s| Digit::unique(s.trim().len() as u32))
            .count()
    })
}

fn to_usize2(s: String) -> Option<usize> {
    let mut i = s.split('|');
    let segment = Segment::solve(&i.next().unwrap().split(' ').collect::<Vec<&str>>());
    i.next().map(|s| {
        s.trim()
            .split(' ')
            .filter_map(|s| Digit::decode(&segment, s))
            .fold(0usize, |mut acc, v| {
                acc = (acc * 10) + v.value();
                acc
            })
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
    fn check_to_usize() {
        let s =
            "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc";
        assert_eq!(to_usize(s.to_string()), Some(3));
    }

    #[test]
    fn check_to_usize_v2() {
        let s =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        assert_eq!(to_usize2(s.to_string()), Some(5353));
    }

    #[test]
    fn check_solve() {
        let s =
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
        let mut i = s.split('|');
        let segment = Segment::solve(&i.next().unwrap().split(' ').collect::<Vec<&str>>());
        assert_eq!(segment.zero, "abcdeg".to_string());
        assert_eq!(segment.one, "ab".to_string());
        assert_eq!(segment.two, "acdfg".to_string());
        assert_eq!(segment.three, "abcdf".to_string());
        assert_eq!(segment.four, "abef".to_string());
        assert_eq!(segment.five, "bcdef".to_string());
        assert_eq!(segment.six, "bcdefg".to_string());
        assert_eq!(segment.seven, "abd".to_string());
        assert_eq!(segment.eight, "abcdefg".to_string());
        assert_eq!(segment.nine, "abcdef".to_string());
    }

    #[test]
    fn check_decode() {
        let segment = Segment::new(
            "cagedb", "ab", "gcdfa", "fbcad", "eafb", "cdfbe", "cdfgeb", "dab", "acedgfb", "cefabd",
        );
        assert_eq!(Digit::decode(&segment, "aedgfb"), None);
        assert_eq!(Digit::decode(&segment, "cagedb"), Some(Digit::Zero));
        assert_eq!(Digit::decode(&segment, "ba"), Some(Digit::One));
        assert_eq!(Digit::decode(&segment, "gcdfa"), Some(Digit::Two));
        assert_eq!(Digit::decode(&segment, "fbcad"), Some(Digit::Three));
        assert_eq!(Digit::decode(&segment, "eafb"), Some(Digit::Four));
        assert_eq!(Digit::decode(&segment, "cdfbe"), Some(Digit::Five));
        assert_eq!(Digit::decode(&segment, "cdfgeb"), Some(Digit::Six));
        assert_eq!(Digit::decode(&segment, "dab"), Some(Digit::Seven));
        assert_eq!(Digit::decode(&segment, "acedgfb"), Some(Digit::Eight));
        assert_eq!(Digit::decode(&segment, "cefabd"), Some(Digit::Nine));
    }

    #[test]
    fn check_inputs() -> Result<()> {
        let inputs = inputs(to_usize2, from_path("test/test.txt")?);
        assert_eq!(inputs.iter().sum::<usize>(), 61229);

        Ok(())
    }
}
