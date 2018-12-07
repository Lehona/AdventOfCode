#[macro_use]
extern crate nom;
extern crate itertools;

use itertools::Itertools;
use nom::{digit1, types::CompleteStr};
fn main() {
    let input = include_str!("../input.txt");
    let coords: Vec<_> = input
        .lines()
        .map(|l| parse_coordinate(::nom::types::CompleteStr(l)).unwrap().1)
        .collect();

    let x_max = *coords.iter().map(|(x, _y)| x).max().unwrap();
    let y_max = *coords.iter().map(|(_x, y)| y).max().unwrap();

    part_one(&coords, x_max, y_max);
    part_two(&coords, x_max, y_max);

    print_board(&coords, x_max, y_max);
}

fn part_one(coords: &Vec<(isize, isize)>, x_max: isize, y_max: isize) {
    let normal_areas = get_areas(&coords, (0, x_max), (0, y_max));
    let big_areas = get_areas(&coords, (-1, x_max + 1), (-1, y_max + 1));

    let without_infinity: Vec<_> = normal_areas
        .iter()
        .zip(big_areas.iter())
        .enumerate()
        .filter(|(_i, (a, b))| a == b)
        .map(|(_i, (a, _b))| *a)
        .collect::<Vec<_>>();

    println!(
        "The biggest area is: {:?}",
        without_infinity.iter().max().unwrap()
    );
}

fn part_two(coords: &Vec<(isize, isize)>, x_max: isize, y_max: isize) {
    const DIST: usize = 10_000;
    let safety_delta = 100;

    let safe_area_size = (0 - safety_delta..x_max + safety_delta)
        .cartesian_product(0 - safety_delta..y_max + safety_delta)
        .map(|(x, y)| sum_of_distances(coords, x, y))
        .filter(|dist| *dist < DIST)
        .count();

    println!("The size of the safe area is: {}", safe_area_size);
}

fn sum_of_distances(coords: &Vec<(isize, isize)>, x: isize, y: isize) -> usize {
    coords
        .iter()
        .map(|(x_c, y_c)| distance(*x_c, *y_c, x, y))
        .sum()
}

fn get_areas(
    coords: &Vec<(isize, isize)>,
    x_dim: (isize, isize),
    y_dim: (isize, isize),
) -> Vec<usize> {
    let mut closests = vec![0; coords.len()];
    let (x_min, x_max) = x_dim;
    let (y_min, y_max) = y_dim;

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let owner = get_owner(&coords, (x, y));
            match owner {
                Option::None => {}
                Option::Some((nr, _dist)) => closests[nr] += 1,
            }
        }
    }

    closests
}

fn print_board(coords: &Vec<(isize, isize)>, x_max: isize, y_max: isize) {
    println!("Map key (by order of appearance): ");
    for nr in 0..coords.len() {
        println!("{}: {}", nr, display_code(nr));
    }

    for x in 0..=x_max {
        for y in 0..=y_max {
            let found = coords
                .iter()
                .enumerate()
                .find(|(_i, (x_c, y_c))| *x_c == x && *y_c == y);
            match found {
                Option::Some((i, _point)) => print!("{}", display_code(i)),
                Option::None => {
                    let owner = get_owner(coords, (x, y));
                    match owner {
                        Option::None => print!("."),
                        Option::Some((nr, _area)) => print!("{}", display_code(nr)),
                    }
                }
            }
        }
        println!();
    }
}

fn display_code(nr: usize) -> char {
    let nr = nr as u8;
    (match nr {
        0..=25 => b'a' + nr,
        26..=50 => b'A' + (nr - 26),
        _ => b'X',
    }) as char
}

fn get_owner(coords: &Vec<(isize, isize)>, point: (isize, isize)) -> Option<(usize, usize)> {
    let (x, y) = point;
    let owner = coords
        .iter()
        .enumerate()
        .map(|(i, (x_cord, y_cord))| (i, distance(*x_cord, *y_cord, x, y)));

    let owner = unique_min_by(owner, |(_i0, dist0), (_i1, dist1)| dist0.cmp(dist1));

    match owner {
        UniqueMin::None => Option::None,
        UniqueMin::NonUnique(_) => Option::None,
        UniqueMin::Some(val) => Option::Some(val),
    }
}

enum UniqueMin<B> {
    None,
    NonUnique(B),
    Some(B),
}
use std::cmp::Ordering;
fn unique_min_by<I: Iterator<Item = B>, B: Default, F: Fn(&B, &B) -> Ordering>(
    iter: I,
    cmp: F,
) -> UniqueMin<B> {
    iter.fold(UniqueMin::None, |acc, next| match acc {
        UniqueMin::None => UniqueMin::Some(next),
        UniqueMin::NonUnique(min) => match cmp(&min, &next) {
            Ordering::Equal => UniqueMin::NonUnique(min),
            Ordering::Less => UniqueMin::NonUnique(min),
            Ordering::Greater => UniqueMin::Some(next),
        },
        UniqueMin::Some(min) => match cmp(&min, &next) {
            Ordering::Equal => UniqueMin::NonUnique(min),
            Ordering::Less => UniqueMin::Some(min),
            Ordering::Greater => UniqueMin::Some(next),
        },
    })
}

fn distance(x1: isize, y1: isize, x2: isize, y2: isize) -> usize {
    use core::cmp::{max, min};
    let d_x = max(x1, x2) - min(x1, x2);
    let d_y = max(y1, y2) - min(y1, y2);

    (d_x + d_y) as usize
}

named!(parse_coordinate<CompleteStr, (isize, isize)>, do_parse!(
    x: digit1 >>
    tag!(", ") >>
    y: digit1 >>
    ((x.parse().unwrap(), y.parse().unwrap()))
));
