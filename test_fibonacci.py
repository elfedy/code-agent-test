import unittest
from fibonacci import FibonacciIterator

class TestFibonacciIterator(unittest.TestCase):

    def test_valid_limit(self):
        fib_iter = FibonacciIterator(5)
        result = list(fib_iter)
        self.assertEqual(result, [0, 1, 1, 2, 3])

    def test_zero_limit(self):
        fib_iter = FibonacciIterator(0)
        result = list(fib_iter)
        self.assertEqual(result, [])

    def test_invalid_limit_type(self):
        with self.assertRaises(TypeError):
            FibonacciIterator("abc")

    def test_negative_limit(self):
        with self.assertRaises(ValueError):
            FibonacciIterator(-1)

if __name__ == '__main__':
    unittest.main()
