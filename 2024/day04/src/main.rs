fn main() {
    let lines = include_str!("../input.txt")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    {
        let is_xmas = |start: (usize, usize), diff: (isize, isize)| {
            (0..4)
                .map(|i| {
                    (
                        (start.0 as isize) + (diff.0 * i),
                        (start.1 as isize) + (diff.1 * i),
                    )
                })
                .filter(|(x, y)| *x >= 0 && *y >= 0)
                .filter_map(|(x, y)| lines.get(y as usize).and_then(|line| line.get(x as usize)))
                .collect::<String>()
                == String::from("XMAS")
        };

        let total = (0..lines[0].len())
            .flat_map(|x| (0..lines.len()).map(move |y| (x, y)))
            .map(|start| {
                let sum = [
                    is_xmas(start, (0, -1)) as usize,
                    is_xmas(start, (0, 1)) as usize,
                    is_xmas(start, (-1, 0)) as usize,
                    is_xmas(start, (1, 0)) as usize,
                    is_xmas(start, (-1, -1)) as usize,
                    is_xmas(start, (-1, 1)) as usize,
                    is_xmas(start, (1, -1)) as usize,
                    is_xmas(start, (1, 1)) as usize,
                ]
                .into_iter()
                .sum::<usize>();

                if sum != 0 {
                    println!("{start:?} {sum}");
                }

                sum
            })
            .sum::<usize>();

        println!("{total}");
    }

    {
        let get = |pos: (usize, usize)| lines[pos.1][pos.0];

        let check = |pos: (usize, usize)| {
            if get(pos) != 'A' {
                return false;
            }

            let chars = [
                get((pos.0 - 1, pos.1 - 1)),
                get((pos.0 - 1, pos.1 + 1)),
                get((pos.0 + 1, pos.1 - 1)),
                get((pos.0 + 1, pos.1 + 1)),
            ];

            chars.iter().filter(|c| **c == 'M').count() == 2
            && chars.iter().filter(|c| **c == 'S').count() == 2
            && chars[0] != chars[3]
        };

        let total = (1..lines[0].len() - 1)
            .flat_map(|x| (1..lines.len() - 1).map(move |y| (x, y)))
            .filter(|pos| check(*pos))
            .count();

        println!("{total}");
    }
}
