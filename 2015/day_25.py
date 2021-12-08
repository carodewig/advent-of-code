""" day 25: let it snow """


def seq_number(row, col):
    def seq_number_col1(row):
        index = 1
        row_num = 1
        while row_num != row:
            row_num += 1
            index += row_num - 1

        return index

    row_for_diagonal = col - 1 + row
    return seq_number_col1(row_for_diagonal) + col - 1


def code(row, col):
    seq = seq_number(row, col)

    index = 1
    value = 20151125

    while index < seq:
        value = (value * 252533) % 33554393
        index += 1

    return value


assert code(2947, 3029) == 19980801
