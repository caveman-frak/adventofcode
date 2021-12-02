use {
    common::{convert::to_u32, input::Inputs},
    std::{cmp::Ordering, io::Error, path::Path},
};

fn main() -> Result<(), Error> {
    let depths = Depths::new(
        false,
        Inputs::from_file(to_u32, Path::new("day1/data/input.txt"))?,
    );

    let windows = depths.windows(depths.depths());
    let variances = depths.variances(&windows);
    let count = depths.count(&variances, &Variance::Increased);
    println!("No of increases = {}", count);

    Ok(())
}

#[derive(Debug)]
struct Depths {
    debug: bool,
    depths: Vec<u32>,
}

impl Depths {
    fn new(debug: bool, depths: Vec<u32>) -> Self {
        Depths { debug, depths }
    }

    fn depths(&self) -> &Vec<u32> {
        &self.depths
    }

    fn windows(&self, depths: &Vec<u32>) -> Vec<u32> {
        let mut windows = Vec::new();
        let mut window = Window::new();

        for current in depths {
            window.push(Some(*current));
            if window.is_full() {
                if self.debug {
                    println!("{}", current);
                }
                windows.push(window.sum());
            }
        }

        windows
    }

    fn variances(&self, depths: &Vec<u32>) -> Vec<Variance> {
        let mut variances = Vec::new();
        let mut previous: Option<&u32> = None;

        for current in depths {
            if self.debug {
                println!("{}", current);
            }
            variances.push(Variance::compare(previous, Some(current)));

            previous = Some(current);
        }

        variances
    }

    fn count(&self, variances: &Vec<Variance>, variance: &Variance) -> usize {
        variances
            .iter()
            .inspect(|v| self.debug(v))
            .filter(|v| v == &variance)
            .count()
    }

    fn debug(&self, variance: &&Variance) {
        if self.debug {
            println!("{:?}", variance)
        }
    }
}

#[derive(Debug, PartialEq)]
enum Variance {
    Unchanged,
    Increased,
    Decreased,
    NotApplicable,
}

impl Variance {
    fn compare(first: Option<&u32>, second: Option<&u32>) -> Self {
        match (first, second) {
            (Some(first), Some(second)) => match first.cmp(&second) {
                Ordering::Less => Variance::Increased,
                Ordering::Greater => Variance::Decreased,
                _ => Variance::Unchanged,
            },
            (_, _) => Variance::NotApplicable,
        }
    }
}

#[derive(Debug)]
struct Window {
    a: Option<u32>,
    b: Option<u32>,
    c: Option<u32>,
}

impl Window {
    fn new() -> Self {
        Self {
            a: None,
            b: None,
            c: None,
        }
    }

    fn push(&mut self, item: Option<u32>) {
        self.a = self.b;
        self.b = self.c;
        self.c = item;
    }

    fn sum(&self) -> u32 {
        self.a.unwrap_or(0) + self.b.unwrap_or(0) + self.c.unwrap_or(0)
    }

    fn is_full(&self) -> bool {
        self.a.map_or(false, |_| true)
            && self.b.map_or(false, |_| true)
            && self.c.map_or(false, |_| true)
    }
}

#[cfg(test)]
mod tests {
    use {self::Variance::*, super::*};

    #[test]
    fn check_compare_lt() {
        assert_eq!(Variance::compare(Some(&10), Some(&20)), Increased);
    }

    #[test]
    fn check_compare_gt() {
        assert_eq!(Variance::compare(Some(&20), Some(&10)), Decreased);
    }

    #[test]
    fn check_compare_eq() {
        assert_eq!(Variance::compare(Some(&10), Some(&10)), Unchanged);
    }

    #[test]
    fn check_compare_first() {
        assert_eq!(Variance::compare(None, Some(&10)), NotApplicable);
    }

    #[test]
    fn check_compare_last() {
        assert_eq!(Variance::compare(Some(&10), None), NotApplicable);
    }

    #[test]
    fn check_variance() {
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(
            depths.variances(depths.depths()),
            vec![
                NotApplicable,
                Increased,
                Increased,
                Increased,
                Decreased,
                Increased,
                Increased,
                Increased,
                Decreased,
                Increased
            ]
        );
    }

    #[test]
    fn check_count() {
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        let variances = depths.variances(depths.depths());

        assert_eq!(depths.count(&variances, &Increased), 7);
    }

    #[test]
    fn check_windows() {
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);

        assert_eq!(
            depths.windows(depths.depths()),
            vec![607, 618, 618, 617, 647, 716, 769, 792]
        )
    }

    #[test]
    fn check_window_variances() {
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        let windows = depths.windows(depths.depths());

        assert_eq!(
            depths.variances(&windows),
            vec![
                NotApplicable,
                Increased,
                Unchanged,
                Decreased,
                Increased,
                Increased,
                Increased,
                Increased,
            ]
        )
    }

    #[test]
    fn check_window_count() {
        let depths = Depths::new(true, vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        let windows = depths.windows(depths.depths());
        let variances = depths.variances(&windows);

        assert_eq!(depths.count(&variances, &Increased), 5)
    }
}
