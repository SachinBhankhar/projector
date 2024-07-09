package main

import (
	"fmt"
	"strings"
)

func getInput() string {
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
.#..#...#.#`
}

func main() {
	treeCount := 0
	for r, line := range strings.Split(getInput(), "\n") {
		a := line[r*3%len(line)]
		if string(a) == "#" {
			treeCount += 1
		}
	}

	fmt.Printf("%v", treeCount)
}
