from typing import List, Tuple


def parse_instruction(line: str) -> Tuple[str, int]:
    """
    Parses one instruction, like "acc +5", returning
    a tuple (op_str, number)
    """
    op, count_str = line.split()
    return op, int(count_str)


def parse_code(text: str) -> List[Tuple[str, int]]:
    """
    Parses a program that is one instruction per line.
    """
    return list(parse_instruction(line) for line in text.splitlines(keepends=False))


def run(code: List[Tuple[str, int]]) -> Tuple[str, int]:
    """
    Runs a program, returning status and accumulator.

    The status can be either "loop" or "halt".
    """
    pc = 0
    acc = 0
    instructions_run = set()
    while True:
        if pc == len(code):
            return "halt", acc
        if pc in instructions_run:
            return "loop", acc
        instructions_run.add(pc)
        op, n = code[pc]
        if op == "acc":
            acc += n
            pc += 1
        elif op == "nop":
            pc += 1
        elif op == "jmp":
            pc += n
        else:
            raise Exception("bad op code: " + op)


def other_one(op: str) -> str:
    if op == "jmp":
        return "nop"
    elif op == "nop":
        return "jmp"
    else:
        raise Exception("BUG")


def fix_code(code: List[Tuple[str, int]]) -> List[Tuple[str, int]]:
    """Fixes a program by swapping a jmp/nop"""
    assert run(code)[0] == "loop"
    answers = []
    for i in range(len(code)):
        op, n = code[i]
        if op in ["jmp", "nop"]:
            code[i] = other_one(op), n
            if run(code)[0] == "halt":
                answers.append(i)
            code[i] = op, n
    assert len(answers) == 1
    result = list(code)
    op, n = result[answers[0]]
    result[answers[0]] = other_one(op), n
    return result


def main():
    with open("input.txt") as f:
        code = parse_code(f.read())
    print("Accumulator at loop:", run(code))
    print("Answer after fixing:", run(fix_code(code)))


if __name__ == "__main__":
    main()
