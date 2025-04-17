pub fn add(numbers: String) -> i64 {
    if numbers.len() == 0 {
        return 0;
    }

    let num = numbers.parse::<i64>().unwrap();
    num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        let result = add(String::from(""));
        assert_eq!(result, 0);
    }

    #[test]
    fn single_digit() {
        let result = add(String::from("12"));
        assert_eq!(result, 12);
    }
}
