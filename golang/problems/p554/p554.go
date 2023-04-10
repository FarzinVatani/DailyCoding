package main

import (
	"fmt"
)

func egyptianFraction(numenator int, denominator int) {
	if denominator == 0 || numenator == 0 || numenator > denominator {
		return
	}

	if denominator%numenator == 0 {
		fmt.Printf("1 / %d\n", denominator/numenator)
		return
	}

	d := denominator/numenator + 1
	fmt.Printf("1 / %d\n", d)

	egyptianFraction(d*numenator-denominator, d*denominator)
}

func main() {
	nr := 4
	dr := 13

	egyptianFraction(nr, dr)
}
