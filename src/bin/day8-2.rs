/*
--- Part Two ---

Content with the amount of tree cover available, the Elves just need to know the best spot to build their tree house: they would like to be able to see a lot of trees.

To measure the viewing distance from a given tree, look up, down, left, and right from that tree; stop if you reach an edge or at the first tree that is the same height or taller than the tree under consideration. (If a tree is right on the edge, at least one of its viewing distances will be zero.)

The Elves don't care about distant trees taller than those found by the rules above; the proposed tree house has large eaves to keep it dry, so they wouldn't be able to see higher than the tree house anyway.

In the example above, consider the middle 5 in the second row:

30373
25512
65332
33549
35390

    Looking up, its view is not blocked; it can see 1 tree (of height 3).
    Looking left, its view is blocked immediately; it can see only 1 tree (of height 5, right next to it).
    Looking right, its view is not blocked; it can see 2 trees.
    Looking down, its view is blocked eventually; it can see 2 trees (one of height 3, then the tree of height 5 that blocks its view).

A tree's scenic score is found by multiplying together its viewing distance in each of the four directions. For this tree, this is 4 (found by multiplying 1 * 1 * 2 * 2).

However, you can do even better: consider the tree of height 5 in the middle of the fourth row:

30373
25512
65332
33549
35390

    Looking up, its view is blocked at 2 trees (by another tree with a height of 5).
    Looking left, its view is not blocked; it can see 2 trees.
    Looking down, its view is also not blocked; it can see 1 tree.
    Looking right, its view is blocked at 2 trees (by a massive tree of height 9).

This tree's scenic score is 8 (2 * 2 * 1 * 2); this is the ideal spot for the tree house.

Consider each tree on your map. What is the highest scenic score possible for any tree?
*/
use std::fs;

fn main() {
    let input = fs::read_to_string("src/bin/day8.txt").unwrap();

    let forest: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = forest.len();
    let cols = forest[0].len();
    let mut scenic_scores = vec![vec![0; cols]; rows];
    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            scenic_scores[y][x] = scenic_score(&forest, rows, cols, x, y);
        }
    }
    let highest_scenic_score = scenic_scores.iter().flat_map(|r| r.iter()).max().unwrap();

    println!("Solution: {highest_scenic_score}");
}

fn scenic_score(forest: &Vec<Vec<u32>>, rows: usize, cols: usize, x: usize, y: usize) -> u32 {
    let height = forest[y][x];

    let mut left = 0;
    for i in (0..x).rev() {
        if forest[y][i] < height {
            left += 1
        } else {
            break;
        }
    }
    left = left.max(1);

    let mut right = 1;
    for i in x + 1..cols {
        if forest[y][i] < height {
            right += 1
        } else {
            break;
        }
    }
    right = right.min((cols - x - 1) as u32);

    let mut top = 1;
    for i in (0..y).rev() {
        if forest[i][x] < height {
            top += 1
        } else {
            break;
        }
    }
    top = top.max(1);

    let mut bot = 0;
    for i in y + 1..rows {
        if forest[i][x] < height {
            bot += 1
        } else {
            break;
        }
    }
    bot = bot.max(1);

    left * right * top * bot
}
