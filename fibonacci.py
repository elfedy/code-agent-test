class FibonacciIterator:
    def __init__(self, limit: int):
        if not isinstance(limit, int):
            raise TypeError("Limit must be an integer")
        if limit < 0:
            raise ValueError("Limit must be a non-negative integer")
        self.limit = limit
        self.a = 0
        self.b = 1
        self.count = 0

    def __iter__(self):
        return self

    def __next__(self):
        if self.count < self.limit:
            result = self.a
            self.a, self.b = self.b, self.a + self.b
            self.count += 1
            return result
        else:
            raise StopIteration
