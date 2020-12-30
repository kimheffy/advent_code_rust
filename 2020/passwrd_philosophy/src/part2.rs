pub fn part2(line: &str) -> bool {
    let parse: Vec<&str> = line.split(": ").map(|l| l.trim()).collect();
    let instructions: Vec<&str> = parse[0].split(" ").map(|l| l.trim()).collect();
    let positions: Vec<usize> = instructions[0].split("-").map(|l| l.parse::<usize>().unwrap()).collect();
    let low = positions[0];
    let low = low - 1;
    let high = positions[1];
    let high = high - 1;
    let target = instructions[1].parse::<char>().unwrap();
    let passwrd = parse[1];

    let mut counter = 0;

    for (i, val) in passwrd.chars().enumerate() {
        if i == low {
            if val == target {
                counter += 1;
            }
        } else if i == high {
            if val == target {
                counter += 1;
            }
        }
    }

    counter == 1
}
