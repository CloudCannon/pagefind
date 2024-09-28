"""
Prepend wrappers/python/src to PYTHONPATH.
"""

import os
from pathlib import Path


new_pythonpath = str(Path("src").absolute())
if old_pythonpath := os.environ.get("PYTHONPATH"):
    new_pythonpath = os.pathsep.join(
        [  # os.pathsep is ":" for unix, ";" for windows
            new_pythonpath,
            old_pythonpath,
        ]
    )

with open(os.environ["GITHUB_ENV"], "a") as f:
    f.write(f"PYTHONPATH={new_pythonpath}\n")
