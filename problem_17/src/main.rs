fn main() {
    let count: usize = (1..=1000).map(|i| get_letter_count(i)).sum();
    println!("Total count: {count}");
}

fn get_letter_count(num: usize) -> usize {
    if num == 0 {
        return 0;
    }
    if num < 20 {
        return match num {
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            10 => "ten",
            11 => "eleven",
            12 => "twelve",
            13 => "thirteen",
            14 => "fourteen",
            15 => "fifteen",
            16 => "sixteen",
            17 => "seventeen",
            18 => "eighteen",
            19 => "nineteen",
            _ => panic!("Already checked n < 10 and n < 20"),
        }
        .len();
    } else if num < 100 {
        let ones = num % 10;
        let tens = num / 10;
        return get_letter_count(ones)
            + match tens {
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                _ => panic!("Already checked n >= 20 and n < 100"),
            }
            .len();
    } else if num < 1000 {
        let tens = num % 100;
        let hundreds = num / 100;
        // after dividing by a hundred, we can count the hundreds digit in the same way
        // as ones digits (e.g. "one".len for 100 etc), and just add the length of "hundred and"
        // (ignoring the space). If there is no remainder after dividing by 100, don't use "and".
        let and_len = if tens == 0 { 0 } else { "and".len() };
        return get_letter_count(tens) + get_letter_count(hundreds) + "hundred".len() + and_len;
    } else {
        let hundreds = num % 1000;
        let thousands = num / 1000;
        println!("thousands: {thousands}");
        return get_letter_count(hundreds) + get_letter_count(thousands) + "thousand".len();
    }
}
