import re

lines = [line for line in open("Day19/input", "r").read().strip().split("\n")]
rules = { line.split(": ")[0]: line.split(": ")[1] for line in lines[:lines.index('')] }
messages = [line for line in lines[lines.index('')+1:]]

rules.update({ "8": "42+" })
rules.update({ "11": "42 31 | 42 42 31 31 | 42 42 42 31 31 31 | 42 42 42 42 31 31 31 31 | 42 42 42 42 42 31 31 31 31 31" })

regex = "0"
while re.search(r'[0-9]+', regex):
    match = re.search(r'[0-9]+', regex)
    regex = regex.replace(match.group(), "(" + rules.get(match.group()) + ")", 1)
regex = regex.replace('("', '').replace('")', '').replace(' ', '')

print(len([message for message in messages if re.match("^" + regex + "$", message)]))
