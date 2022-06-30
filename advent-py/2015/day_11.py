""" day 11: corporate policy """

import re

STRAIGHTS_OF_LETTERS = ["".join([chr(i), chr(i + 1), chr(i + 2)]) for i in range(ord("a"), ord("a") + 24)]

BANNED_LETTERS = re.compile(r"[iol]")
DOUBLE_LETTERS = re.compile(r"(.)\1")


def increment_password(_password):
    password = list(_password)
    for i in reversed(range(len(password))):
        new_letter_ord = ord(password[i]) + 1
        if new_letter_ord <= ord("z"):
            password[i] = chr(new_letter_ord)
            return "".join(password)

        password[i] = "a"

    return "".join(password)


def straight_present(password):
    for straight in STRAIGHTS_OF_LETTERS:
        if straight in password:
            return True

    return False


def no_banned_letters_present(password):
    return not bool(re.search(BANNED_LETTERS, password))


def pairs_present(password):
    return len(set(re.findall(DOUBLE_LETTERS, password))) >= 2


def valid_password(password):
    return no_banned_letters_present(password) and pairs_present(password) and straight_present(password)


def next_valid_password(password):
    new_password = password
    while new_password := increment_password(new_password):
        if valid_password(new_password):
            return new_password


assert increment_password("xx") == "xy"
assert increment_password("xy") == "xz"
assert increment_password("xz") == "ya"
assert increment_password("ya") == "yb"
assert increment_password("yb") == "yc"

assert straight_present("hijklmmn")
assert not no_banned_letters_present("hijklmmn")
assert pairs_present("abbceffg")
assert not straight_present("abbceffg")
assert not pairs_present("abbcegjk")

assert next_valid_password("abcdefgh") == "abcdffaa"
assert next_valid_password("ghijklmn") == "ghjaabcc"

assert next_valid_password("vzbxkghb") == "vzbxxyzz"
assert next_valid_password("vzbxxyzz") == "vzcaabcc"
