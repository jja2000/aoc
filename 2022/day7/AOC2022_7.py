import sys

print("Hi! You've used", sys.argv[1], "as input.")
print("You've chosen to calculate the answer for part", sys.argv[2]+".")
with open(sys.argv[1]) as f:
    lines = [item.split() for item in [item.strip() for item in f.readlines()]]

currentPath = []
dirList = []
fileList = []

for l in lines:
    if l[0] == "dir":
        dir = currentPath.copy()
        dir.append(l[1])
        if not dirList.count([dir, 0]):
            dirList.append([dir, 0])
    elif l[1] == "cd" and l[2] == "/":
        currentPath = [l[2]]
        if not dirList.count([currentPath, 0]):
            dirList.append([currentPath.copy(), 0])
    elif l[1] == "cd" and l[2] != "..":
        currentPath.append(l[2])

        if not dirList.count([currentPath, 0]):
            dirList.append([currentPath.copy(), 0])
    elif l[1] == "cd" and l[2] == "..":
        currentPath.pop()
    elif l[0].isdigit():
        file = currentPath.copy()
        file.append(l[1])
        if not fileList.count(file):
            fileList.append((l[0], file))

if sys.argv[2] == '1':
    for f in fileList:
        for i in range(1, len(f[1])):
            for d in dirList:
                if f[1][:-i] == d[0]:
                    d[1] += int(f[0])
    result = 0
    for d in dirList:
        if d[1] <= 100000:
            result += d[1]
    print("The solution for part 1:", result)
elif sys.argv[2] == '2':
    print("The solution for part 2:") 