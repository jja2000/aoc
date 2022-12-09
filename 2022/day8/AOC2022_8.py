import sys, time

print("Hi! You've used", sys.argv[1], "as input.")
print("You've chosen to calculate the answer for part", sys.argv[2]+".")
with open(sys.argv[1]) as f:
    lines = [item.strip() for item in f.readlines()]

result = 0
if sys.argv[2] == '1':
    result += 2 * len(lines) + 2 * len(lines[0]) - 4
    for i,y in enumerate(lines):
        if i != 0 and i != len(lines) - 1:
            for j,x in enumerate(lines[i]):
                if j != 0 and j != len(lines[i]) -1:
                    xNegTripped = False
                    xPosTripped = False
                    yNegTripped = False
                    yPosTripped = False
                    for k in range(0, len(lines)):
                        if k < i:
                            if lines[i][j] <= lines[k][j]:
                                yNegTripped = True
                                continue
                        elif k > i:
                            if lines[i][j] <= lines[k][j]:
                                yPosTripped = True
                                continue
                    
                    for l in range (0, len(lines[i])):
                        if l < j:
                            if lines[i][j] <= lines[i][l]:
                                xNegTripped = True
                                continue
                        elif l > j:
                            if lines[i][j] <= lines[i][l]:
                                xPosTripped = True
                                continue
                    if not xNegTripped or not xPosTripped or not yNegTripped or not yPosTripped:
                        result += 1
elif sys.argv[2] == '2':
    for i,y in enumerate(lines):
        if i != 0 and i != len(lines) - 1:
            for j,x in enumerate(lines[i]):
                if j != 0 and j != len(lines[i]) -1:
                    yNegView = 0
                    yPosView = 0
                    xNegView = 0
                    xPosView = 0
                    for k in range(i - 1, -1, -1):
                        yNegView += 1
                        if lines[i][j] <= lines[k][j]:
                            break

                    for l in range(i + 1, len(lines)):
                        yPosView += 1
                        if lines[i][j] <= lines[l][j]:
                            break

                    for m in range(j - 1, -1, -1):
                        xNegView += 1
                        if lines[i][j] <= lines[i][m]:
                            break
                    
                    for n in range(j + 1, len(lines[i])):
                        xPosView += 1
                        if lines[i][j] <= lines[i][n]:
                            break
                    
                    newResult = yNegView * yPosView * xNegView * xPosView
                    if result < newResult:
                        result = newResult
print("The solution for part", sys.argv[2]+":", result)