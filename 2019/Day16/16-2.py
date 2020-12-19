import numpy
from math import floor

times = 10000
offsetIndexRange = 7

data = [int(i) for i in open("input.txt", "r").read().rstrip()]
values = numpy.tile(data, floor((len(data) * times - int("".join([str(i) for i in data[:offsetIndexRange]])))/len(data)))
values = numpy.concatenate((data[len(data) - (((len(data) * times) - int("".join([str(i) for i in data[:offsetIndexRange]]))) - len(values)):], values))

for _ in range(100):
    values = numpy.array([numpy.sum(values[i:]) % 10 for i in range(len(values))])

print("".join([str(i) for i in values[:8]]))