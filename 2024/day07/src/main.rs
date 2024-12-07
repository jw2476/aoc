fn main() {
    let lines = include_str!("../input.txt")
        .split('\n')
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let results = lines
        .iter()
        .map(|line| line.split(':').next().unwrap().parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let operands = lines
        .iter()
        .map(|line| {
            line.split(": ")
                .nth(1)
                .unwrap()
                .split(' ')
                .map(|operand| operand.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let calibrations = results
        .into_iter()
        .zip(operands)
        .collect::<Vec<(u64, Vec<u64>)>>();

    let valid = calibrations
        .iter()
        .filter(|(result, operands)| {
            (0..3_u64.pow(operands.len() as u32 - 1)).any(|operators| {
                operands[1..]
                    .iter()
                    .enumerate()
                    .fold(operands[0], |total, (i, operand)| {
                        let operator = (operators / 3_u64.pow(i as u32)) % 3;
                        match operator {
                            0 => total + operand,
                            1 => total * operand,
                            2 => format!("{total}{operand}").parse::<u64>().unwrap(),
                            _ => panic!()
                        }
                    })
                    == *result
            })
        })
        .map(|(result, _)| result)
        .sum::<u64>();

    println!("{valid}");
}
