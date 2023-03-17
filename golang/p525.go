package main

import (
	"fmt"
	"os"
)

type direction uint
const (
	Up direction = iota
	Right
	Down
	Left
)

func common(ls [][]int, c *int, i *int, j *int, d direction) {
	if *c == 0 {
		os.Exit(0)
	}
	for k := 0; k < *c; k++ {
		switch d {
			case Up:
				*j--
			case Right:
				*i++
			case Down:
				*j++
			case Left:
				*i--
		}
		fmt.Println(ls[*j][*i])
	}
	*c--
}

func main() {
	matrix := [][]int{{1, 2}, {3, 4}, {5, 6}, {7, 8}}
	fmt.Println(matrix)

	fmt.Println("\n======START======")

	i := -1
	j := 0

	x := len(matrix[0])
	y := len(matrix) - 1

	for true {
		common(matrix, &x, &i, &j, Right)
		common(matrix, &y, &i, &j, Down)
		common(matrix, &x, &i, &j, Left)
		common(matrix, &y, &i, &j, Up)
	}
}