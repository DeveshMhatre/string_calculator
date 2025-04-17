pub fn add(numbers: String) -> i64 {
    if numbers.len() == 0 {
        return 0;
    }

    let nums: Vec<i64> = numbers
        .split(",")
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let mut result = 0;

    for num in nums {
        result += num;
    }

    result
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

    #[test]
    #[ignore]
    #[should_panic]
    fn not_a_number() {
        let _result = add(String::from("Aj"));
    }

    #[test]
    fn comma_separated_digits() {
        let result = add(String::from("1,2,3,12,21"));
        assert_eq!(result, 39);
    }
}
