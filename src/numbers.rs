fn _int_to_words(x: usize) -> String {
    let mut result: Vec<String> = vec![];
    let thousands = x / 1000;
    if thousands > 0 {
        result.push(format!("{} thousand", int_to_words(thousands)));
    }

    let hundreds = (x % 1000) / 100;
    if hundreds > 0 {
        if thousands > 0 {
            result.push(String::from(" "));
        }
        result.push(format!("{} hundred", int_to_words(hundreds)));
    }

    if (thousands > 0 && x % 1000 > 0) || (hundreds > 0 && x % 100 > 0) {
        result.push(String::from(" and "));
    }

    if x % 100 > 19 {
        let tens = (x % 100) / 10;
        result.push(int_to_words(tens * 10));

        let units = x % 10;
        if units > 0 {
            result.push(String::from("-"));
            result.push(int_to_words(units));
        }
    } else {
        let units = x % 100;
        if units > 0 {
            result.push(int_to_words(units));
        }
    }

    return result.join("");
}

pub fn int_to_words(x: usize) -> String {
    return match x {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        30 => String::from("thirty"),
        40 => String::from("forty"),
        50 => String::from("fifty"),
        60 => String::from("sixty"),
        70 => String::from("seventy"),
        80 => String::from("eighty"),
        90 => String::from("ninety"),
        other => _int_to_words(other),
    };
}

#[cfg(test)]
mod tests {
    use super::int_to_words;
    use test_case::test_case;

    #[test_case(0, "zero" ; "zero")]
    #[test_case(1, "one" ; "one")]
    #[test_case(2, "two" ; "two")]
    #[test_case(3, "three" ; "three")]
    #[test_case(10, "ten" ; "ten")]
    #[test_case(11, "eleven" ; "eleven")]
    #[test_case(15, "fifteen" ; "fifteen")]
    #[test_case(20, "twenty" ; "twenty")]
    #[test_case(21, "twenty-one" ; "twenty-one")]
    #[test_case(25, "twenty-five" ; "twenty-five")]
    #[test_case(50, "fifty" ; "fifty")]
    #[test_case(100, "one hundred" ; "one hundred")]
    #[test_case(167, "one hundred and sixty-seven" ; "one hundred and sixty-seven")]
    #[test_case(67003, "sixty-seven thousand and three" ; "sixty-seven thousand and three")]
    #[test_case(67499, "sixty-seven thousand four hundred and ninety-nine" ; "sixty-seven thousand four hundred and ninety-nine")]
    fn test_int_to_words(i: usize, expected: &str) {
        assert_eq!(int_to_words(i), expected)
    }
}
