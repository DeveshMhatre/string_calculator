pub fn add(numbers: String) -> i64 {
    if numbers.len() == 0 {
        return 0;
    }

    let mut input = numbers;

    let mut delimiters: Vec<char> = vec![',', '\n'];

    if let Some(first_line) = input.lines().next() {
        if first_line.starts_with("//") {
            delimiters.clear();
            let delimiter = first_line.replace("//", "");
            for char in delimiter.chars() {
                delimiters.push(char);
            }
            input = input.lines().skip(1).collect();
        }
    };

    let nums: Vec<i64> = input
        .split(|n| delimiters.contains(&n))
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let mut result = 0;
    let mut negative_numbers: Vec<i64> = vec![];

    for num in nums {
        if num < 0 {
            negative_numbers.push(num);
        } else {
            result += num;
        }
    }

    if negative_numbers.len() > 0 {
        let negative_numbers: Vec<String> =
            negative_numbers.iter().map(|n| n.to_string()).collect();
        panic!(
            "negative numbers not allowed {}",
            negative_numbers.join(",")
        )
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
    #[should_panic]
    fn not_a_number() {
        let _result = add(String::from("1,2,34,5,Aj"));
    }

    #[test]
    fn comma_separated_digits() {
        let result = add(String::from("1,2,3,12,21"));
        assert_eq!(result, 39);
    }

    #[test]
    #[should_panic(expected = "negative numbers not allowed")]
    fn negative_number() {
        let _result = add(String::from("23,-1,50,-20,5"));
    }

    #[test]
    fn comma_and_newline_separated_digits() {
        let result = add(String::from("1\n2,3"));
        assert_eq!(result, 6);
    }

    #[test]
    #[should_panic]
    fn newline_as_input() {
        let _result = add(String::from("1,\n"));
    }

    #[test]
    fn specify_delimiter_at_start() {
        let result = add(String::from("//;\n1;2"));
        assert_eq!(result, 3);
    }
}
