const ASCII_ALPHABET_START: u32 = ('A' as u32) - 1;

fn main() {
    let input = include_str!("../names.txt");
    let mut names = parse_input(input);
    names.sort();
    let score = names.iter().enumerate().fold(0u32, |acc, (i, n)| {
        acc.checked_add(score_name(n, (i as u32) + 1))
            .expect("Overflowed u32")
    });
    println!("Total score: {score}");
}

fn score_name(name: &str, position: u32) -> u32 {
    name.chars()
        .map(|c| (c as u32) - ASCII_ALPHABET_START)
        .sum::<u32>()
        * position
}

fn parse_input(input: &str) -> Vec<&str> {
    let mut names = Vec::new();
    for name in input.split(",") {
        let name = name.trim_matches('"');
        names.push(name);
    }
    names
}
