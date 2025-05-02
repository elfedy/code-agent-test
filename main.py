import logging
import datetime

logging.basicConfig(level=logging.INFO)

now = datetime.datetime.now()
logging.info(f"Program started at: {now}")

print("Hello, world!")
import math

num1 = 10
num2 = 5
sum_result = math.fsum([num1, num2])
print(f"The sum of {num1} and {num2} is: {sum_result}")