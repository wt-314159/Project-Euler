use std::collections::HashSet;

const A_VALUE: u8 = 'A' as u8;

fn main() {
    let words = include_str!("../0042_words.txt");
    // assume max word length is 20, in which case if all
    // characters are a 'Y', max word value is 20 * 26
    // (overestimate to make sure we don't miss any)
    let triangle_nums = get_triangle_numbers(20 * 26);
    let count = words
        .split(",")
        .map(|w| get_word_value(w.trim_matches('"')))
        .filter(|v| triangle_nums.contains(v))
        .count();
    println!("triangle words count: {count}");
}

fn get_triangle_numbers(max: usize) -> HashSet<usize> {
    let mut triangle_nums = HashSet::new();
    for i in 1.. {
        // t_n = 1/2 * n(n + 1)
        let t = i * (i + 1) / 2;
        if t > max {
            break;
        }
        triangle_nums.insert(t);
    }
    triangle_nums
}

fn get_word_value(word: &str) -> usize {
    word.chars().map(|c| char_value(c) as usize).sum()
}

fn char_value(c: char) -> u8 {
    c as u8 - A_VALUE + 1
}
