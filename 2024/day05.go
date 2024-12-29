package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func day05() (string, string) {

	data, err := os.ReadFile("./inputs/05")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.Trim(string(data), "\n"), "\n")

	emptyLineIndex := 0
	for i, line := range lines {
		if line == "" {
			emptyLineIndex = i
			break
		}
	}

	instructionLines := lines[:emptyLineIndex]
	orderLines := lines[emptyLineIndex+1:]

	rules := make(map[int64]map[int64]bool)
	for _, inst := range instructionLines {
		numbers := strings.Split(inst, "|")
		a, _ := strconv.ParseInt(numbers[0], 10, 32)
		b, _ := strconv.ParseInt(numbers[1], 10, 32)

		mustPrecede, ok := rules[b]
		if !ok {
			mustPrecede = make(map[int64]bool, 0)
			rules[b] = mustPrecede
		}

		mustPrecede[a] = true
	}

	var correctOrders int64
	var incorrectOrders int64

	for _, order := range orderLines {

		valid := true
		visited := make([]int64, 0)
		numbers := strings.Split(order, ",")

	restartOrder:
		for p := 0; p < len(numbers); {
			part := numbers[p]
			n, _ := strconv.ParseInt(part, 10, 32)

			for i, v := range visited {
				mustPrecede := rules[v]
				if _, ok := mustPrecede[n]; ok {
					valid = false

					c := make([]string, i)
					_ = copy(c, numbers[:i])
					c = append(c, part)
					c = append(c, numbers[i:p]...)
					c = append(c, numbers[p+1:]...)
					numbers = c

					visited = make([]int64, 0)
					p = 0
					continue restartOrder
				}
			}

			p += 1
			visited = append(visited, n)
		}

		middleNumber := numbers[(len(numbers)-1)/2]
		value, _ := strconv.ParseInt(middleNumber, 10, 32)
		if valid {
			correctOrders += value
		} else {
			incorrectOrders += value
		}

	}

	return fmt.Sprintf("%d", correctOrders), fmt.Sprintf("%d", incorrectOrders)

}
