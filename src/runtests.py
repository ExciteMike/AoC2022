from datetime import date
from subprocess import run

year = 2022
today = date.today()
for i in range (1,32):
    if today >= date.fromisoformat(f"{year}-12-{i:02d}"):
        run(f'cargo test day{i:02d} --bin aoc{year} --quiet')
print('done')