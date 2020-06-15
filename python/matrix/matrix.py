class Matrix:
    def __init__(self, matrix_string):
        def parse_row(row):
            return list(map(lambda x: int(x), row.split(" ")))

        self.rows = list(map(parse_row, matrix_string.split("\n"))) 

    def row(self, index):
        return self.rows[index - 1]

    def column(self, index):
        return [row[index - 1] for row in self.rows]
