""" day 12: jsabacusframework.io """

import json


def nested_sum(obj):
    if isinstance(obj, int):
        return obj

    if isinstance(obj, str):
        return 0

    if isinstance(obj, list):
        return sum([nested_sum(x) for x in obj])

    if isinstance(obj, dict):
        if "red" in obj.values():
            return 0

        return sum([nested_sum(x) for x in obj.values()])

    raise Exception(f"Unsupported type {type(obj)}")


assert nested_sum(json.loads("[1,2,3]")) == 6
assert nested_sum(json.loads('{"a":2,"b":4}')) == 6
assert nested_sum(json.loads("[[[3]]]")) == 3
assert nested_sum(json.loads('{"a":{"b":4},"c":-1}')) == 3
assert nested_sum(json.loads('{"a":[-1,1]}')) == 0
assert nested_sum(json.loads('[-1,{"a":1}]')) == 0
assert nested_sum(json.loads("[]")) == 0
assert nested_sum(json.loads("{}")) == 0

assert nested_sum(json.loads('[1,{"c":"red","b":2},3]')) == 4
assert nested_sum(json.loads('{"d":"red","e":[1,2,3,4],"f":5}')) == 0
assert nested_sum(json.loads('[1,"red",5]')) == 6


with open("data/12.txt") as fh:
    print(nested_sum(json.load(fh)))
