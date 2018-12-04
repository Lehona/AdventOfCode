#[macro_use]
extern crate nom;

type MidnightMinutes = [usize; 60];

fn main() {
    let input = include_str!("../input.txt");

    let mut entries: Vec<_> = input
        .lines()
        .map(|l| entry(CompleteStr(l)).unwrap().1)
        .collect();

    entries.sort_unstable_by(|a, b| a.time.cmp(&b.time));

    let mut times = HashMap::new();
    let mut intervals = HashMap::new();
    let mut current_id = 0;
    let mut current_time = Default::default();
    for entry in entries {
        match entry.act {
            Activity::Begin(id) => current_id = id,
            Activity::Sleep => current_time = entry.time,
            Activity::WakeUp => {
                let e = times.entry(current_id).or_insert(0);
                *e += entry.time.minute - current_time.minute;

                let vec = intervals.entry(current_id).or_insert(Vec::new());

                vec.push((current_time, entry.time));
            }
        }
    }

    // strategy 1

    let result = times.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();

    let sleeping_minutes = get_sleeping_minutes(&intervals[result.0]);
    let max_minute = get_max_sleeping_minute(&sleeping_minutes);
    println!(
        "The guard {} was asleep the longest, for {} minutes, at minute {}, so the solution is {}.",
        *result.0,
        *result.1,
        max_minute,
        *result.0 * max_minute
    );

    // strategy 2

    let (guard, minutes, minute) = intervals
        .iter()
        .map(|(key, vec)| (*key, get_sleeping_minutes(vec)))
        .map(|(id, minutes)| (id, minutes, get_max_sleeping_minute(&minutes)))
        .max_by_key(|(id, minutes, minute)| minutes[*minute])
        .unwrap();

    println!(
        "The guard {} was asleep the longest on minute {} for {} minutes, so the solution is {}.",
        guard,
        minute,
        minutes[minute],
        guard * minute
    );
}

fn get_max_sleeping_minute(minutes: &MidnightMinutes) -> usize {
    minutes
        .iter()
        .enumerate()
        .max_by_key(|(i, min)| *min)
        .unwrap()
        .0
}

fn get_sleeping_minutes(intervals: &Vec<(TimeStamp, TimeStamp)>) -> MidnightMinutes {
    let mut minutes = [0; 60];

    for (start, end) in intervals {
        if start.hour != 0
            && end.hour != 0
            && start.day == end.day
            && start.month == end.month
            && start.year == end.year
        {
            panic!("Illegal sleep intervall");
        };

        for sleeping_minute in start.minute..end.minute {
            minutes[sleeping_minute] += 1;
        }
    }

    minutes
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct TimeStamp {
    pub year: usize,
    pub month: usize,
    pub day: usize,
    pub hour: usize,
    pub minute: usize,
}

use nom::types::CompleteStr;
use nom::{digit1, multispace1};
use std::collections::HashMap;

named!(timestamp<CompleteStr, TimeStamp>, do_parse!(
    char!('[') >>
    year: digit1 >>
    char!('-') >>
    month: digit1 >>
    char!('-') >>
    day: digit1 >>
    multispace1 >>
    hour: digit1 >>
    char!(':') >>
    minute: digit1 >>
    char!(']') >>

    (TimeStamp {
        year: year.parse().unwrap(),
        month: month.parse().unwrap(),
        day: day.parse().unwrap(),
        hour: hour.parse().unwrap(),
        minute: minute.parse().unwrap()
    })
));

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub time: TimeStamp,
    pub act: Activity,
}

#[derive(Debug, Clone, Copy)]
pub enum Activity {
    Begin(usize),
    Sleep,
    WakeUp,
}

named!(entry<CompleteStr, Entry>, do_parse!(
    time: timestamp >>
    act: activity >>
    (Entry{time, act})
));

named!(activity<CompleteStr, Activity>, alt!(
    begin | sleep | awake
));

named!(begin<CompleteStr, Activity>, do_parse!(
    tag!(" Guard") >>
    multispace1 >>
    char!('#') >>
    id: digit1 >>
    multispace1 >>
    tag!("begins shift") >>
    (Activity::Begin(id.parse().unwrap()))
));

named!(sleep<CompleteStr, Activity>,
    map!(
        tag!(" falls asleep"),
        |_|Activity::Sleep
    )
);

named!(awake<CompleteStr, Activity>,
    map!(
        tag!(" wakes up"),
        |_|Activity::WakeUp
    )
);

//[1518-03-04 00:39] falls asleep
