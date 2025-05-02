mport logging
import datetime

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

logging.info(f"Program started at: {datetime.datetime.now()}")

print("Hello, world!")