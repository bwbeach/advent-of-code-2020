
with open('input.txt') as f:
    numbers = [
        int(word)
        for word in f.read().split()
    ]

for i in range(len(numbers)):
    a = numbers[i]
    for j in range(i + 1, len(numbers)):
        b = numbers[j]
        for k in range(j + 1, len(numbers)):
            c = numbers[k]
            if a + b + c == 2020:
                print(a, b, c, a * b * c)
