#!/usr/bin/env python3
"""Validate the sporePrint entity registry and content front matter.

Runs before zola build to catch data problems the Zola compiler can't:
  - Missing required fields per entity kind
  - Taxonomy tags in content that don't match registry keys
  - Registry keys that no content page references
  - Stale aggregate totals

Exit 0 = clean, exit 1 = errors found.
"""

import sys
import re
from pathlib import Path

try:
    import tomllib
except ImportError:
    import tomli as tomllib  # Python < 3.11

ROOT = Path(__file__).resolve().parent.parent
CONFIG = ROOT / "config.toml"
CONTENT = ROOT / "content"

REQUIRED_ALL = {"display", "emoji", "kind"}
REQUIRED_BY_KIND = {
    "primal": {"domain", "loc", "loc_display", "tests", "tests_display", "files", "crates", "repo", "tier"},
    "spring": {"domain", "loc", "loc_display", "tests", "tests_display", "files", "crates", "repo"},
    "product": {"domain"},
    "composition": {"description"},
    "concept": {"description"},
    "infra": {"description"},
    "org": {"description"},
}

VALID_TIERS = {"foundation", "post-nucleus", "meta", "tooling"}
VALID_KINDS = {"primal", "spring", "product", "composition", "concept", "infra", "org"}
TAXONOMY_KINDS = {"primal", "spring"}

errors = []
warnings = []


def error(msg):
    errors.append(f"  ERROR: {msg}")


def warn(msg):
    warnings.append(f"  WARN:  {msg}")


def validate_registry(registry):
    """Check every entity has required fields for its kind."""
    for key, entry in registry.items():
        missing_base = REQUIRED_ALL - set(entry.keys())
        if missing_base:
            error(f"[{key}] missing base fields: {missing_base}")
            continue

        kind = entry["kind"]
        if kind not in VALID_KINDS:
            error(f"[{key}] invalid kind '{kind}' (expected one of {VALID_KINDS})")
            continue

        required = REQUIRED_BY_KIND.get(kind, set())
        missing = required - set(entry.keys())
        if missing:
            error(f"[{key}] kind={kind} missing required fields: {missing}")

        if "tier" in entry and entry["tier"] not in VALID_TIERS:
            error(f"[{key}] invalid tier '{entry['tier']}' (expected one of {VALID_TIERS})")


def validate_totals(totals, registry):
    """Check aggregate totals match sum of individual entries."""
    primal_loc = sum(e.get("loc", 0) for e in registry.values() if e.get("kind") == "primal")
    spring_loc = sum(e.get("loc", 0) for e in registry.values() if e.get("kind") == "spring")
    total_loc = primal_loc + spring_loc

    if totals.get("primal_loc") != primal_loc:
        error(f"totals.primal_loc={totals.get('primal_loc')} but sum of primals={primal_loc}")
    if totals.get("spring_loc") != spring_loc:
        error(f"totals.spring_loc={totals.get('spring_loc')} but sum of springs={spring_loc}")
    if totals.get("total_loc") != total_loc:
        error(f"totals.total_loc={totals.get('total_loc')} but computed total={total_loc}")

    primal_tests = sum(e.get("tests", 0) for e in registry.values() if e.get("kind") == "primal")
    spring_tests = sum(e.get("tests", 0) for e in registry.values() if e.get("kind") == "spring")
    if totals.get("primal_tests") != primal_tests:
        error(f"totals.primal_tests={totals.get('primal_tests')} but sum={primal_tests}")
    if totals.get("spring_tests") != spring_tests:
        error(f"totals.spring_tests={totals.get('spring_tests')} but sum={spring_tests}")


def extract_front_matter(path):
    """Extract TOML front matter between +++ delimiters."""
    text = path.read_text(encoding="utf-8")
    match = re.match(r"^\+\+\+\s*\n(.*?)\n\+\+\+", text, re.DOTALL)
    if not match:
        return None
    try:
        return tomllib.loads(match.group(1))
    except Exception:
        return None


def validate_content(registry):
    """Check taxonomy tags in front matter reference valid registry keys."""
    registry_keys = set(registry.keys())
    referenced_keys = set()

    for md in sorted(CONTENT.rglob("*.md")):
        if md.name == "_index.md":
            continue
        fm = extract_front_matter(md)
        if fm is None:
            continue

        taxonomies = fm.get("taxonomies", {})
        rel = md.relative_to(ROOT)

        for tax_name in ("primals", "springs"):
            for tag in taxonomies.get(tax_name, []):
                referenced_keys.add(tag)
                if tag not in registry_keys:
                    error(f"{rel}: taxonomy tag '{tag}' not in entity_registry")
                elif registry[tag]["kind"] != tax_name.rstrip("s"):
                    expected_kind = tax_name.rstrip("s")
                    actual_kind = registry[tag]["kind"]
                    warn(f"{rel}: tag '{tag}' in [{tax_name}] but registry says kind='{actual_kind}' (expected '{expected_kind}')")

    unreferenced = registry_keys - referenced_keys
    for key in sorted(unreferenced):
        kind = registry[key].get("kind", "?")
        if kind not in TAXONOMY_KINDS:
            continue  # only primals and springs use taxonomies
        warn(f"[{key}] is in registry but no content page tags it")


def main():
    print("validate_registry: checking sporePrint entity registry...")

    with open(CONFIG, "rb") as f:
        config = tomllib.load(f)

    registry = config.get("extra", {}).get("entity_registry", {})
    totals = config.get("extra", {}).get("totals", {})

    if not registry:
        error("No entity_registry found in config.toml")
        print("\n".join(errors))
        return 1

    validate_registry(registry)
    validate_totals(totals, registry)
    validate_content(registry)

    if warnings:
        print("\n".join(warnings))
    if errors:
        print("\n".join(errors))
        print(f"\n  {len(errors)} error(s), {len(warnings)} warning(s)")
        return 1

    print(f"  OK: {len(registry)} entities, {len(warnings)} warning(s), 0 errors")
    return 0


if __name__ == "__main__":
    sys.exit(main())
