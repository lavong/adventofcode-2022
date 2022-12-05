/*
--- Day 5: Supply Stacks ---

The expedition can depart as soon as the final supplies have been unloaded from the ships. Supplies are stored in stacks of marked crates, but because the needed supplies are buried under many other crates, the crates need to be rearranged.

The ship has a giant cargo crane capable of moving crates between stacks. To ensure none of the crates get crushed or fall over, the crane operator will rearrange them in a series of carefully-planned steps. After the crates are rearranged, the desired crates will be at the top of each stack.

The Elves don't want to interrupt the crane operator during this delicate procedure, but they forgot to ask her which crate will end up where, and they want to be ready to unload them as soon as possible so they can embark.

They do, however, have a drawing of the starting stacks of crates and the rearrangement procedure (your puzzle input). For example:

    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

In this example, there are three stacks of crates. Stack 1 contains two crates: crate Z is on the bottom, and crate N is on top. Stack 2 contains three crates; from bottom to top, they are crates M, C, and D. Finally, stack 3 contains a single crate, P.

Then, the rearrangement procedure is given. In each step of the procedure, a quantity of crates is moved from one stack to a different stack. In the first step of the above rearrangement procedure, one crate is moved from stack 2 to stack 1, resulting in this configuration:

[D]
[N] [C]
[Z] [M] [P]
 1   2   3

In the second step, three crates are moved from stack 1 to stack 3. Crates are moved one at a time, so the first crate to be moved (D) ends up below the second and third crates:

        [Z]
        [N]
    [C] [D]
    [M] [P]
 1   2   3

Then, both crates are moved from stack 2 to stack 1. Again, because crates are moved one at a time, crate C ends up below crate M:

        [Z]
        [N]
[M]     [D]
[C]     [P]
 1   2   3

Finally, one crate is moved from stack 1 to stack 2:

        [Z]
        [N]
        [D]
[C] [M] [P]
 1   2   3

The Elves just need to know which crate will end up on top of each stack; in this example, the top crates are C in stack 1, M in stack 2, and Z in stack 3, so you should combine these together and give the Elves the message CMZ.

After the rearrangement procedure completes, what crate ends up on top of each stack?
*/
use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/day5.txt").unwrap();

    let mut stack_input = vec![];
    for line in input.lines() {
        if line.is_empty() {
            break;
        }
        stack_input.push(line);
    }
    let stack_count: u32 = stack_input
        .pop()
        .unwrap()
        .trim()
        .chars()
        .last()
        .unwrap()
        .to_digit(10)
        .unwrap();

    stack_input.reverse();
    let mut supply_stacks = SupplyStacks::default();
    supply_stacks
        .stacks
        .resize(stack_count as usize, VecDeque::new());
    stack_input.iter().for_each(|s| {
        for i in 1..=stack_count {
            let c = s.chars().nth((i + (i - 1) * 3) as usize).unwrap();
            if c.is_alphabetic() {
                supply_stacks.stacks[(i - 1) as usize].push_front(c)
            }
        }
    });

    let rearrangements = input.lines().filter(|l| l.starts_with("move")).map(|l| {
        let s = l.split(" ").collect::<Vec<_>>();
        Rearrangement {
            quantity: s[1].parse().unwrap(),
            start: s[3].parse().unwrap(),
            destination: s[5].parse().unwrap(),
        }
    });

    rearrangements.for_each(|r| {
        supply_stacks.stacks[r.start - 1]
            .drain(0..r.quantity)
            .collect::<String>()
            .chars()
            .for_each(|c| supply_stacks.stacks[r.destination - 1].push_front(c));
    });
    let top_crates: String = (0..stack_count)
        .map(|i| supply_stacks.stacks[i as usize].pop_front().unwrap())
        .collect();

    println!("Solution: {top_crates}");
}

#[derive(Debug, Default)]
struct Rearrangement {
    quantity: usize,
    start: usize,
    destination: usize,
}

#[derive(Debug, Default)]
struct SupplyStacks {
    stacks: Vec<VecDeque<char>>,
}
