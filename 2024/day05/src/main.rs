use std::cmp::Ordering;

fn main() {
    let text = include_str!("../input.txt");
    let [rules, updates, ..] = text.split("\n\n").collect::<Vec<_>>()[..] else {
        return;
    };

    let rules = rules
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            (
                line[0..2].parse::<u32>().unwrap(),
                line[3..5].parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let updates = updates
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let valid = updates
        .clone()
        .into_iter()
        .filter(|update| {
            (0..update.len()).all(|i| {
                rules
                    .iter()
                    .filter(|(before, _)| *before == update[i])
                    .all(|(_, after)| !update[0..i].contains(after))
            })
        })
        .inspect(|update| println!("{update:?}"))
        .map(|update| update[(update.len() - 1) / 2])
        .sum::<u32>();

    println!("{valid}");

    let invalid = updates
        .into_iter()
        .filter(|update| {
            !(0..update.len()).all(|i| {
                rules
                    .iter()
                    .filter(|(before, _)| *before == update[i])
                    .all(|(_, after)| !update[0..i].contains(after))
            })
        })
        .map(|mut update| {
            update.sort_by(|a, b| {
                let out_of_order = rules
                    .iter()
                    .any(|(before, after)| before == b && after == a);
                if out_of_order {
                    Ordering::Greater
                } else {
                    Ordering::Less
                }
            });
            update
        })
        .inspect(|update| println!("{update:?}"))
        .map(|update| update[(update.len() - 1) / 2])
        .sum::<u32>();

    println!("{invalid}");
}
