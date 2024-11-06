import re


def process_tag(tag: str) -> str:
    """Convert a git tag to a version string compliant with PEP 440.
    See https://peps.python.org/pep-0440/#public-version-identifiers
    """
    pattern = (
        # note that this pattern accepts a superset of the tagging pattern used
        # in this repository.
        r"^v(?P<major>\d+)"
        r"\.(?P<minor>\d+)"
        r"\.(?P<patch>\d+)"
        r"(-"
        r"(?P<prerelease_kind>alpha|beta|rc)"
        r"\.?(?P<prerelease_number>\d+)"
        ")?"
    )
    parts = re.match(pattern, tag)
    if parts is None:
        raise ValueError(f"Invalid tag: `{tag}` does not match pattern `{pattern}`")
    major = int(parts["major"])
    minor = int(parts["minor"])
    patch = int(parts["patch"])
    suffix = ""

    if (prerelease_kind := parts["prerelease_kind"]) is not None:
        if prerelease_kind == "rc":
            suffix = "rc"
        elif prerelease_kind.startswith("alpha"):
            suffix = "a"
        elif prerelease_kind.startswith("beta"):
            suffix = "b"
    if (prerelease_number := parts["prerelease_number"]) is not None:
        suffix += str(int(prerelease_number))

    return f"{major}.{minor}.{patch}{suffix}"
