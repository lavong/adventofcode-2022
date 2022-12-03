/*
--- Part Two ---

As you finish identifying the misplaced items, the Elves come to you with another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a badge that identifies their group. For efficiency, within each group of three Elves, the badge is the only item type carried by all three Elves. That is, if a group's badge is item type B, then all three Elves will have item type B somewhere in their rucksack, and at most two of the Elves will be carrying any other item type.

The problem is that someone forgot to put this year's updated authenticity sticker on the badges. All of the badges need to be pulled out of the rucksacks so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's badges. The only way to tell which item type is the right one is by finding the one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each group can have a different badge item type. So, in the above example, the first group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is lowercase r; this must be their badges. In the second group, their badge item type must be Z.

Priorities for these items must still be found to organize the sticker attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the second group. The sum of these is 70.

Find the item type that corresponds to the badges of each three-Elf group. What is the sum of the priorities of those item types?
*/
use itertools::Chunk;
use itertools::Itertools;
use std::fs;
use std::str::Lines;

fn main() {
    let input = fs::read_to_string("src/bin/day3.txt").unwrap();

    let chunks = input.lines().chunks(3);
    let priorities: u32 = chunks
        .into_iter()
        .map(|chunk| find_badge(chunk))
        .map(|c| c as u32 - if c.is_lowercase() { 96u32 } else { 38u32 })
        .sum();

    println!("Solution: {priorities}");
}

fn find_badge(mut chunk: Chunk<Lines>) -> char {
    let mut shared: String = chunk.next().unwrap().to_string();
    let line2 = chunk.next().unwrap();
    let line3 = chunk.next().unwrap();

    shared.retain(|c| line2.contains(c));
    shared.retain(|c| line3.contains(c));

    shared.chars().next().unwrap()
}
