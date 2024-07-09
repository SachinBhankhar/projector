fn get_input() -> &'static str {
    return "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
}

fn main() {
    let mut count = 0;

    for (idx, line) in get_input().lines().enumerate() {
        if let Some(c) = line.chars().nth(idx * 3 % line.len()) {
            if c == '#' {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
