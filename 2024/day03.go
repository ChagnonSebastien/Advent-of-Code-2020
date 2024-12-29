package main

import (
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

func day03() (string, string) {

	data, err := os.ReadFile("./inputs/03")
	if err != nil {
		panic(err)
	}

	doRegex, err := regexp.Compile(`do\(\)`)
	if err != nil {
		panic(err)
	}

	dontRegex, err := regexp.Compile(`don't\(\)`)
	if err != nil {
		panic(err)
	}

	regex, err := regexp.Compile(`mul\(\d{1,3},\d{1,3}\)`)
	if err != nil {
		panic(err)
	}

	var sum int64 = 0
	var enabledSum int64 = 0

	dos := doRegex.FindAllIndex(data, -1)
	donts := dontRegex.FindAllIndex(data, -1)
	matches := regex.FindAllIndex(data, -1)
	for _, match := range matches {
		numbers := strings.Split(string(data[match[0]+4:match[1]-1]), ",")

		a, err := strconv.ParseInt(numbers[0], 10, 32)
		if err != nil {
			panic(err)
		}
		b, err := strconv.ParseInt(numbers[1], 10, 32)
		if err != nil {
			panic(err)
		}

		lastDo := 0
		for _, doMatch := range dos {
			if doMatch[0] < match[0] {
				lastDo = doMatch[0]
			} else {
				break
			}
		}

		lastDont := -1
		for _, dontMatch := range donts {
			if dontMatch[0] < match[0] {
				lastDont = dontMatch[0]
			} else {
				break
			}
		}

		sum += a * b
		if lastDo > lastDont {
			enabledSum += a * b
		}
	}

	return fmt.Sprintf("%d", sum), fmt.Sprintf("%d", enabledSum)

}
