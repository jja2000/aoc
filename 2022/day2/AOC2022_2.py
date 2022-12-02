import sys,time

gameList = []

print("Hi! You've used", sys.argv[1], "as input.")
print("You've chosen to calculate the answer for part", sys.argv[2]+".")
with open(sys.argv[1]) as f:
    lines = [item.strip() for item in f.readlines()]

for c in lines:
    gameList.append((c.split()[0], c.split()[1]))

if sys.argv[2] == '1':
    totalPoints = 0
    for game in gameList:
        if game[1] == 'X':
            totalPoints += 1
            if game[0] == 'A':
                totalPoints += 3
            elif game[0] == 'B':
                totalPoints += 0
            elif game[0] == 'C':
                totalPoints += 6
            else:
                print("Oop! Something else went wrong (I think)")
        elif game[1] == 'Y':
            totalPoints += 2
            if game[0] == 'A':
                totalPoints += 6
            elif game[0] == 'B':
                totalPoints += 3
            elif game[0] == 'C':
                totalPoints += 0
            else:
                print("Oop! Something else went wrong (I think)")
        elif game[1] == 'Z':
            totalPoints += 3
            if game[0] == 'A':
                totalPoints += 0
            elif game[0] == 'B':
                totalPoints += 6
            elif game[0] == 'C':
                totalPoints += 3
            else:
                print("Oop! Something else went wrong (I think)")
        else:
            print("Oop! Something went wrong (I think)")
    print("Total amount of points is:", totalPoints)
elif sys.argv[2] == '2':
    totalPoints = 0
    for game in gameList:
        if game[1] == 'X':
            totalPoints += 0
            if game[0] == 'A':
                totalPoints += 3
            elif game[0] == 'B':
                totalPoints += 1
            elif game[0] == 'C':
                totalPoints += 2
            else:
                print("Oop! Something else went wrong (I think)")
        elif game[1] == 'Y':
            totalPoints += 3
            if game[0] == 'A':
                totalPoints += 1
            elif game[0] == 'B':
                totalPoints += 2
            elif game[0] == 'C':
                totalPoints += 3
            else:
                print("Oop! Something else went wrong (I think)")
        elif game[1] == 'Z':
            totalPoints += 6
            if game[0] == 'A':
                totalPoints += 2
            elif game[0] == 'B':
                totalPoints += 3
            elif game[0] == 'C':
                totalPoints += 1
            else:
                print("Oop! Something else went wrong (I think)")
        else:
            print("Oop! Something went wrong (I think)")
    print("Total amount of points is:", totalPoints)