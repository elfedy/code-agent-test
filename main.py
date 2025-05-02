import logging
import datetime

logging.basicConfig(level=logging.INFO)

now = datetime.datetime.now()
logging.info(f"Program started at: {now}")

print("Hello, world!")
print("----------------------")
from math_module.math import sum

num1 = 5
num2 = 10
result = sum(num1, num2)
print(result)