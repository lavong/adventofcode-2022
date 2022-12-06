/*
--- Part Two ---

Your device's communication system is correctly detecting packets, but still isn't working. It looks like it also needs to look for messages.

A start-of-message marker is just like a start-of-packet marker, except it consists of 14 distinct characters rather than 4.

Here are the first positions of start-of-message markers for all of the above examples:

    mjqjpqmgbljsphdztnvjfqwrcgsmlb: first marker after character 19
    bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 23
    nppdvjthqldpwncqszvftbrmjlhg: first marker after character 23
    nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 29
    zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 26

How many characters need to be processed before the first start-of-message marker is detected?
*/
use itertools::Itertools;
use std::collections::HashSet;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/day6.txt").unwrap();

    let chars = input.chars().collect_vec();
    let mut marker = 14;
    for tuple in chars.windows(14) {
        let mut tuple_set: HashSet<char> = HashSet::new();
        let all_different = (0..14)
            .map(|i| tuple[i])
            .fold(true, |acc, c| acc && tuple_set.insert(c));
        if all_different {
            break;
        }
        marker += 1;
    }

    println!("Solution: {marker}");
}
