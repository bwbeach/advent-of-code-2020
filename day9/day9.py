from typing import List


def parse_numbers(text: str) -> List[int]:
    return list(int(item) for item in text.split())


def find_non_sum(numbers: List[int], window: int) -> int:
    answers = []
    for i in range(window, len(numbers)):
        any_match = False
        for j in range(i - window, i - 1):
            for k in range(j + 1, i):
                if numbers[j] + numbers[k] == numbers[i]:
                    any_match = True
        if not any_match:
            answers.append(numbers[i])
    assert len(answers) == 1
    return answers[0]


def find_contiguous_sum(numbers: List[int], total: int) -> List[int]:
    answers = []
    for i in range(len(numbers)):
        for j in range(i + 1, len(numbers)):
            candidate = numbers[i : j + 1]
            s = sum(candidate)
            if s == total:
                answers.append(candidate)
            if total < s:
                break
    assert len(answers) == 1
    return answers[0]


def main():
    with open("input.txt") as f:
        numbers = parse_numbers(f.read())
    answer_1 = find_non_sum(numbers, 25)
    print("Unique non-sum:", answer_1)
    seq = find_contiguous_sum(numbers, answer_1)
    print("Second answer:", min(seq) + max(seq))


if __name__ == "__main__":
    main()
