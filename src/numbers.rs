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

    #[test]
    fn zero() {
        let items: String = int_to_words(0);
        assert_eq!(items, "zero")
    }

    #[test]
    fn one() {
        let items: String = int_to_words(1);
        assert_eq!(items, "one")
    }

    #[test]
    fn two() {
        let items: String = int_to_words(2);
        assert_eq!(items, "two")
    }

    #[test]
    fn three() {
        let items: String = int_to_words(3);
        assert_eq!(items, "three")
    }

    #[test]
    fn ten() {
        let items: String = int_to_words(10);
        assert_eq!(items, "ten")
    }

    #[test]
    fn eleven() {
        let items: String = int_to_words(11);
        assert_eq!(items, "eleven")
    }

    #[test]
    fn fifteen() {
        let items: String = int_to_words(15);
        assert_eq!(items, "fifteen")
    }

    #[test]
    fn twenty() {
        let items: String = int_to_words(20);
        assert_eq!(items, "twenty")
    }

    #[test]
    fn twenty_one() {
        let items: String = int_to_words(21);
        assert_eq!(items, "twenty-one")
    }

    #[test]
    fn twenty_five() {
        let items: String = int_to_words(25);
        assert_eq!(items, "twenty-five")
    }

    #[test]
    fn fifty() {
        let items: String = int_to_words(50);
        assert_eq!(items, "fifty")
    }

    #[test]
    fn one_hundred() {
        let items: String = int_to_words(100);
        assert_eq!(items, "one hundred")
    }

    #[test]
    fn one_hundred_and_six() {
        let items: String = int_to_words(106);
        assert_eq!(items, "one hundred and six")
    }

    #[test]
    fn one_hundred_sixty_seven() {
        let items: String = int_to_words(167);
        assert_eq!(items, "one hundred and sixty-seven")
    }

    #[test]
    fn sixty_seven_thousand_and_three() {
        let items: String = int_to_words(67003);
        assert_eq!(items, "sixty-seven thousand and three")
    }

    #[test]
    fn sixty_seven_thousand_four_hundred_and_ninety_nine() {
        let items: String = int_to_words(67499);
        assert_eq!(items, "sixty-seven thousand four hundred and ninety-nine")
    }
}
