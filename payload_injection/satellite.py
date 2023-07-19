import requests
import time

# satellite apiserver configuration
url = "http://status:8000/pulse/3"

while (True):
    x = requests.post(url)
    time.sleep(2)