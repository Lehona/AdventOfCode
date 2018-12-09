#![feature(drain_filter)]

#[macro_use]
extern crate nom;
extern crate itertools;

use itertools::Itertools;
use nom::types::CompleteByteSlice;
use std::collections::{HashMap, HashSet};

type Step = u8;
type Requires = u8;

fn main() {
    let input = include_bytes!("../input.txt");

    let input = parse_input(CompleteByteSlice(input)).unwrap().1;

    let mut steps = HashSet::new();
    let mut grouped_requirements = HashMap::new();

    for requirement in &input {
        steps.insert(requirement.step);
        steps.insert(requirement.requires);

        grouped_requirements
            .entry(requirement.step)
            .or_insert(Vec::new());
        grouped_requirements
            .entry(requirement.requires)
            .or_insert(Vec::new());
    }

    for (step, requirements) in input.into_iter().group_by(|r| r.step).into_iter() {
        grouped_requirements
            .entry(step)
            .or_insert(Vec::new())
            .extend(requirements.map(|r| r.requires));
    }

    part_one(&steps, &grouped_requirements);
    part_two(&steps, &grouped_requirements);
}

fn part_one(steps: &HashSet<Step>, requirements: &HashMap<Step, Vec<Requires>>) {
    let mut completed: Vec<Step> = Vec::new();
    let mut uncompleted: Vec<Step> = Vec::new();
    uncompleted.extend(steps);

    while !uncompleted.is_empty() {
        let mut available_this_turn: Vec<_> =
            available_steps(&uncompleted, &completed, &requirements);

        available_this_turn.sort_unstable();

        let chosen_step = match available_this_turn.first() {
            None => break,
            Some(step) => *step,
        };

        completed.push(chosen_step);

        let index = uncompleted.iter().position(|x| *x == chosen_step).unwrap();
        uncompleted.remove(index);
    }

    print_steps(&completed);
}

// Part two

const MAX_WORKERS: usize = 5;

struct Workers {
    seconds: usize,
    workers: Vec<(Step, usize)>,
}

impl Workers {
    fn new() -> Self {
        Workers {
            seconds: 0,
            workers: Vec::with_capacity(MAX_WORKERS),
        }
    }

    fn seconds(&self) -> usize {
        self.seconds
    }

    fn take_jobs(&mut self, available: &mut Vec<Step>) -> Vec<Step> {
        let available_workers = MAX_WORKERS - self.workers.len();

        let taken_jobs: Vec<_> = available.iter().take(available_workers).cloned().collect();

        let time = self.seconds;
        self.workers
            .extend(taken_jobs.iter().map(|step| (*step, time)));

        taken_jobs
    }

    fn time_to_complete(step: Step) -> usize {
        ((step - b'A') + 61) as usize
    }

    fn tick(&mut self) -> Vec<Step> {
        self.seconds += 1;

        let time_spent = self.seconds;

        self.workers
            .drain_filter(|(step, time)| (*time + Workers::time_to_complete(*step)) <= time_spent)
            .map(|(step, _time)| step)
            .collect()
    }
}

fn part_two(steps: &HashSet<Step>, requirements: &HashMap<Step, Vec<Requires>>) {
    const BASE_STEP_TME: usize = 6;

    let mut workers = Workers::new();
    let mut completed: Vec<Step> = Vec::new();
    let mut uncompleted: Vec<Step> = Vec::new();
    uncompleted.extend(steps);

    let nr_of_steps = uncompleted.len();
    let mut seconds = 0;

    while completed.len() != nr_of_steps {
        let mut available = available_steps(&uncompleted, &completed, &requirements);
        available.sort_unstable();

        let taken = workers.take_jobs(&mut available);

        if !taken.is_empty() {
            println!(
                "The following steps were taken as jobs on second {}:",
                workers.seconds()
            );
            print_steps(&taken);
        }

        for chosen_step in taken {
            let index = uncompleted.iter().position(|x| *x == chosen_step).unwrap();
            uncompleted.remove(index);
        }

        let done = workers.tick();

        completed.extend(done);
    }

    println!(
        "It took {} workers {} seconds to complete all steps",
        MAX_WORKERS,
        workers.seconds()
    );
}

fn available_steps(
    uncompleted: &Vec<Step>,
    completed: &Vec<u8>,
    requirements: &HashMap<Step, Vec<Requires>>,
) -> Vec<u8> {
    uncompleted
        .iter()
        .filter(|step| can_be_completed(&requirements[*step], &completed))
        .cloned()
        .collect()
}

fn print_steps(steps: &Vec<Step>) {
    for step in steps {
        print!("{}", *step as char);
    }
    println!();
}

fn can_be_completed(requirements: &Vec<Requires>, completed: &Vec<Step>) -> bool {
    requirements.iter().all(|r| completed.contains(r))
}

#[derive(Debug)]
struct Requirement {
    step: u8,
    requires: u8,
}

named!(parse_input<CompleteByteSlice, Vec<Requirement>>,
    exact!(separated_list!(
        is_a!("\r\n"),
        parse_requirement
    ))
);

named!(parse_requirement<CompleteByteSlice, Requirement>, do_parse!(
    tag!("Step ") >>
    requires: take!(1) >>
    tag!(" must be finished before step ") >>
    step: take!(1) >>
    tag!(" can begin.") >>

    (Requirement {
        step: step.0[0],
        requires: requires.0[0]
    })
));
