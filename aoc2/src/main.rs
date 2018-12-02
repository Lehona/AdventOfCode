use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    part_one(input);
    part_two(input);
}

fn part_one(input: &str) {
    let mut two_count = 0;
    let mut three_count = 0;

    for line in input.lines() {
        let counts = get_letter_counts(line);
        if has_letter_count(&counts, 3) {
            three_count += 1;
        };
        if has_letter_count(&counts, 2) {
            two_count += 1;
        }
    }

    println!(
        "{} words with double letters, {} words with triple letters makes checksum = {}",
        two_count,
        three_count,
        three_count * two_count
    );
}

fn part_two(input: &str) {
    for a in input.lines() {
        for b in input.lines() {
            if letter_delta(a, b) == 1 {
                println!("{} and {}", a, b);
            }
        }
    }
}

fn letter_delta(a: &str, b: &str) -> usize {
    a.chars()
        .zip(b.chars())
        .map(|(a, b)| (a != b) as usize)
        .sum()
}

fn has_letter_count(counts: &HashMap<char, usize>, needle: usize) -> bool {
    for (_, value) in counts.iter() {
        if *value == needle {
            return true;
        }
    }

    false
}

fn get_letter_counts(word: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();

    for c in word.chars() {
        let entry = map.entry(c).or_insert(0);
        *entry += 1;
    }

    map
}
