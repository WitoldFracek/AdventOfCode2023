with open('../data/data4.txt') as file:
    lines = file.readlines()

total = 0
for line in lines:
    winning, guessed = line.split('|')
    winning: str = winning.split(':')[1].strip()
    guessed: str = guessed.strip()

    win_set = set()
    for num in winning.split():
        if num != '':
            win_set.add(int(num))

    guess_set = set()
    for num in guessed.split():
        if num != '':
            guess_set.add(int(num))

    n = len(win_set & guess_set)
    if n > 0:
        total += 2 ** (n - 1)
print(total)
