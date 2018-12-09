#[macro_use]
extern crate nom;

use nom::types::CompleteStr;

fn main() {
    let input = include_str!("../input.txt");

    let nodes = parse_node(CompleteStr(input)).unwrap().1;

    println!("The checksum (metadata-sum) is {}", nodes.sum_of_metadata());
    println!("The checksum (indexed-sum) is {}", nodes.indexed_checksum());
}

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn sum_of_metadata(&self) -> usize {
        let of_children = self
            .children
            .iter()
            .map(|c| c.sum_of_metadata())
            .sum::<usize>();

        self.metadata.iter().sum::<usize>() + of_children
    }

    fn indexed_checksum(&self) -> usize {
        match self.children.len() {
            0 => self.sum_of_metadata(),
            _ => self
                .metadata
                .iter()
                .filter_map(|m| self.children.get(*m - 1).map(|c| c.indexed_checksum()))
                .sum::<usize>(),
        }
    }
}

use nom::digit1;

named!(p_usize<CompleteStr, usize>,
    map_res!(digit1, |d: CompleteStr| d.parse::<usize>())
);

named!(parse_node<CompleteStr, Node>, ws!(do_parse!(
    nr_of_children: p_usize >>
    nr_of_metadata: p_usize >>
    children: many_m_n!(nr_of_children, nr_of_children, parse_node) >>
    metadata: many_m_n!(nr_of_metadata, nr_of_metadata, ws!(p_usize)) >>

    (Node { children, metadata })

)));
