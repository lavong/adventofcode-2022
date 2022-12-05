/*
--- Part Two ---

As you watch the crane operator expertly rearrange the crates, you notice the process isn't following your prediction.

Some mud was covering the writing on the side of the crane, and you quickly wipe it away. The crane isn't a CrateMover 9000 - it's a CrateMover 9001.

The CrateMover 9001 is notable for many new and exciting features: air conditioning, leather seats, an extra cup holder, and the ability to pick up and move multiple crates at once.

Again considering the example above, the crates begin in the same configuration:

    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

Moving a single crate from stack 2 to stack 1 behaves the same as before:

[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 

However, the action of moving three crates from stack 1 to stack 3 means that those three moved crates stay in the same order, resulting in this new configuration:

        [D]
        [N]
    [C] [Z]
    [M] [P]
 1   2   3

Next, as both crates are moved from stack 2 to stack 1, they retain their order as well:

        [D]
        [N]
[C]     [Z]
[M]     [P]
 1   2   3

Finally, a single crate is still moved from stack 1 to stack 2, but now it's crate C that gets moved:

        [D]
        [N]
        [Z]
[M] [C] [P]
 1   2   3

In this example, the CrateMover 9001 has put the crates in a totally different order: MCD.

Before the rearrangement process finishes, update your simulation so that the Elves know where they should stand to be ready to unload the final supplies. After the rearrangement procedure completes, what crate ends up on top of each stack?
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
            .rev()
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
