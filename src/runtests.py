from datetime import date
from subprocess import run

today = date.today()
for i in range (1,32):
    if today >= date.fromisoformat(f"2022-12-{i:02d}"):
        run(f'cargo test --bin day{i:02d} --quiet')
print('done')