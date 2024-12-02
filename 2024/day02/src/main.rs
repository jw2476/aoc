#![feature(array_windows)]

fn main() {
    let safe = include_str!("../input.txt")
        .split("\n")
        .filter(|x| !x.is_empty())
        .filter(|report| {
            let report = report
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(|x| x.parse::<i32>().unwrap());

            (0..report.clone().count() + 1) // + 1 so there's a run that removes nothing
                .find(|remove| {
                    let report = report
                        .clone()
                        .enumerate()
                        .filter(|(i, _)| i != remove)
                        .map(|(_, x)| x)
                        .collect::<Vec<_>>();
                    let increasing = report.array_windows().all(|[a, b]| a < b);
                    let decreasing = report.array_windows().all(|[a, b]| a > b);
                    let change = report.array_windows().all(|[a, b]| (a - b).abs() <= 3);

                    (increasing || decreasing) && change
                })
                .is_some() 
        })
        .count();

    println!("{safe}");
}
