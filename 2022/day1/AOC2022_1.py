import sys

calorieList = [0]

print("Hi! You've used", sys.argv[1], "as input.")
print("You've chosen to calculate the answer for part", sys.argv[2]+".")
with open(sys.argv[1]) as f:
    lines = [item.strip() for item in f.readlines()]

elfIndex = 0
for c in lines:
    if c == '':
        calorieList.append(0)
        elfIndex+=1
    else:
        calorieList[elfIndex] += int(c)

calorieList.sort(reverse=True)

if sys.argv[2] == '1':
    calories = calorieList[0]
    print("The Elf with the most calories has", calories)
elif sys.argv[2] == '2':
    print("The elves with the top 3 amount of calories have:")
    top3Total = 0
    for x in range(3):
        print(calorieList[x])
        top3Total +=calorieList[x]
    print("Which is", top3Total, "in total.")