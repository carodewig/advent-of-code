""" day 10: elves look, elves say """


def translate_one(string):
    if not string:
        return ""

    if not len(set(string)) == 1:
        raise Exception(f"String must have only one unique characters (ie 1, 11); {string} provided")

    return f"{len(string)}{string[0]}"


def translate(string):
    output_string = ""
    same_chars = ""

    for char in string:
        if same_chars == "" or same_chars[0] == char:
            same_chars += char
        else:
            output_string += translate_one(same_chars)
            same_chars = char

    output_string += translate_one(same_chars)
    return output_string


def chain_translate(times, string):
    output = string
    for _ in range(times):
        output = translate(output)

    return output


assert translate("1") == "11"
assert translate("11") == "21"
assert translate("21") == "1211"
assert translate("1211") == "111221"
assert translate("111221") == "312211"

assert chain_translate(5, "1") == "312211"

print(len(chain_translate(50, "1113222113")))
