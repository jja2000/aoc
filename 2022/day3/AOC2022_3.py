import sys, string

def splitInput(item):
    return (item[:int(len(item)/2)], item[int(len(item)/2):])

letters = list(string.ascii_letters)

rucksacks = []

prioritySum = 0

print("Hi! You've used", sys.argv[1], "as input.")
print("You've chosen to calculate the answer for part", sys.argv[2]+".")
with open(sys.argv[1]) as f:
    lines = [item.strip() for item in f.readlines()]

if sys.argv[2] == '1':
    for l in lines:
        rucksacks.append(splitInput(l))

    for r in rucksacks:
        for l in letters:
            if r[0].count(l) and r[1].count(l) > 0:
                prioritySum += letters.index(l) + 1
elif sys.argv[2] == '2':
    index = 0
    groups = []
    group = []
    for l in lines:
        index += 1
        group.append(l)
        if index == 3:
            index = 0
            groups.append(group)
            group = []
    
    for g in groups:
        for l in letters:
            if g[0].count(l) and g[1].count(l) and g[2].count(l) > 0:
                prioritySum += letters.index(l) + 1

print(prioritySum)