/*
--- Part Two ---

You're worried you might not ever get your items back. So worried, in fact, that your relief that a monkey's inspection didn't damage an item no longer causes your worry level to be divided by three.

Unfortunately, that relief was all that was keeping your worry levels from reaching ridiculous levels. You'll need to find another way to keep your worry levels manageable.

At this rate, you might be putting up with these monkeys for a very long time - possibly 10000 rounds!

With these new rules, you can still figure out the monkey business after 10000 rounds. Using the same example above:

== After round 1 ==
Monkey 0 inspected items 2 times.
Monkey 1 inspected items 4 times.
Monkey 2 inspected items 3 times.
Monkey 3 inspected items 6 times.

== After round 20 ==
Monkey 0 inspected items 99 times.
Monkey 1 inspected items 97 times.
Monkey 2 inspected items 8 times.
Monkey 3 inspected items 103 times.

== After round 1000 ==
Monkey 0 inspected items 5204 times.
Monkey 1 inspected items 4792 times.
Monkey 2 inspected items 199 times.
Monkey 3 inspected items 5192 times.

== After round 2000 ==
Monkey 0 inspected items 10419 times.
Monkey 1 inspected items 9577 times.
Monkey 2 inspected items 392 times.
Monkey 3 inspected items 10391 times.

== After round 3000 ==
Monkey 0 inspected items 15638 times.
Monkey 1 inspected items 14358 times.
Monkey 2 inspected items 587 times.
Monkey 3 inspected items 15593 times.

== After round 4000 ==
Monkey 0 inspected items 20858 times.
Monkey 1 inspected items 19138 times.
Monkey 2 inspected items 780 times.
Monkey 3 inspected items 20797 times.

== After round 5000 ==
Monkey 0 inspected items 26075 times.
Monkey 1 inspected items 23921 times.
Monkey 2 inspected items 974 times.
Monkey 3 inspected items 26000 times.

== After round 6000 ==
Monkey 0 inspected items 31294 times.
Monkey 1 inspected items 28702 times.
Monkey 2 inspected items 1165 times.
Monkey 3 inspected items 31204 times.

== After round 7000 ==
Monkey 0 inspected items 36508 times.
Monkey 1 inspected items 33488 times.
Monkey 2 inspected items 1360 times.
Monkey 3 inspected items 36400 times.

== After round 8000 ==
Monkey 0 inspected items 41728 times.
Monkey 1 inspected items 38268 times.
Monkey 2 inspected items 1553 times.
Monkey 3 inspected items 41606 times.

== After round 9000 ==
Monkey 0 inspected items 46945 times.
Monkey 1 inspected items 43051 times.
Monkey 2 inspected items 1746 times.
Monkey 3 inspected items 46807 times.

== After round 10000 ==
Monkey 0 inspected items 52166 times.
Monkey 1 inspected items 47830 times.
Monkey 2 inspected items 1938 times.
Monkey 3 inspected items 52013 times.

After 10000 rounds, the two most active monkeys inspected items 52166 and 52013 times. Multiplying these together, the level of monkey business in this situation is now 2713310158.

Worry levels are no longer divided by three after each item is inspected; you'll need to find another way to keep your worry levels manageable. Starting again from the initial state in your puzzle input, what is the level of monkey business after 10000 rounds?
*/
use std::fs;

use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("src/bin/day11.txt").unwrap();

    let mut monkeys = parse(&input);
    let mut inspections: Vec<u64> = vec![0; monkeys.len()];
    let monkey_business_restrainer: u64 = monkeys.iter().fold(1, |acc, m| acc * m.3);

    let rounds = 10000;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = monkeys[i].clone();

            for item in monkey.1.iter() {
                let worry_level: u64 = match monkey.2 {
                    Operation::Multiply(m) => item * m,
                    Operation::Add(a) => item + a,
                    Operation::MultiplyOld => item * item,
                } % monkey_business_restrainer;
                let destination = match worry_level % monkey.3 {
                    0 => monkey.4 .0,
                    _ => monkey.4 .1,
                };
                monkeys[destination].1.push(worry_level);
            }
            inspections[i] += monkeys[i].1.len() as u64;
            monkeys[i].1.clear();
        }
    }
    inspections.sort_by(|a, b| b.cmp(a));
    let monkey_business = inspections[0] * inspections[1];

    println!("Solution: {:?}", monkey_business);
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Multiply(u64),
    Add(u64),
    MultiplyOld,
}

fn parse(input: &String) -> Vec<(u64, Vec<u64>, Operation, u64, (usize, usize), u64)> {
    let chunks = input.lines().chunks(7);
    let monkeys = chunks.into_iter().map(|mut chunk| {
        let name: u64 = chunk.next().unwrap()[6..]
            .trim()
            .split(':')
            .next()
            .unwrap()
            .parse()
            .unwrap();
        let items = chunk
            .next()
            .unwrap()
            .split(':')
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split(',')
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let op_str = chunk
            .next()
            .unwrap()
            .split("=")
            .skip(1)
            .next()
            .unwrap()
            .trim();
        let operation = match &op_str[4..5] {
            "*" => match op_str[5..].trim() {
                "old" => Operation::MultiplyOld,
                _ => Operation::Multiply(op_str[5..].trim().parse().unwrap()),
            },
            _ => Operation::Add(op_str[5..].trim().parse().unwrap()),
        };
        let divisible_by = chunk
            .next()
            .unwrap()
            .split("by")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse::<u64>()
            .unwrap();
        let dest1 = chunk
            .next()
            .unwrap()
            .split("monkey")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();
        let dest2 = chunk
            .next()
            .unwrap()
            .split("monkey")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .parse::<usize>()
            .unwrap();

        (name, items, operation, divisible_by, (dest1, dest2), 0)
    });
    monkeys.collect_vec()
}
