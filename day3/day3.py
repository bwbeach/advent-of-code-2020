
# Read the grid of trees.  Result is a list of rows, from top to bottom.
with open("input.txt") as f:
    tree_rows = f.read().splitlines(keepends=False)

# The trees repeat infinitely to the right.  Instead of replicating
# the data, we'll just use mod arithmetic when indexing, using this
# count as the modulus.  We expect all of the rows to be the same
# length.
column_count = len(tree_rows[0])
assert all(len(row) == column_count for row in tree_rows)

# We use 0-based indexing, so the first position is (0, 0), the next
# is (1, 3), etc.  Check the locations in each row, and count the trees
# encountered.  This is the answer for part 1.
count = sum(
    1
    for i in range(len(tree_rows))
    if tree_rows[i][(i * 3) % column_count] == "#"
)
print(count)

# For part 2, well generalize and factor out a function to compute
# the answer for any (over, down) pair.
def count_trees(over, down):
    return sum(
        1
        for i in range(0, len(tree_rows), down)
        if tree_rows[i][(i * over // down) % column_count] == "#"
    )

# Helper function to multply things
def product(items):
    result = 1
    for x in items:
        result *= x
    return result

# Now, multiple the answers to these inputs
inputs = [
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2),
]
print(
    product(
        count_trees(over, down)
        for over, down in inputs
    )
)
