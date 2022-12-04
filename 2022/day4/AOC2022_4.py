import sys

print("Hi! You've used", sys.argv[1], "as input.")
print("You've chosen to calculate the answer for part", sys.argv[2]+".")
with open(sys.argv[1]) as f:
    lines = [item.strip() for item in f.readlines()]

answer = 0
for l in lines:
    range1 = [*range(int(l.split(',')[0].split('-')[0]), int(l.split(',')[0].split('-')[1])+1)]
    range2 = [*range(int(l.split(',')[1].split('-')[0]), int(l.split(',')[1].split('-')[1])+1)]
    if sys.argv[2] == '1':
        if all(item in range1 for item in range2) or all(item in range2 for item in range1):
            answer += 1
    elif sys.argv[2] == '2':
        if any(item in range1 for item in range2) or any(item in range2 for item in range1):
            answer += 1

print("The answer for this part is:", answer)