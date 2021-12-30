pub fn to_u32(s: String) -> Option<u32> {
    s.parse().ok()
}

pub fn to_vec_u32(s: String) -> Option<Vec<u32>> {
    Some(s.chars().filter_map(|c| c.to_digit(10)).collect())
}

pub fn to_vec<T>(s: String, pattern: char, convert: fn(String) -> Option<T>) -> Vec<T> {
    s.split(pattern)
        .filter_map(|s| convert(s.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_u32_success() {
        assert_eq!(to_u32("100".to_string()), Some(100));
    }

    #[test]
    fn check_u32_failure() {
        assert_eq!(to_u32("ABC".to_string()), None);
    }

    #[test]
    fn check_vec_u32() {
        assert_eq!(to_vec("2,4,6".to_string(), ',', to_u32), vec![2, 4, 6]);
    }

    #[test]
    fn check_vec() {
        assert_eq!(to_vec_u32("246".to_string()), Some(vec![2, 4, 6]));
    }
}
