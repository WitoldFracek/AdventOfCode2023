from qwlist import QList

print(sum(map(lambda x: 1 << x - 1, filter(lambda x: x > 0, map(lambda s: len({int(e) for e in s[0].split()} & {int(e) for e in s[1].split()}), map(lambda s: (s[0].split(':')[1].strip(), s[1].strip()), map(lambda l: l.split('|'), open('4'))))))))

print(
    QList(open('4'))
    .map(lambda l: l.split('|'))
    .map(lambda s: (s[0].split(':')[1].strip(), s[1].strip()))
    .map(lambda s: len({int(e) for e in s[0].split()} & {int(e) for e in s[1].split()}))
    .filter(lambda x: x > 0)
    .map(lambda x: 2 ** (x - 1))
    .fold(lambda acc, x: acc + x, 0)
)


with open('data4.txt') as file:
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
