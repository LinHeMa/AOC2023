import re

with open("./day01input.txt") as f:
    data = f.read().strip()


def calibration(data):
    ls = data.split("\n")
    ns = [re.findall("\d", x) for x in ls]
    return sum(int(n[0] + n[-1]) for n in ns)


# Part 1
print(calibration(data))

# # Part 2
# data = (
#     data.replace("one", "one1one")
#     .replace("two", "two2two")
#     .replace("three", "three3three")
#     .replace("four", "four4four")
#     .replace("five", "five5five")
#     .replace("six", "six6six")
#     .replace("seven", "seven7seven")
#     .replace("eight", "eight8eight")
#     .replace("nine", "nine9nine")
# )

# another way
data = (
    data.replace("one", "o1e")
    .replace("two", "t2o")
    .replace("three", "t3e")
    .replace("four", "f4r")
    .replace("five", "f5e")
    .replace("six", "s6x")
    .replace("seven", "s7n")
    .replace("eight", "e8t")
    .replace("nine", "n9e")
)

print(calibration(data))