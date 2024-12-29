package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func isSafe(values []string) bool {

	prevValue, err := strconv.ParseInt(values[0], 10, 32)
	if err != nil {
		panic(err)
	}
	prevChange := int64(0)
	initiated := false
	safe := true
	for _, n := range values[1:] {
		value, err := strconv.ParseInt(n, 10, 32)
		if err != nil {
			panic(err)
		}

		change := value - prevValue
		if math.Abs(float64(change)) < 1 || math.Abs(float64(change)) > 3 || initiated && prevChange*change < 0 {
			safe = false
			break
		}

		initiated = true
		prevValue = value
		prevChange = change

	}

	return safe
}

func day02() (string, string) {

	data, err := os.ReadFile("./inputs/02")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.Trim(string(data), "\n"), "\n")

	safeReport := 0
	salvageable := 0

	for _, report := range lines {
		values := strings.Split(report, " ")

		if isSafe(values) {
			safeReport += 1
		} else {
			for i := 0; i < len(values); i += 1 {

				c := make([]string, i)
				_ = copy(c, values[:i])
				c = append(c, values[i+1:]...)
				if isSafe(c) {
					salvageable += 1
					break
				}
			}
		}
	}

	return fmt.Sprintf("%d", safeReport), fmt.Sprintf("%d", safeReport+salvageable)

}
