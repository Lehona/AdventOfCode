#[macro_use]
extern crate nom;

use nom::types::CompleteStr;

fn main() {
    let input = include_str!("../input.txt");

    let input: Vec<_> = input.lines().map(|l|parse_claim(CompleteStr(l)).unwrap().1).collect();

    let (x, y) = ClaimsField::compute_max_size(input.iter());
    let mut field = ClaimsField::with_sizes(x, y);

    for claim in &input {
        field.add_claim(*claim);
    }

    println!("Number of overlapping claims: {}", field.overlapping());

    for claim in input {
        if field.is_unique(claim) {
            println!("The ID of the unique claim is: {}", claim.nr);
        }
    }

}

struct ClaimsField {
    claims: Vec<Vec<usize>>,
    x: usize,
    y: usize,
}

impl ClaimsField {
    fn with_sizes(x: usize, y: usize) -> Self {
        ClaimsField {
            x,
            y,
            claims: vec![vec![0; y]; x],
        }
    }

    fn set_claim(&mut self, x: usize, y: usize) {
        if x >= self.x || y >= self.y {
            panic!("Out of bounds access");
        }

        self.claims[x][y] += 1;
    }

    pub fn add_claim(&mut self, claim: Claim) {
        for x in claim.x_offset..claim.x_offset+claim.x_size {
            for y in claim.y_offset..claim.y_offset+claim.y_size {
                self.set_claim(x, y);
            }
        }
    }


    pub fn compute_max_size<'a, I: Iterator<Item=&'a Claim>>(input: I) -> (usize, usize) {
        let sizes: Vec<_> = input.map(|claim| {
            let x_end = claim.x_offset + claim.x_size;
            let y_end = claim.y_offset + claim.y_size;
            (x_end, y_end)
        }).collect();

        (sizes.iter().map(|(x, y)|*x).max().unwrap(),
        sizes.iter().map(|(x, y)|*y).max().unwrap())
    }

    pub fn overlapping(&self) -> usize {
        let mut counter = 0;
        for inner_vec in &self.claims {
            for nr_of_claims in inner_vec {
                if *nr_of_claims > 1 {
                    counter += 1;
                }
            }
        };

        counter
    }

    pub fn is_unique(&self, claim: Claim) -> bool {
        for x in claim.x_offset..claim.x_offset+claim.x_size {
            for y in claim.y_offset..claim.y_offset+claim.y_size {
                if self.claims[x][y] != 1 {
                    return false;
                }
            }
        };
        true
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Claim {
    pub nr: usize,
    pub x_offset: usize,
    pub y_offset: usize,
    pub x_size: usize,
    pub y_size: usize,
}

use nom::digit1;
named!(parse_claim<CompleteStr, Claim>, ws!(do_parse!(
    char!('#') >>
    nr: digit1 >>
    char!('@') >>
    x_offset: digit1 >>
    char!(',') >>
    y_offset: digit1 >>
    char!(':') >>
    x_size: digit1 >>
    char!('x') >>
    y_size: digit1 >>
    (Claim {
        nr: nr.parse().unwrap(),
        x_offset: x_offset.parse().unwrap(),
        y_offset: y_offset.parse().unwrap(),
        x_size: x_size.parse().unwrap(),
        y_size: y_size.parse().unwrap()
    })
)));
