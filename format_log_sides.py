#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parent
LOG_FILE = ROOT / "log.txt"
LEFT_OUT = ROOT / "left.txt"
RIGHT_OUT = ROOT / "right.txt"


def tokenize(text: str) -> list[str]:
    tokens: list[str] = []
    current: list[str] = []

    for ch in text:
        if ch in "{}[](),":
            if current:
                token = "".join(current).strip()
                if token:
                    tokens.append(token)
                current = []
            tokens.append(ch)
        else:
            current.append(ch)

    if current:
        token = "".join(current).strip()
        if token:
            tokens.append(token)

    return tokens


def pretty_format(text: str) -> str:
    tokens = tokenize(text)
    indent = 0
    lines: list[str] = []
    current_line = ""
    i = 0

    while i < len(tokens):
        tok = tokens[i]

        if tok == "(":
            if current_line:
                current_line += tok
                lines.append(current_line)
            else:
                lines.append("  " * indent + tok)
            indent += 1
            current_line = ""
        elif tok in "[{":
            if current_line:
                current_line += " " + tok
                lines.append(current_line)
            else:
                lines.append("  " * indent + tok)
            indent += 1
            current_line = ""
        elif tok in ")]}":
            if current_line:
                lines.append(current_line)
                current_line = ""
            indent = max(indent - 1, 0)
            line = "  " * indent + tok
            if i + 1 < len(tokens) and tokens[i + 1] == ",":
                line += ","
                i += 1
            lines.append(line)
        elif tok == ",":
            if current_line:
                current_line += ","
                lines.append(current_line)
                current_line = ""
            elif lines:
                lines[-1] += ","
        else:
            if current_line:
                current_line += " " + tok
            else:
                current_line = "  " * indent + tok

        i += 1

    if current_line:
        lines.append(current_line)

    return "\n".join(lines) + "\n"


def extract_part(lines: list[str], label: str) -> str:
    marker = f"{label}:"
    for line in lines:
        if marker in line:
            return line.split(marker, 1)[1].strip()
    raise ValueError(f"Could not find '{label}' entry in log file")


def main() -> int:
    try:
        lines = LOG_FILE.read_text().splitlines()
    except FileNotFoundError:
        print(f"Missing log file: {LOG_FILE}", file=sys.stderr)
        return 1

    left_raw = extract_part(lines, "left")
    right_raw = extract_part(lines, "right")

    LEFT_OUT.write_text(pretty_format(left_raw))
    RIGHT_OUT.write_text(pretty_format(right_raw))

    return 0


if __name__ == "__main__":
    sys.exit(main())
