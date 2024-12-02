fn main() {
    let text = include_str!("../input.txt")
        .split('\n')
        .filter(|line| !line.is_empty());
    let mut left = text
        .clone()
        .map(|line| line[0..5].parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut right = text
        .map(|line| line[8..13].parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    left.sort();
    right.sort();

    let distance = left
        .clone()
        .into_iter()
        .zip(right.clone())
        .map(|(lhs, rhs)| (lhs - rhs).abs())
        .sum::<i32>();

    println!("{distance}");

    let similarity = left
        .into_iter()
        .map(|lhs| lhs as usize * right.iter().filter(|rhs| lhs == **rhs).count())
        .sum::<usize>();

    println!("{similarity}");
}
