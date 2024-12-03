use regex::Regex;

fn main() {
    let text = include_str!("../input.txt");

    let regex = Regex::new("mul\\(([0-9]){1,3}\\,([0-9]){1,3}\\)").unwrap();

    let dos = Regex::new("do\\(\\)")
        .unwrap()
        .find_iter(text)
        .map(|x| x.start())
        .collect::<Vec<_>>();
    let donts = Regex::new("don't\\(\\)")
        .unwrap()
        .find_iter(text)
        .map(|x| x.start())
        .collect::<Vec<_>>();

    let instructions = regex
        .captures_iter(text)
        .zip(regex.find_iter(text))
        .map(|(capture, range)| (capture[0].to_string(), range.start()))
        .collect::<Vec<_>>();

    println!("{instructions:?}");

    let sum = instructions
        .into_iter()
        .map(|(instruction, start)| {
            let closest_dont = donts
                .iter()
                .copied()
                .filter(|dont| *dont < start)
                .max()
                .unwrap_or(0);
            let closest_do = dos
                .iter()
                .copied()
                .filter(|x| *x < start)
                .max()
                .unwrap_or(0);

            if closest_do < closest_dont {
                return 0;
            }

            instruction[4..(instruction.len() - 1)]
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum::<u32>();

    println!("{sum:?}");
}
