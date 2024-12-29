package main

import (
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

func day01() (string, string) {

	data, err := os.ReadFile("./inputs/01")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.Trim(string(data), "\n"), "\n")

	firstList := make([]int64, 0)
	secondList := make([]int64, 0)

	for _, line := range lines {
		numbers := strings.Split(line, "   ")
		a, err := strconv.ParseInt(numbers[0], 10, 32)
		if err != nil {
			panic(err)
		}
		b, err := strconv.ParseInt(numbers[1], 10, 32)
		if err != nil {
			panic(err)
		}

		firstList = append(firstList, a)
		secondList = append(secondList, b)
	}

	slices.SortFunc(firstList, func(i, j int64) int {
		return int(i - j)
	})

	slices.SortFunc(secondList, func(i, j int64) int {
		return int(i - j)
	})

	sum := 0
	for i := 0; i < len(firstList); i += 1 {
		sum += int(math.Abs(float64(firstList[i] - secondList[i])))
	}

	amounts := make(map[int64]int)

	for _, n := range secondList {
		prev := amounts[n]
		amounts[n] = prev + 1
	}

	similarityScore := 0

	for _, n := range firstList {
		similarityScore += int(n) * amounts[n]
	}

	return fmt.Sprintf("%d", sum), fmt.Sprintf("%d", similarityScore)

}
