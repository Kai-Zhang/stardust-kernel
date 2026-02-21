#!/usr/bin/env python3
"""Validate ROADMAP done milestones have Layer D URL evidence in review docs."""

from __future__ import annotations

import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
ROADMAP = ROOT / "docs" / "milestones" / "ROADMAP.md"
REVIEWS = ROOT / "docs" / "reviews"

DONE_RE = re.compile(r"- status:\s*`done`")
ID_RE = re.compile(r"- id:\s*`([^`]+)`")
TOP_LEVEL_HEADER_RE = re.compile(r"^###\s+M\d+[A-Za-z]?\s*$")
URL_RE = re.compile(r"https://github\.com/[^\s)]+/actions/runs/\d+")

# M0 predates release gate policy and has no review artifact.
POLICY_EXEMPT = {"M0-workspace-bootstrap"}


def parse_milestones(text: str) -> list[tuple[str, str]]:
    milestones: list[tuple[str, str]] = []
    current_id: str | None = None
    in_top_level_block = False

    for line in text.splitlines():
        if TOP_LEVEL_HEADER_RE.match(line.strip()):
            in_top_level_block = True
            current_id = None
            continue

        if line.startswith("### ") and not TOP_LEVEL_HEADER_RE.match(line.strip()):
            in_top_level_block = False
            current_id = None
            continue

        if not in_top_level_block:
            continue

        id_match = ID_RE.search(line)
        if id_match:
            current_id = id_match.group(1)
            continue

        if current_id and DONE_RE.search(line):
            milestones.append((current_id, "done"))
            current_id = None

    return milestones


def review_dir_name(milestone_id: str) -> str:
    prefix, _, rest = milestone_id.partition("-")
    if not rest or not prefix.startswith("M"):
        raise ValueError(f"Unexpected milestone id format: {milestone_id}")
    return f"{prefix}-{rest}"


def main() -> int:
    errors: list[str] = []
    milestones = parse_milestones(ROADMAP.read_text(encoding="utf-8"))

    for milestone_id, _ in milestones:
        if milestone_id in POLICY_EXEMPT:
            continue

        review_path = REVIEWS / review_dir_name(milestone_id) / "review.md"
        if not review_path.exists():
            errors.append(f"{milestone_id}: missing review file {review_path.relative_to(ROOT)}")
            continue

        content = review_path.read_text(encoding="utf-8")
        has_layer_d_section = (
            "## Layer D Release Gate Evidence" in content
            or "## Layer D Release Evidence" in content
        )
        if not has_layer_d_section:
            errors.append(
                f"{milestone_id}: missing Layer D evidence section "
                "('Layer D Release Gate Evidence' or 'Layer D Release Evidence')"
            )
            continue

        urls = URL_RE.findall(content)
        if len(urls) < 2:
            errors.append(f"{milestone_id}: expected >=2 GitHub Actions run URLs, found {len(urls)}")

    if errors:
        print("milestone evidence gate: FAIL")
        for err in errors:
            print(f"- {err}")
        return 1

    print("milestone evidence gate: PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
