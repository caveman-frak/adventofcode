use {
    anyhow::{Error, Result},
    common::input::Inputs,
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
    let inputs = Inputs::from_path(to_diagnostic, "day3/data/input.txt")?;

    let diagnostics = inputs.iter().fold(Diagnostics::new(), |mut a, d| {
        a += d;
        a
    });

    let gamma: u32 = diagnostics.gamma().try_into()?;
    let epsilon: u32 = diagnostics.epsilon().try_into()?;

    println!("{}", gamma * epsilon);

    Ok(())
}

#[derive(Debug)]
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
        u32::from_str_radix(&*format!("{:b}", d), 2)
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

#[derive(Debug)]
struct Diagnostics {
    count: u32,
    totals: Vec<u32>,
}

impl Diagnostics {
    fn new() -> Self {
        Self {
            count: 0,
            totals: Vec::new(),
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

impl AddAssign<&Diagnostic> for Diagnostics {
    fn add_assign(&mut self, diagnostic: &Diagnostic) {
        if self.totals.len() == 0 {
            self.init(&diagnostic);
        }

        self.count += 1;

        for (i, value) in diagnostic.values.iter().enumerate() {
            if *value {
                self.totals[i] += 1;
            }
        }
    }
}

impl AddAssign<Diagnostic> for Diagnostics {
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
        let mut diagnostics = Diagnostics::new();
        diagnostics += diagnostic;
        assert_eq!(diagnostics.count, 1);
        assert_eq!(diagnostics.totals, vec![1, 0, 0, 1, 0]);
    }

    #[test]
    fn check_gamma() {
        let diagnostic = Diagnostic::new(&vec![true, false, false, true, false]);
        let mut diagnostics = Diagnostics::new();
        diagnostics += diagnostic;
        assert_eq!(
            diagnostics.gamma().values,
            vec![true, false, false, true, false]
        );
        assert_eq!(
            diagnostics.epsilon().values,
            vec![false, true, true, false, true]
        );
    }

    #[test]
    fn check_diagnostics() -> Result<()> {
        let mut diagnostics = Diagnostics::new();
        diagnostics += Diagnostic::new(&vec![false, false, true, false, false]);
        diagnostics += Diagnostic::new(&vec![true, true, true, true, false]);
        diagnostics += Diagnostic::new(&vec![true, false, true, true, false]);

        diagnostics += Diagnostic::new(&vec![true, false, true, true, true]);
        diagnostics += Diagnostic::new(&vec![true, false, true, false, true]);
        diagnostics += Diagnostic::new(&vec![false, true, true, true, true]);

        diagnostics += Diagnostic::new(&vec![false, false, true, true, true]);
        diagnostics += Diagnostic::new(&vec![true, true, true, false, false]);
        diagnostics += Diagnostic::new(&vec![true, false, false, false, false]);

        diagnostics += Diagnostic::new(&vec![true, true, false, false, true]);
        diagnostics += Diagnostic::new(&vec![false, false, false, true, false]);
        diagnostics += Diagnostic::new(&vec![false, true, false, true, false]);

        let gamma = &diagnostics.gamma();
        let epsilon = &diagnostics.epsilon();
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
}
