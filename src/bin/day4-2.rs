/*
--- Part Two ---

It seems like there is still quite a bit of duplicate work planned. Instead, the Elves would like to know the number of pairs that overlap at all.

In the above example, the first two pairs (2-4,6-8 and 2-3,4-5) don't overlap, while the remaining four pairs (5-7,7-9, 2-8,3-7, 6-6,4-6, and 2-6,4-8) do overlap:

    5-7,7-9 overlaps in a single section, 7.
    2-8,3-7 overlaps all of the sections 3 through 7.
    6-6,4-6 overlaps in a single section, 6.
    2-6,4-8 overlaps in sections 4, 5, and 6.

So, in this example, the number of overlapping assignment pairs is 4.

In how many assignment pairs do the ranges overlap?
*/
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/day4.txt").unwrap();

    let solution = input
        .lines()
        .map(|line| {
            line.split(",")
                .flat_map(|r| r.split("-"))
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .filter(|i| {
            let section1 = i[0]..=i[1];
            let section2 = i[2]..=i[3];
            section1.contains(&i[2])
                || section1.contains(&i[3])
                || section2.contains(&i[0])
                || section2.contains(&i[1])
        })
        .count();

    println!("Solution: {solution}");
}
