#[macro_use]
extern crate nom;
#[macro_use]
extern crate intrusive_collections;

use std::cell::Cell;
use nom::types::CompleteStr;
use intrusive_collections::{LinkedList, linked_list::{Cursor, CursorMut}, LinkedListLink, };

intrusive_adapter!(MarbleAdapter = Box<Marble>: Marble { link: LinkedListLink });
#[derive(Debug)]
struct Marble {
    link: LinkedListLink,
    value: Cell<usize>
}

fn main() {
    let input = include_str!("../input.txt");

    let (players, marbles) = game_params(CompleteStr(input)).unwrap().1;
    part_one(players, marbles);
    part_two(players, marbles);
}

fn part_one(players: usize, marbles: usize) {
    let mut state = LinkedList::new(MarbleAdapter::new());
    let mut game = Game::new(players, marbles, &mut state);

    game.play();

    println!("The winner in game 1 is: {}", game.players.iter().max_by_key(|p|p.0).unwrap().0);
}

fn part_two(players: usize, marbles: usize) {
    let mut state = LinkedList::new(MarbleAdapter::new());
    let mut game = Game::new(players, marbles * 100, &mut state);

    game.play();

    println!("The winner in game two is: {}", game.players.iter().max_by_key(|p| p.0).unwrap().0);
}


#[derive(Debug, Clone, Copy)]
struct Player(usize);



impl Marble {
    fn new(nr: usize) -> Box<Self> {
        Box::new(Marble {
            link: LinkedListLink::new(),
            value: Cell::new(nr)
        })
    }
}

struct Game<'a> {
    players: Vec<Player>,
    marbles: usize,
    next_marble: usize,
    current_marble: CursorMut<'a, MarbleAdapter>
}

use intrusive_collections::{Adapter, linked_list::Link};
fn advance_cursor_mut<A: Adapter<Link=Link>>(cursor: &mut CursorMut<A>, steps: isize) {
    if steps < 0 {
        for _i in steps..0 {
            cursor.move_prev();

            if cursor.is_null() {
                cursor.move_prev();
            }
        }
    } else {
        for _i in 0..steps {
            cursor.move_next();

            if cursor.is_null() {
                cursor.move_next();
            }
        }
    }

}

impl<'a> Game<'a> {
    fn new(players: usize, marbles: usize, list: &'a mut LinkedList<MarbleAdapter>) -> Self {
        list.push_back(Marble::new(0));

        let mut game = Game {
            players: vec![Player(0); players],
            marbles,
            next_marble: 1,
            current_marble: list.cursor_mut()
        };

        game.current_marble.move_next();

        game
    }

    fn play(&mut self) {
        //print_cursor(self.current_marble.as_cursor(), 0);
        while self.next_marble <= self.marbles {
            self.turn();
        }
    }

    fn turn(&mut self) {
        for player in &mut self.players {
            if self.next_marble > self.marbles { break; }

            if self.next_marble % 23 == 0 {
                player.0 += self.next_marble;

                advance_cursor_mut(&mut self.current_marble, -7);
                let removed = self.current_marble.remove().unwrap();

                player.0 += removed.value.get();
            } else {
                let new_marble = Marble::new(self.next_marble);
                advance_cursor_mut(&mut self.current_marble, 1);
                self.current_marble.insert_after(new_marble);
                advance_cursor_mut(&mut self.current_marble, 1);
            }


            //let current_val = self.current_marble.get().unwrap().value.get();
            //print_cursor(self.current_marble.as_cursor(), current_val);

            self.next_marble += 1;
        }
    }
}

#[allow(unused)]
fn print_cursor(mut cursor: Cursor<MarbleAdapter>, current_marble_val: usize) {
    while !cursor.is_null() {
        cursor.move_next();
    }
    cursor.move_next();

    while !cursor.is_null() {
        let cell = &cursor.get().unwrap().value;

        if cell.get() == current_marble_val {
           print!("({}) ", current_marble_val);
        } else {
            print!("{} ", cell.get());
        }

        cursor.move_next();
    }

    println!();

}

use nom::digit1;
named!(game_params<CompleteStr, (usize, usize)>, do_parse!(
    players: digit1 >>
    tag!(" players; last marble is worth ") >>
    marbles: digit1 >>

    ((players.parse().unwrap(), marbles.parse().unwrap()))
));
