package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func test(remainder int, numbers []int, allowConcat bool) bool {
	if len(numbers) == 0 {
		return remainder == 0
	}

	current := numbers[len(numbers)-1]
	if remainder < current {
		return false
	}

	if allowConcat && strings.HasSuffix(fmt.Sprint(remainder), fmt.Sprint(current)) {
		newRemainder, _ := strconv.ParseInt(strings.TrimSuffix(fmt.Sprint(remainder), fmt.Sprint(current)), 10, 64)
		if testResult := test(int(newRemainder), numbers[:len(numbers)-1], allowConcat); testResult {
			return true
		}
	}

	if remainder%current == 0 {
		if testResult := test(remainder/current, numbers[:len(numbers)-1], allowConcat); testResult {
			return true
		}
	}

	return test(remainder-current, numbers[:len(numbers)-1], allowConcat)
}

func day07() (string, string) {

	data, err := os.ReadFile("./inputs/07")
	if err != nil {
		panic(err)
	}

	lines := strings.Split(strings.Trim(string(data), "\n"), "\n")

	var sum int64
	var sumWithConcat int64
	for _, line := range lines {
		parts := strings.Split(line, ": ")
		testResult, _ := strconv.ParseInt(parts[0], 10, 64)
		numbers := make([]int, 0)
		for _, element := range strings.Split(parts[1], " ") {
			number, _ := strconv.ParseInt(element, 10, 64)
			numbers = append(numbers, int(number))
		}

		if valid := test(int(testResult), numbers, false); valid {
			sum += testResult
		}

		if valid := test(int(testResult), numbers, true); valid {
			sumWithConcat += testResult
		}
	}

	return fmt.Sprintf("%d", sum), fmt.Sprintf("%d", sumWithConcat)

}
