import sys, numpy

print("Hi! You've used", sys.argv[1], "as input.")
print("You've chosen to calculate the answer for part", sys.argv[2]+".")
with open(sys.argv[1]) as f:
    line = f.readline().strip()

if sys.argv[2] == '1':
    n = 4
elif sys.argv[2] == '2':
    n = 14
window = [None] * n

for i, l in enumerate(line):
        window.append(l)
        window.pop(0)
        if(len(set(window)) == len(window) and not window.count(None)):
            print("Unique window", window, "found at:", i + 1)
            break