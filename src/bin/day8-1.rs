/*
--- Day 8: Treetop Tree House ---

The expedition comes across a peculiar patch of tall trees all planted carefully in a grid. The Elves explain that a previous expedition planted these trees as a reforestation effort. Now, they're curious if this would be a good location for a tree house.

First, determine whether there is enough tree cover here to keep a tree house hidden. To do this, you need to count the number of trees that are visible from outside the grid when looking directly along a row or column.

The Elves have already launched a quadcopter to generate a map with the height of each tree (your puzzle input). For example:

30373
25512
65332
33549
35390

Each tree is represented as a single digit whose value is its height, where 0 is the shortest and 9 is the tallest.

A tree is visible if all of the other trees between it and an edge of the grid are shorter than it. Only consider trees in the same row or column; that is, only look up, down, left, or right from any given tree.

All of the trees around the edge of the grid are visible - since they are already on the edge, there are no trees to block the view. In this example, that only leaves the interior nine trees to consider:

    The top-left 5 is visible from the left and top. (It isn't visible from the right or bottom since other trees of height 5 are in the way.)
    The top-middle 5 is visible from the top and right.
    The top-right 1 is not visible from any direction; for it to be visible, there would need to only be trees of height 0 between it and an edge.
    The left-middle 5 is visible, but only from the right.
    The center 3 is not visible from any direction; for it to be visible, there would need to be only trees of at most height 2 between it and an edge.
    The right-middle 3 is visible from the right.
    In the bottom row, the middle 5 is visible, but the 3 and 4 are not.

With 16 trees visible on the edge and another 5 visible in the interior, a total of 21 trees are visible in this arrangement.

Consider your map; how many trees are visible from outside the grid?
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
    let mut visible_trees = vec![vec![true; cols]; rows];
    for y in 1..rows - 1 {
        for x in 1..cols - 1 {
            visible_trees[y][x] = is_visible(&forest, rows, cols, x, y);
        }
    }
    let visible_tree_count = visible_trees
        .iter()
        .flat_map(|r| r.iter())
        .fold(0, |acc, v| if *v { acc + 1 } else { acc });

    println!("Solution: {visible_tree_count}");
}

fn is_visible(forest: &Vec<Vec<u32>>, rows: usize, cols: usize, x: usize, y: usize) -> bool {
    let height = forest[y][x];

    // visible left
    (0..x)
        .map(|i| forest[y][i])
        .fold(true, |acc, h| acc && h < height) ||

    // visible right
    (x + 1..cols)
        .map(|i| forest[y][i])
        .rfold(true, |acc, h| acc && h < height) ||

    // visible top
    (0..y)
        .map(|i| forest[i][x])
        .fold(true, |acc, h| acc && h < height) ||

    // visible bot
    (y + 1..rows)
        .map(|i| forest[i][x])
        .rfold(true, |acc, h| acc && h < height)
}
