/*
--- Part Two ---

Something seems off about your calculation. The cooling rate depends on exterior surface area, but your calculation also included the surface area of air pockets trapped in the lava droplet.

Instead, consider only cube sides that could be reached by the water and steam as the lava droplet tumbles into the pond. The steam will expand to reach as much as possible, completely displacing any air on the outside of the lava droplet but never expanding diagonally.

In the larger example above, exactly one cube of air is trapped within the lava droplet (at 2,2,5), so the exterior surface area of the lava droplet is 58.

What is the exterior surface area of your scanned lava droplet?
*/
use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("src/bin/day18.txt").unwrap();

    let droplets = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',').map(|x| x.parse::<i64>().unwrap());
            (
                coords.next().unwrap(),
                coords.next().unwrap(),
                coords.next().unwrap(),
            )
        })
        .collect::<HashSet<_>>();

    let min = droplets
        .iter()
        .flat_map(|&(x, y, z)| [x, y, z])
        .min()
        .unwrap()
        - 1;
    let max = droplets
        .iter()
        .flat_map(|&(x, y, z)| [x, y, z])
        .max()
        .unwrap()
        + 1;
    let mut visited = HashSet::new();
    let mut to_visit = vec![(0, 0, 0)];
    while let Some(p) = to_visit.pop() {
        for (x, y, z) in adjacent_coords(p) {
            if !droplets.contains(&(x, y, z))
                && !visited.contains(&(x, y, z))
                && [x, y, z].iter().all(|&i| (min..=max).contains(&i))
            {
                visited.insert((x, y, z));
                to_visit.push((x, y, z));
            }
        }
    }
    let exterior_surface_area = droplets
        .iter()
        .flat_map(|&c| adjacent_coords(c))
        .filter(|s| visited.contains(s))
        .count();

    println!("Solution: {exterior_surface_area}");
}

fn adjacent_coords((x, y, z): (i64, i64, i64)) -> [(i64, i64, i64); 6] {
    [
        (x - 1, y, z),
        (x, y - 1, z),
        (x, y, z - 1),
        (x + 1, y, z),
        (x, y + 1, z),
        (x, y, z + 1),
    ]
}
