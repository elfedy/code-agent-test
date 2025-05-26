lass FibonacciIterator:
    def __init__(self):
        self.a = 0
        self.b = 1

    def __iter__(self):
        return self

    def __next__(self):
        fib_number = self.a
        self.a, self.b = self.b, self.a + self.b
        return fib_number

# Example usage
fib_iter = FibonacciIterator()
for _ in range(10):
    print(next(fib_iter))