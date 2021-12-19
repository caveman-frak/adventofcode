use {
    anyhow::{Error, Result},
    common::input::{from_path, inputs},
    std::{
        convert::TryFrom,
        fmt::{self, Binary, Formatter, Write},
        num::ParseIntError,
        ops::AddAssign,
        result,
        str::FromStr,
    },
};

fn main() -> Result<()> {
    let inputs = inputs(to_diagnostic, from_path("day3/data/input.txt")?);

    part1(&inputs)?;
    part2(&inputs)?;

    Ok(())
}

fn part1(inputs: &[Diagnostic]) -> Result<()> {
    let summary = Summary::summarize(inputs);

    let gamma: u32 = summary.gamma().try_into()?;
    let epsilon: u32 = summary.epsilon().try_into()?;

    println!(" Day 3 Part 1 => {}", gamma * epsilon);

    Ok(())
}

fn part2(inputs: &[Diagnostic]) -> Result<()> {
    let rating_high = Summary::rating(inputs, vec![], true);
    let rating_low = Summary::rating(inputs, vec![], false);

    let oxygen: u32 = rating_high.try_into()?;
    let co2: u32 = rating_low.try_into()?;

    println!("Day 3 Part 2 => {}", oxygen * co2);

    Ok(())
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Diagnostic {
    values: Vec<bool>,
}

impl Diagnostic {
    fn new(values: &[bool]) -> Self {
        let mut v = Vec::new();
        v.resize(values.len(), false);
        v.clone_from_slice(values);
        Self { values: v }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn flip(mut self) -> Self {
        for (i, value) in self.values.clone().iter().enumerate() {
            self.values[i] = !value;
        }

        self
    }

    fn matches(&self, criteria: &[bool]) -> bool {
        for (i, value) in criteria.iter().enumerate() {
            if &self.values[i] != value {
                return false;
            }
        }
        true
    }
}

impl Binary for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.values
            .iter()
            .map(|v| match v {
                true => '1',
                false => '0',
            })
            .for_each(|v| f.write_char(v).unwrap());
        Ok(())
    }
}

impl TryFrom<String> for Diagnostic {
    type Error = Error;

    fn try_from(s: String) -> Result<Self> {
        Diagnostic::try_from(&*s)
    }
}

impl TryFrom<&str> for Diagnostic {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self> {
        let v: Vec<bool> = s.chars().map(|c| c == '1').collect();
        Ok(Diagnostic::new(&v))
    }
}

impl FromStr for Diagnostic {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Diagnostic::try_from(s)
    }
}

impl TryFrom<Diagnostic> for u32 {
    type Error = ParseIntError;

    fn try_from(d: Diagnostic) -> result::Result<Self, Self::Error> {
        u32::try_from(&d)
    }
}

impl TryFrom<&Diagnostic> for u32 {
    type Error = ParseIntError;

    fn try_from(d: &Diagnostic) -> result::Result<Self, Self::Error> {
        u32::from_str_radix(&*format!("{:b}", d), 2)
    }
}

fn to_diagnostic(s: String) -> Option<Diagnostic> {
    s.parse().ok()
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct Summary {
    count: u32,
    totals: Vec<u32>,
}

impl Summary {
    fn new() -> Self {
        Self {
            count: 0,
            totals: Vec::new(),
        }
    }

    fn summarize(inputs: &[Diagnostic]) -> Self {
        inputs.iter().fold(Summary::new(), |mut a, d| {
            a += d;
            a
        })
    }

    fn rating(inputs: &[Diagnostic], mut criteria: Vec<bool>, high: bool) -> Diagnostic {
        let position = criteria.len();
        let summary = Self::summarize(inputs);
        if summary.count == 1 {
            summary.gamma()
        } else {
            let diagnostic = match high {
                true => summary.gamma(),
                false => summary.epsilon(),
            };
            criteria.push(diagnostic.values[position]);
            let filtered: Vec<Diagnostic> = inputs
                .iter()
                .cloned()
                .filter(|v| v.matches(&criteria))
                .collect();

            Self::rating(&filtered, criteria, high)
        }
    }

    fn init(&mut self, diagnostic: &Diagnostic) {
        self.count = 0;
        self.totals.resize(diagnostic.len(), 0);
    }

    fn gamma(&self) -> Diagnostic {
        let mut gamma = Vec::new();
        gamma.resize(self.totals.len(), false);
        let half = (self.count + 1) / 2;

        for (i, total) in self.totals.iter().enumerate() {
            if *total >= half {
                gamma[i] = true;
            }
        }

        Diagnostic::new(&gamma)
    }

    fn epsilon(&self) -> Diagnostic {
        self.gamma().flip()
    }
}

impl AddAssign<&Diagnostic> for Summary {
    fn add_assign(&mut self, diagnostic: &Diagnostic) {
        if self.totals.is_empty() {
            self.init(diagnostic);
        }

        self.count += 1;

        for (i, value) in diagnostic.values.iter().enumerate() {
            if *value {
                self.totals[i] += 1;
            }
        }
    }
}

impl AddAssign<Diagnostic> for Summary {
    fn add_assign(&mut self, diagnostic: Diagnostic) {
        self.add_assign(&diagnostic)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_add_assign() {
        let diagnostic = Diagnostic::new(&vec![true, false, false, true, false]);
        let mut summary = Summary::new();
        summary += diagnostic;
        assert_eq!(summary.count, 1);
        assert_eq!(summary.totals, vec![1, 0, 0, 1, 0]);
    }

    #[test]
    fn check_gamma() {
        let diagnostic = Diagnostic::new(&vec![true, false, false, true, false]);
        let mut summary = Summary::new();
        summary += diagnostic;
        assert_eq!(
            summary.gamma().values,
            vec![true, false, false, true, false]
        );
        assert_eq!(
            summary.epsilon().values,
            vec![false, true, true, false, true]
        );
    }

    #[test]
    fn check_diagnostics() -> Result<()> {
        let summary = Summary::summarize(&vec![
            Diagnostic::new(&vec![false, false, true, false, false]),
            Diagnostic::new(&vec![true, true, true, true, false]),
            Diagnostic::new(&vec![true, false, true, true, false]),
            Diagnostic::new(&vec![true, false, true, true, true]),
            Diagnostic::new(&vec![true, false, true, false, true]),
            Diagnostic::new(&vec![false, true, true, true, true]),
            Diagnostic::new(&vec![false, false, true, true, true]),
            Diagnostic::new(&vec![true, true, true, false, false]),
            Diagnostic::new(&vec![true, false, false, false, false]),
            Diagnostic::new(&vec![true, true, false, false, true]),
            Diagnostic::new(&vec![false, false, false, true, false]),
            Diagnostic::new(&vec![false, true, false, true, false]),
        ]);

        let gamma = &summary.gamma();
        let epsilon = &summary.epsilon();
        assert_eq!(gamma.values, vec![true, false, true, true, false]);
        assert_eq!(epsilon.values, vec![false, true, false, false, true]);
        assert_eq!(u32::try_from(gamma)?, 22);
        assert_eq!(u32::try_from(epsilon)?, 9);
        assert_eq!(u32::try_from(gamma)? * u32::try_from(epsilon)?, 198);

        Ok(())
    }

    #[test]
    fn check_parse() -> Result<()> {
        let d: Diagnostic = "01001".parse()?;

        assert_eq!(d.values, vec![false, true, false, false, true]);

        Ok(())
    }

    #[test]
    fn check_binary() -> Result<()> {
        let d: Diagnostic = "01001".parse()?;

        assert_eq!(format!("{:b}", d), "01001");

        Ok(())
    }

    #[test]
    fn check_u32() -> Result<()> {
        let d: Diagnostic = "01001".parse()?;
        let i: u32 = d.try_into()?;

        assert_eq!(i, 9);

        Ok(())
    }

    #[test]
    fn check_matches() {
        let d: Diagnostic = "01001".parse().unwrap();

        assert_eq!(d.matches(&vec![]), true);
        assert_eq!(d.matches(&vec![false]), true);
        assert_eq!(d.matches(&vec![true]), false);
        assert_eq!(d.matches(&vec![false, true]), true);
        assert_eq!(d.matches(&vec![false, false]), false);
    }

    #[test]
    fn check_rating_high() {
        let inputs = vec![
            Diagnostic::new(&vec![false, false, true, false, false]),
            Diagnostic::new(&vec![true, true, true, true, false]),
            Diagnostic::new(&vec![true, false, true, true, false]),
            Diagnostic::new(&vec![true, false, true, true, true]),
            Diagnostic::new(&vec![true, false, true, false, true]),
            Diagnostic::new(&vec![false, true, true, true, true]),
            Diagnostic::new(&vec![false, false, true, true, true]),
            Diagnostic::new(&vec![true, true, true, false, false]),
            Diagnostic::new(&vec![true, false, false, false, false]),
            Diagnostic::new(&vec![true, true, false, false, true]),
            Diagnostic::new(&vec![false, false, false, true, false]),
            Diagnostic::new(&vec![false, true, false, true, false]),
        ];
        let rating = Summary::rating(&inputs, vec![], true);

        assert_eq!(rating.values, vec![true, false, true, true, true]);
    }

    #[test]
    fn check_rating_low() {
        let inputs = vec![
            Diagnostic::new(&vec![false, false, true, false, false]),
            Diagnostic::new(&vec![true, true, true, true, false]),
            Diagnostic::new(&vec![true, false, true, true, false]),
            Diagnostic::new(&vec![true, false, true, true, true]),
            Diagnostic::new(&vec![true, false, true, false, true]),
            Diagnostic::new(&vec![false, true, true, true, true]),
            Diagnostic::new(&vec![false, false, true, true, true]),
            Diagnostic::new(&vec![true, true, true, false, false]),
            Diagnostic::new(&vec![true, false, false, false, false]),
            Diagnostic::new(&vec![true, true, false, false, true]),
            Diagnostic::new(&vec![false, false, false, true, false]),
            Diagnostic::new(&vec![false, true, false, true, false]),
        ];
        let rating = Summary::rating(&inputs, vec![], false);

        assert_eq!(rating.values, vec![false, true, false, true, false]);
    }
}
