pub fn to_u32(s: String) -> Option<u32> {
    s.parse().ok()
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
}
