def findValue(output):
    for noun in range(100):
        for verb in range(100):
            data = [int(x) for x in open("input.txt", "r").read().split(',')]
            data[1] = noun
            data[2] = verb

            i = 0
            while data[i] != 99:
                if data[i] == 1:
                    data[data[i+3]] = data[data[i+1]] + data[data[i+2]]
                else:
                    data[data[i+3]] = data[data[i+1]] * data[data[i+2]]

                i += 4
            
            if data[0] == output:
                return 100 * noun + verb

print(findValue(19690720))