from day8.day8 import fix_code, parse_code, run

SAMPLE_CODE_LOOP = """nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"""

SAMPLE_CODE_HALT = """nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6
"""


def test_parse():
    assert parse_code("nop +0\nacc +1\nacc -6") == [("nop", 0), ("acc", 1), ("acc", -6)]


def test_run_loop():
    code = parse_code(SAMPLE_CODE_LOOP)
    assert run(code) == ("loop", 5)


def test_run_halt():
    code = parse_code(SAMPLE_CODE_HALT)
    assert run(code) == ("halt", 8)


def test_fix_code():
    assert fix_code(parse_code(SAMPLE_CODE_LOOP)) == parse_code(SAMPLE_CODE_HALT)
