use {
    crate::convert::to_vec,
    std::{
        fs::File,
        io::{BufRead, BufReader, Result},
        path::Path,
    },
};

pub fn from_path(path: &str) -> Result<impl BufRead> {
    from_file(Path::new(path))
}

fn from_file(path: &Path) -> Result<impl BufRead> {
    Ok(BufReader::new(File::open(path)?))
}

pub fn inputs<R: BufRead, T>(convert: fn(String) -> Option<T>, reader: R) -> Vec<T> {
    reader
        .lines()
        .filter_map(|result| result.ok())
        .filter_map(convert)
        .collect()
}

pub fn list<R: BufRead, T>(convert: fn(String) -> Option<T>, reader: R) -> Vec<T> {
    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(|s| to_vec(s, ',', convert))
        .flatten()
        .collect()
}

pub fn batch<R: BufRead, T: Clone>(
    convert: fn(String) -> Option<T>,
    batch: fn(&Option<T>) -> bool,
    reader: R,
) -> Vec<Vec<T>> {
    let mut store: Vec<T> = Vec::new();
    reader
        .lines()
        .filter_map(|result| result.ok())
        .map(convert)
        .filter_map(|value| match batch(&value) {
            false if value.is_some() => {
                store.push(value.unwrap());
                None
            }
            false => None,
            true => {
                let result = store.clone();
                store.clear();
                Some(result)
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use {super::*, crate::convert::to_u32, std::io::Cursor};

    #[test]
    fn check_reader() {
        let buffer = Cursor::new(b"100\n200\n300");
        assert_eq!(inputs(to_u32, buffer), vec![100, 200, 300]);
    }

    #[test]
    fn check_file() {
        assert_eq!(
            inputs(to_u32, from_file(Path::new("test/test.txt")).unwrap()),
            vec![101, 201, 301]
        );
    }

    #[test]
    fn check_path() {
        assert_eq!(
            inputs(to_u32, from_path("test/test.txt").unwrap()),
            vec![101, 201, 301]
        );
    }

    #[test]
    fn check_list() {
        let buffer = Cursor::new("100,200,300\n400,500");
        assert_eq!(list(to_u32, buffer), vec![100, 200, 300, 400, 500]);
    }

    #[test]
    fn check_batch() {
        let buffer = Cursor::new("1\n2\n3\n\n4\n5\n6\n\n");
        assert_eq!(
            batch(to_u32, |n| n.is_none(), buffer),
            vec![vec![1, 2, 3], vec![4, 5, 6]]
        );
    }
}
