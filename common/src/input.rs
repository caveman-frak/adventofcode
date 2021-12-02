use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    path::Path,
};

pub struct Inputs {}

#[allow(dead_code)]
impl Inputs {
    pub fn from_file<T>(convert: fn(String) -> Option<T>, path: &Path) -> Result<Vec<T>, Error> {
        Ok(Inputs::inputs(convert, BufReader::new(File::open(path)?)))
    }

    fn inputs<R: BufRead, T>(convert: fn(String) -> Option<T>, reader: R) -> Vec<T> {
        reader
            .lines()
            .filter_map(|result| result.ok())
            .filter_map(convert)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use {super::*, crate::convert::to_u32, std::io::Cursor};

    #[test]
    fn check_reader() {
        let buffer = Cursor::new(b"100\n200\n300");
        assert_eq!(Inputs::inputs(to_u32, buffer), vec![100, 200, 300]);
    }

    #[test]
    fn check_file() {
        assert_eq!(
            Inputs::from_file(to_u32, Path::new("test/test.txt")).unwrap(),
            vec![101, 201, 301]
        );
    }
}
