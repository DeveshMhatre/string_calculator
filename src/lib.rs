pub fn add(_numbers: String) -> i64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let result = add(String::from(""));
        assert_eq!(result, 0);
    }
}
