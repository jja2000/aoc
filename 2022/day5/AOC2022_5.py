import sys

print("Hi! You've used", sys.argv[1], "as input.")
print("You've chosen to calculate the answer for part", sys.argv[2]+".")
with open(sys.argv[1]) as f:
    lines = f.readlines()

stacksAmount = None
lastInitialLine = None
initialized = False
stack = []
stacks = []

for l in lines:
    if any(char.isdigit() for char in l) and l.startswith(' '):
        lastInitialLine = lines.index(l)
        digits = [int(x) for x in l.split() if x.isdigit()]
        stacksAmount = max(digits) if digits else None
        for i in range(0, stacksAmount):
            stack = []
            for j in range(lastInitialLine - 1, -1, -1):
                if lines[j][1+4*i] != ' ':
                    crate = lines[j][1+4*i]
                    stack.append(crate)
            stacks.append(stack)
        initialized = True
    elif initialized == True and l != "\n":
        command = l.split()
        if sys.argv[2] == '1':
            for i in range(0,int(command[1])):
                crate = stacks[int(command[3])-1][-1]
                stacks[int(command[3])-1].pop()
                stacks[int(command[5])-1].append(crate)
        elif sys.argv[2] == '2':
            grabbed = stacks[int(command[3])-1][-int(command[1]):]
            del stacks[int(command[3])-1][len(stacks[int(command[3])-1]) - int(command[1]):]
            stacks[int(command[5])-1] += grabbed

solution = []

if sys.argv[2] == '1':
    for i in range(0, stacksAmount):
        solution.append(stacks[i][-1])
    print("The solution for part 1 is:", *solution)
elif sys.argv[2] == '2':
    for i in range(0, stacksAmount):
        solution.append(stacks[i][-1])
    print("The solution for part 2 is:", *solution)