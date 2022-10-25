fn _int_to_words(x: i64) -> String {
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

pub fn int_to_words(x: i64) -> String {
    return match x {
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
