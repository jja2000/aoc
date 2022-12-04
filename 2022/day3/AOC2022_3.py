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
    
    print(prioritySum)
elif sys.argv[2] == '2':
    print("solution for part 2")