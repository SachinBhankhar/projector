function getInput(): string {
    return `..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#`;
}

enum Thing {
    Tree,
    Snow,
}


const things = getInput().split('\n')
    .map(s => s.split("")
        .map(x => x === "." ? Thing.Snow : Thing.Tree))

const colLen = things[0].length;
let count = 0;
let idx = 0;

things.forEach((thingRow, i) => {
    if (thingRow[i * 3 % colLen] === Thing.Tree) {
        count += 1
    }
});
console.log(count);
