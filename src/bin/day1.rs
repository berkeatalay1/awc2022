fn main() {
    let input = include_str!("../input/day1.txt");
    let lines = input.split("\n\n");

    // flat map removes results and errors -> errors are dropped
    let mut lines_parsed: Vec<u32> = lines
        .map(|line| line.split("\n").flat_map(|num| num.parse::<u32>()).sum())
        .collect();

    lines_parsed.sort_by(|a, b| b.cmp(a));

    println!("---------------------- Part1 -------------------------");
    println!("{:?}", lines_parsed[0]);
    println!("---------------------- Part1 -------------------------");

    println!("---------------------- Part2 -------------------------");
    println!("{:?}", lines_parsed.iter().take(3).sum::<u32>());
    println!("---------------------- Part2 -------------------------");
}
