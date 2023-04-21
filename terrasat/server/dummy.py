import requests
import time

# satellite apiserver configuration
# this will need to be updated with a new ip address
url = "http://localhost:8000/pulse/6"

while (True):
    x = requests.put(url)
    time.sleep(2)