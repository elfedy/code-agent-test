import unittest

# Assuming the Fibonacci iterator is in a file named fibonacci.py
# and the iterator class is named FibonacciIterator
from fibonacci import FibonacciIterator

class TestFibonacciIterator(unittest.TestCase):

    def test_first_10_numbers(self):
        fib_iterator = FibonacciIterator()
        expected_numbers = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        actual_numbers = [next(fib_iterator) for _ in range(10)]
        self.assertEqual(actual_numbers, expected_numbers)


