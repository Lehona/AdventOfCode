fn main() {
    let input = include_str!("../input.txt");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let result: i64 = input
        .lines()
        .map(|l| l.parse::<i64>().expect("illegal line"))
        .sum();

    println!("The result of puzzle 1 is: {}", result);
}

fn part_two(input: &str) {
    let as_numbers: Vec<_> = input
        .lines()
        .map(|l| l.parse::<i64>().expect("illegal line"))
        .collect();

    let mut previous_freqs = ::std::collections::HashMap::new();

    let mut current = 0;

    loop {
        for i in &as_numbers {
            current += i;

            let occured_previously = previous_freqs.entry(current).or_insert(false);

            if *occured_previously {
                println!("The result of puzzle 2 is: {}", current);
                return;
            } else {
                *occured_previously = true;
            }
        }
    }
}
