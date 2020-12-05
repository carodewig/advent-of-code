""" day 2: I was told there would be no math """

def wrapping_paper(l, w, h):
    return 2*l*w + 2*w*h + 2*h*l + min(l*w, l*h, w*h)

def ribbon(l, w, h):
    return 2 * min(l+w, w+h, h+l) + l*w*h

def parse_present_dimensions(present_str):
    return [int(x) for x in present_str.split("x")]


assert wrapping_paper(*parse_present_dimensions("2x3x4")) == 58
assert wrapping_paper(*parse_present_dimensions("1x1x10")) == 43
assert ribbon(*parse_present_dimensions("2x3x4")) == 34
assert ribbon(*parse_present_dimensions("1x1x10")) == 14

PRESENTS = []
with open("data/02.txt") as fh:
    PRESENTS = fh.readlines()

print(sum([wrapping_paper(*parse_present_dimensions(present)) for present in PRESENTS]))
print(sum([ribbon(*parse_present_dimensions(present)) for present in PRESENTS]))
