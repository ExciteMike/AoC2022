import sys
import urllib.request

year = 2022
day = int(sys.argv[1])
session_cookie = open("session_cookie", "r").read()
fname = f"puzzle_input/day{day:02}"
f = None
try:
    f = open(fname, "r")
except FileNotFoundError:
    url = f"https://adventofcode.com/{year}/day/{int(day)}/input"
    opener = urllib.request.build_opener()
    opener.addheaders = [("Cookie", f"session={session_cookie}")]
    urllib.request.install_opener(opener)
    urllib.request.urlretrieve(url, fname)
    f = open(fname, "r")