import subprocess
from datetime import datetime

while True:
    try:
        subprocess.run(["cargo", "run", "--release"])
    except:
        print(f"failed - {datetime.now()}")
        continue