import logging
import datetime

logging.basicConfig(level=logging.INFO)

now = datetime.datetime.now()
logging.info(f"Program started at: {now}")

print("Hello, world!")