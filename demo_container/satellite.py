import requests
import time

# satellite apiserver configuration
url = "http://status:5001/status"
status = {
    "statusx":"ok",
}

while (True):
    x = requests.post(url, json = status)
    time.sleep(2)