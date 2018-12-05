fn main() {
    let input = include_bytes!("../input.txt");

    let reacted = fully_react(input.clone().to_vec());
    println!(
        "The length of the polymer after fully reacting: {}",
        reacted.len()
    );

    let len = (b'a'..=b'z')
        .map(|b| fully_react(clean(input.clone().to_vec(), b)).len())
        .min()
        .unwrap();


    println!("The shorted possible polymer is of length: {}", len);
}

fn clean(mut polymers: Vec<u8>, b: u8) -> Vec<u8> {
    let lower = b.to_ascii_lowercase();
    let upper = b.to_ascii_uppercase();
    polymers.retain(|polymer| *polymer != upper && *polymer != lower);
    polymers
}

fn fully_react(polymers: Vec<u8>) -> Vec<u8> {
    let mut length = 0;
    let mut folded: Vec<_> = polymers.into_iter().fold(Vec::new(), fold_polymers);
    while length != folded.len() {
        length = folded.len();
        folded = folded.into_iter().fold(Vec::new(), fold_polymers);
    }

    folded
}

fn fold_polymers(mut acc: Vec<u8>, next: u8) -> Vec<u8> {
    if let Some(last) = acc.pop() {
        if !can_react(last, next) {
            acc.push(last);
            acc.push(next);
        }
    } else {
        acc.push(next);
    }
    acc
}

fn can_react(a: u8, b: u8) -> bool {
    if is_lowercase(a) {
        return a.to_ascii_uppercase() == b;
    } else {
        return a.to_ascii_lowercase() == b;
    }
}

fn is_lowercase(a: u8) -> bool {
    if a >= b'a' && a <= b'z' {
        return true;
    };

    false
}
