package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func day09() (string, string) {

	data, err := os.ReadFile("./inputs/09")
	if err != nil {
		panic(err)
	}

	sequence := strings.Trim(string(data), "\n")

	checksum := int64(0)
	forwardDiskMapOffset := 0
	backwardDiskMapOffset := len(sequence) - 1
	backfillId := backwardDiskMapOffset / 2
	backfillSize, _ := strconv.ParseInt(string(sequence[backwardDiskMapOffset]), 10, 16)
	memoryOffset := int64(0)

	for forwardDiskMapOffset <= backwardDiskMapOffset {
		sectionSize, _ := strconv.ParseInt(string(sequence[forwardDiskMapOffset]), 10, 16)
		if forwardDiskMapOffset == backwardDiskMapOffset {
			sectionSize = backfillSize
		}

		if forwardDiskMapOffset%2 == 0 {
			// In a file
			for i := memoryOffset; i < memoryOffset+sectionSize; i += 1 {
				checksum += i * int64(forwardDiskMapOffset/2)
			}
			memoryOffset += sectionSize
		} else {
			// In a blank space
			spacesRemainingToFill := sectionSize
			for spacesRemainingToFill > 0 {
				copySize := min(backfillSize, spacesRemainingToFill)
				for i := memoryOffset; i < memoryOffset+copySize; i += 1 {
					checksum += i * int64(backfillId)
				}
				memoryOffset += copySize
				spacesRemainingToFill -= copySize

				backfillSize -= copySize
				if backfillSize == 0 {
					backwardDiskMapOffset -= 2
					if backwardDiskMapOffset <= forwardDiskMapOffset {
						break
					}
					backfillId = backwardDiskMapOffset / 2
					backfillSize, _ = strconv.ParseInt(string(sequence[backwardDiskMapOffset]), 10, 16)
				}
			}
		}

		forwardDiskMapOffset += 1
	}

	secondChecksum := int64(0)
	memoryOffset = 0

	lastSmallerOrEqualThan := make([]int, 10)
	for n := 0; n < len(lastSmallerOrEqualThan); n += 1 {
		i := len(sequence) - 1
		value, _ := strconv.ParseInt(string(sequence[i]), 10, 16)

		for int(value) > n && i > 0 {
			i -= 2
			value, _ = strconv.ParseInt(string(sequence[i]), 10, 16)
		}

		lastSmallerOrEqualThan[n] = i
	}

	for forwardDiskMapOffset = 0; forwardDiskMapOffset < len(sequence); forwardDiskMapOffset += 1 {
		sectionSize, _ := strconv.ParseInt(string(sequence[forwardDiskMapOffset]), 10, 16)

		if forwardDiskMapOffset%2 == 0 {
			// In a file
			if forwardDiskMapOffset > lastSmallerOrEqualThan[sectionSize] {
				memoryOffset += sectionSize
				continue
			}

			for i := memoryOffset; i < memoryOffset+sectionSize; i += 1 {
				secondChecksum += i * int64(forwardDiskMapOffset/2)
			}
			memoryOffset += sectionSize

		} else {
			// In an empty space

			remainingSpaceInSection := sectionSize
			lastEligibleIndex := lastSmallerOrEqualThan[remainingSpaceInSection]

			for lastEligibleIndex > forwardDiskMapOffset {
				moveSize, _ := strconv.ParseInt(string(sequence[lastEligibleIndex]), 10, 16)

				for i := memoryOffset; i < memoryOffset+moveSize; i += 1 {
					secondChecksum += i * int64(lastEligibleIndex/2)
				}
				memoryOffset += moveSize
				remainingSpaceInSection -= moveSize

				for n := 0; n < len(lastSmallerOrEqualThan); n += 1 {
					if lastSmallerOrEqualThan[n] != lastEligibleIndex {
						continue
					}

					i := lastEligibleIndex

					for {

						i -= 2
						value, _ := strconv.ParseInt(string(sequence[i]), 10, 16)

						lastSmallerContainingValue := lastSmallerOrEqualThan[value]
						if lastSmallerContainingValue < i {
							continue
						}

						if int(value) <= n || i < 0 {
							break
						}
					}

					lastSmallerOrEqualThan[n] = i
				}

				lastEligibleIndex = lastSmallerOrEqualThan[remainingSpaceInSection]

			}

			memoryOffset += remainingSpaceInSection
		}

	}

	return fmt.Sprintf("%d", checksum), fmt.Sprintf("%d", secondChecksum)

}
