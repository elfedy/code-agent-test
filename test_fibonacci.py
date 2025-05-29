import unittest
from fibonacci import Fibonacci

class TestFibonacci(unittest.TestCase):

    def test_fibonacci_basic(self):
        fib = Fibonacci(limit=5)
        self.assertEqual(list(fib), [0, 1, 1, 2, 3])

    def test_fibonacci_no_limit(self):
        fib = Fibonacci()
        iterator = iter(fib)
        results = []
        for _ in range(10):
            results.append(next(iterator))
        self.assertEqual(results, [0, 1, 1, 2, 3, 5, 8, 13, 21, 34])

    def test_fibonacci_overflow(self):
        fib = Fibonacci()
        iterator = iter(fib)
        with self.assertRaises(StopIteration):
            while True:
                next(iterator)

if __name__ == '__main__':
    unittest.main()
