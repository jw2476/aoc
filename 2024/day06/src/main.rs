use std::collections::HashSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let tiles = include_str!("../input.txt")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let width = tiles[0].len() as isize;
    let height = tiles.len() as isize;

    let obstructions = tiles
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .collect::<Vec<(isize, isize)>>();

    let guard = tiles
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, c)| **c == '^')
                .map(move |(x, _)| (x as isize, y as isize))
        })
        .next()
        .unwrap();

    /*let mut positions = HashSet::new();

    loop {
        if (guard.0 < 0) || (guard.1 < 0) || (guard.0 >= width) || (guard.1 >= height) {
            break;
        }

        positions.insert(guard);

        if obstructions.contains(&(guard.0 + facing.0, guard.1 + facing.1)) {
            facing = match facing {
                (0, -1) => (1, 0),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (1, 0) => (0, 1),
                _ => panic!(),
            };
            continue;
        }

        guard = (guard.0 + facing.0, guard.1 + facing.1);
    }

    println!("{}", positions.len());*/

    let loops = tiles
        .iter()
        .enumerate()
        .flat_map(|(y, line)| (0..line.len()).map(move |x| (x as isize, y as isize)))
        .collect::<Vec<_>>()
        .par_iter()
        .filter(|(x, y)| {
            let mut obstructions = obstructions.clone();

            let mut guard = guard;
            let mut facing = (0, -1);
            obstructions.push((*x, *y));

            let mut positions = Vec::new();

            loop {
                if positions.contains(&(guard, facing)) {
                    break true;
                }

                if (guard.0 < 0) || (guard.1 < 0) || (guard.0 >= width) || (guard.1 >= height) {
                    break false;
                }

                positions.push((guard, facing));

                if obstructions.contains(&(guard.0 + facing.0, guard.1 + facing.1)) {
                    facing = match facing {
                        (0, -1) => (1, 0),
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        (1, 0) => (0, 1),
                        _ => panic!(),
                    };
                    continue;
                }

                guard = (guard.0 + facing.0, guard.1 + facing.1);
            }
        })
        .count();

    println!("{loops}");
}
