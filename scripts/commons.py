#!/usr/bin/env python3
"""commons.py — the data commons of the sporePrint: the files the pages name, held here with their hashes.

The pages name data files, scripts and feeds: `scuffle.json`, `model-arena.py`, a benchmark, a shader.
For a long time the names were all a reader got. This tool finds each named file, checks it, copies it
under static/commons/ and writes down what it is and where it came from, so that anyone holding a copy
of this repository holds the files too, and can prove it.

  python scripts/commons.py gather --tree <dir> [--tree <dir> ...] [--brought <index.json>] [--picks <picks.json>]
      Read content/**/*.md, find each named file in the trees, check it, copy it to static/commons/<repo>/<path>
      and write static/commons/manifest.json. Names that fit several different files are written to
      static/commons/undecided.json and are not copied until someone picks (--picks).

  python scripts/commons.py check [<dir or base address>] [--json]
      Hash what a holder has against the manifest. A holder is a directory holding static/commons
      (default: this repository) or the address of a site that serves /commons/.

What is not copied, and is listed in the manifest with the reason: a file that names a house network
address, carries a login, a key or a quoted secret, is larger than 5 MB, or cannot be told apart from
another file of the same name. A name no tree holds is listed as not found. Nothing is guessed.
"""
import argparse
import hashlib
import json
import os
import re
import subprocess
import sys
import time
import urllib.error
import urllib.request
from collections import Counter, defaultdict
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
COMMONS = REPO / "static" / "commons"
MANIFEST = COMMONS / "manifest.json"
UNDECIDED = COMMONS / "undecided.json"

EXT = ("json|jsonl|py|sh|pi|rs|wgsl|mid|midi|pdb|csv|tsv|toml|ipynb|wasm|png|svg|mp3|wav|ogg|mp4|webm|pdf|html|js|"
       "eml|txt|md|cff|bib")
DATA_EXT = {"json", "jsonl", "csv", "tsv", "mid", "midi", "pdb", "wasm", "mp3", "wav", "ogg", "mp4", "webm", "pdf",
            "ipynb", "png", "svg", "bib"}
CODE_EXT = {"py", "sh", "pi", "rs", "wgsl", "js", "toml"}
PRUNE = {".git", "target", "node_modules", "__pycache__", ".venv", "venv", ".cache", "east-blobs"}
MAX_BYTES = 5 * 1024 * 1024

# A name like these says what kind of file it is, not which one. Alone, it does not point at anything.
LOOSE = re.compile(r"^(config|main|lib|mod|index|readme|cargo|package|manifest|settings|data|test|tests|utils|setup|"
                   r"build|makefile|changelog|license|status|results?|output|input|run|start|init|__init__|app|server|"
                   r"client|types|error|errors|common|schema|state|metrics|report|summary|notes|todo|peers|provenance|"
                   r"signatures|checksums)\.[a-z0-9]+$", re.I)

HOLD = [
    ("names a house network address", re.compile(rb"\b(?:10|192\.168|172\.(?:1[6-9]|2\d|3[01]))(?:\.\d{1,3}){2}\.\d{1,3}\b")),
    ("carries a login inside an address", re.compile(rb"://[^/\s\"'<>@]+:[^@/\s\"'<>]+@")),
    ("carries a private key", re.compile(rb"BEGIN [A-Z ]*PRIVATE KEY")),
    ("carries what looks like an access token", re.compile(
        rb"ghp_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{30,}|sk-[A-Za-z0-9_-]{32,}|AKIA[0-9A-Z]{16}|xox[baprs]-[A-Za-z0-9-]{20,}")),
    ("carries a quoted password or secret", re.compile(
        rb"(?i)\b(?:password|passwd|passphrase|secret|api_?key|auth_?token)\w*\s*[=:]\s*[\"'][^\"'\s$<>{}]{6,}[\"']")),
    ("logs in to a machine with a password on the command line", re.compile(rb"sshpass\s+-p")),
]


def git(cwd, *args):
    r = subprocess.run(["git", "-c", "core.quotepath=false"] + list(args), cwd=str(cwd), capture_output=True)
    return r.stdout.decode("utf-8", "replace").strip() if r.returncode == 0 else ""


def git_bytes(cwd, *args):
    r = subprocess.run(["git", "-c", "core.quotepath=false"] + list(args), cwd=str(cwd), capture_output=True)
    return r.stdout if r.returncode == 0 else None


def sha256(raw):
    return hashlib.sha256(raw).hexdigest()


def posix(p):
    return str(p).replace("\\", "/")


# ── what the pages name ──────────────────────────────────────────────────────

def named_by_pages():
    """name -> {"pages": [page...], "said": [the words in backticks, as written]}"""
    tick = re.compile(r"`([^`\s]*?([A-Za-z0-9_.\-]+\.(?:%s)))`" % EXT)
    named = defaultdict(lambda: {"pages": set(), "said": set()})
    pages = sorted((REPO / "content").rglob("*.md"))
    for page in pages:
        text = page.read_text(encoding="utf-8", errors="replace")
        body = text.split("---", 2)[2] if text.startswith("---") and text.count("---") >= 2 else text
        rel = posix(page.relative_to(REPO))
        for m in tick.finditer(body):
            said, name = m.group(1), m.group(2)
            if name.rsplit(".", 1)[-1] in ("md", "html", "txt") and "/" not in said:
                continue                       # prose files named in passing are not what this is about
            named[name]["pages"].add(rel)
            named[name]["said"].add(said)
    return named, len(pages)


def kind_of(name):
    ext = name.rsplit(".", 1)[-1].lower()
    return "data" if ext in DATA_EXT else "code" if ext in CODE_EXT else "other"


def page_address(page):
    """content/science/111_the_omnibus.md -> /science/111-the-omnibus/"""
    p = page[len("content/"):-len(".md")]
    if p.endswith("/_index") or p == "_index":
        p = p[:-len("_index")].rstrip("/")
    return "/" + re.sub(r"[_\s]+", "-", p).lower().strip("/") + "/"


# ── where files are ──────────────────────────────────────────────────────────

def index_trees(trees, wanted):
    """name -> [absolute paths], for wanted names only. This repository's own files come first."""
    found = defaultdict(list)
    for tree in [REPO] + [Path(t).resolve() for t in trees]:
        for d, dirs, files in os.walk(tree):
            here = Path(d)
            dirs[:] = [x for x in dirs if x not in PRUNE and not (here / x == REPO / "public")
                       and not (here / x == COMMONS) and not (tree != REPO and (here / x) == REPO)]
            for f in files:
                if f in wanted:
                    found[f].append(here / f)
    return found


_repo_cache = {}


def repo_of(path):
    """The repository a file sits in: (top, name, address of its origin without any login)."""
    d = path.parent
    walked = []
    while True:
        if d in _repo_cache:
            hit = _repo_cache[d]
            break
        walked.append(d)
        if (d / ".git").exists():
            origin = re.sub(r"://[^@/]+@", "://", git(d, "remote", "get-url", "origin"))
            origin = re.sub(r"^git@([^:]+):", r"https://\1/", origin)
            name = re.sub(r"\.git$", "", origin.rstrip("/").rsplit("/", 1)[-1]) if origin else d.name
            hit = (d, same_repo(name), public_address(origin))
            break
        if d.parent == d:
            hit = (None, "", "")
            break
        d = d.parent
    for w in walked:
        _repo_cache[w] = hit
    return hit


def same_repo(name):
    """The house forge keeps some repositories under an `infra-` name; it is the same repository."""
    return re.sub(r"^infra-", "", name)


def public_address(origin):
    """A repository's address, unless it is an address inside the house."""
    return "" if HOLD[0][1].search(origin.encode()) or "localhost" in origin else origin


def where_from(path):
    top, name, origin = repo_of(path)
    if top is None:
        return {"repo": "", "address": "", "path": path.name, "commit": "", "date": "",
                "committed": "not in a repository"}
    rel = posix(path.relative_to(top))
    tracked = bool(git(top, "ls-files", "--error-unmatch", "--", rel))
    changed = bool(git(top, "status", "--porcelain", "--", rel))
    last = (git(top, "log", "-1", "--format=%H %cI", "--", rel) if tracked else "").split()
    return {"repo": name, "address": origin, "path": rel,
            "commit": last[0] if last else "", "date": last[1] if len(last) > 1 else "",
            "committed": ("yes" if not changed else "yes, with later changes not committed") if tracked else "no",
            "_top": top}


def committed_bytes(path):
    """A file as its repository committed it, byte for byte; as it lies on disk if it was never committed.
    A checkout may rewrite line ends, and a working copy may carry someone's changes in progress:
    neither is what the repository holds."""
    top, _, _ = repo_of(path)
    if top is not None:
        raw = git_bytes(top, "show", "HEAD:" + posix(path.relative_to(top)))
        if raw is not None:
            return raw
    try:
        return path.read_bytes()
    except OSError:
        return None


def as_committed(src, raw):
    """The bytes are already the committed ones (committed_bytes); say so where the working copy differed."""
    if src["committed"].startswith("yes, with"):
        src["committed"] = "yes"
        src["note"] = "the committed version; the working copy had changes on top, which were left out"
    return raw


def published(src, raw):
    """Has the repository this came from published these very bytes? yes / no / not known."""
    top = src.get("_top")
    if top is None or not src["address"] or not src["commit"]:
        return "not known"
    for branch in ("origin/HEAD", "origin/main", "origin/master"):
        there = git(top, "rev-parse", "--verify", "-q", "%s:%s" % (branch, src["path"]))
        if there:
            mine = hashlib.sha1(b"blob %d\0" % len(raw) + raw).hexdigest()
            return "yes" if there == mine else "no"
    return "not known"


def brought_from(item):
    name = re.sub(r"\.git$", "", item["origin"].rstrip("/").rsplit("/", 1)[-1]) or Path(item["top"]).name or ""
    return {"repo": same_repo(name), "address": public_address(item["origin"]), "path": item["rel"] or item["name"],
            "commit": item["commit"], "date": item.get("date", ""),
            "committed": {"yes": "yes", "yes,changed": "yes, with later changes not committed"}.get(item["tracked"], "no")}


def source_of(c):
    if "src" not in c:
        c["src"] = where_from(c["path"]) if "path" in c else brought_from(c["item"])
    return c["src"]


def one_file_in_versions(cands):
    """Are these all the same file of the same repository, at different times? Copies with the same
    bytes count as the same place."""
    groups = []                                  # each: (set of hashes, set of (repo, path))
    for c in cands:
        src = source_of(c)
        key = (src["repo"], src["path"]) if src["commit"] else None
        mine = [g for g in groups if c["sha"] in g[0] or (key and key in g[1])]
        merged = ({c["sha"]}, {key} if key else set())
        for g in mine:
            merged = (merged[0] | g[0], merged[1] | g[1])
            groups.remove(g)
        groups.append(merged)
    return len(groups) == 1 and len(groups[0][1]) == 1


def reason_to_hold(raw):
    if len(raw) > MAX_BYTES:
        return "larger than 5 MB (%d bytes)" % len(raw)
    for why, rx in HOLD:
        if rx.search(raw):
            return why
    return ""


def safe_part(s):
    return re.sub(r"[^A-Za-z0-9._+-]", "_", s) or "_"


# ── gather ───────────────────────────────────────────────────────────────────

def gather(args):
    named, n_pages = named_by_pages()
    found = index_trees(args.tree, set(named))
    brought = defaultdict(list)                # files carried over from a machine that is not here
    if args.brought:
        for item in json.loads(Path(args.brought).read_text(encoding="utf-8")):
            brought[item["name"]].append(item)
    picks = json.loads(Path(args.picks).read_text(encoding="utf-8")) if args.picks else {}

    old = {}
    if MANIFEST.exists():
        old = {b["name"]: b for b in json.loads(MANIFEST.read_text(encoding="utf-8"))["blobs"]}

    blobs, undecided = [], []
    for name in sorted(named, key=str.lower):
        entry = {"name": name, "kind": kind_of(name), "pages": sorted(named[name]["pages"])}
        said_paths = sorted({s.lstrip("./") for s in named[name]["said"] if "/" in s}, key=len, reverse=True)

        # every place this name was found, with its bytes
        cands = []
        for p in found.get(name, []):
            raw = committed_bytes(p)
            if raw is None:
                continue
            cands.append({"where": posix(p), "raw": raw, "sha": sha256(raw), "here": REPO in p.parents, "path": p})
        for item in brought.get(name, []):
            raw = Path(item["local"]).read_bytes()
            cands.append({"where": "%s:%s" % (item.get("machine", "eastGate"), item["path"]), "raw": raw,
                          "sha": sha256(raw), "here": False, "item": item})
        if not cands:
            blobs.append(dict(entry, state="not found", why="no tree given to the gathering holds a file of this name"))
            continue

        # 1. the page gave a path: the file whose own path ends with it
        how = ""
        chosen = [c for c in cands if any(posix(c["where"]).endswith("/" + s) for s in said_paths)]
        if chosen:
            how = "the page gives its path"
        elif name in picks and [c for c in cands if c["where"].endswith("/" + picks[name]["where"])]:
            picked = {c["sha"] for c in cands if c["where"].endswith("/" + picks[name]["where"])}
            chosen = [c for c in cands if c["sha"] in picked]      # every copy of the picked bytes, so one already here wins
            how = picks[name]["how"]
        else:
            chosen = cands
        distinct = {c["sha"] for c in chosen}
        if len(distinct) > 1 and (how or not LOOSE.match(name)) and one_file_in_versions(chosen):
            dated = [c for c in chosen if source_of(c)["date"]]
            if dated:
                newest = max(dated, key=lambda c: source_of(c)["date"])
                chosen = [c for c in chosen if c["sha"] == newest["sha"]]
                how = (how + "; " if how else "") + "%d versions of this one file, the newest commit taken" % len(distinct)
                distinct = {newest["sha"]}
        if len(distinct) > 1 or (not how and LOOSE.match(name)):
            why = ("%d different files carry this name" % len(distinct) if len(distinct) > 1 else
                   "the name says what kind of file it is, not which one")
            undecided.append({"name": name, "why": why, "pages": entry["pages"], "said": sorted(named[name]["said"]),
                              "candidates": [{"where": c["where"], "sha256": c["sha"], "bytes": len(c["raw"])}
                                             for c in sorted(cands, key=lambda c: c["where"])[:12]]})
            blobs.append(dict(entry, state="undecided", why=why))
            continue
        if not how:
            how = ("the only file of this name" if len(cands) == 1 else
                   "%d copies of this name, all the same bytes" % len(cands))
        c = sorted(chosen, key=lambda c: (not c["here"], c["here"] or not source_of(c)["address"],
                                          c["here"] or not source_of(c)["commit"], len(c["where"]), c["where"]))[0]

        if c["here"]:                          # already part of this repository, where it is
            rel = posix(c["path"].relative_to(REPO))
            blobs.append(dict(entry, state="held", how=how, path=rel, sha256=c["sha"], bytes=len(c["raw"]),
                              address=("/" + rel[len("static/"):]) if rel.startswith("static/") else "",
                              **{"from": {"repo": "sporePrint", "path": rel, "note": "was already here"}}))
            continue

        src = dict(source_of(c))
        raw = as_committed(src, c["raw"])
        src["published_there"] = published(src, raw)
        src.pop("_top", None)
        c = dict(c, raw=raw, sha=sha256(raw))
        why = reason_to_hold(raw)
        if not why and src["repo"] in args.only_published and src["published_there"] != "yes":
            why = "its repository has not published this version, and files of %s go in only once it has" % src["repo"]
        if why:
            blobs.append(dict(entry, state="held back", why=why, sha256=c["sha"], bytes=len(raw), **{"from": src}))
            continue
        rel = "static/commons/%s/%s" % (safe_part(src["repo"] or "loose"), "/".join(safe_part(x) for x in src["path"].split("/")))
        dest = REPO / rel
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_bytes(c["raw"])
        # a directory named target/ or public/ on the way, or a path too long for a checkout on Windows
        if git(REPO, "check-ignore", "--", rel) or len(rel) > 180:
            dest.unlink()
            rel = "static/commons/%s/_/%s" % (safe_part(src["repo"] or "loose"), name)
            dest = REPO / rel
            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_bytes(c["raw"])
        prev = old.get(name, {})
        blobs.append(dict(entry, state="held", how=how, path=rel, address="/" + rel[len("static/"):],
                          sha256=c["sha"], bytes=len(c["raw"]),
                          gathered=prev.get("gathered") if prev.get("sha256") == c["sha"] else time.strftime("%Y-%m-%d"),
                          **{"from": src}))

    # a file under static/commons that no entry points at is left over from an earlier gathering
    kept = {b["path"] for b in blobs if b.get("path", "").startswith("static/commons/")}
    for p in COMMONS.rglob("*"):
        if p.is_file() and p.parent != COMMONS and posix(p.relative_to(REPO)) not in kept:
            p.unlink()
    for d in sorted((p for p in COMMONS.rglob("*") if p.is_dir()), key=lambda p: -len(p.parts)):
        if not any(d.iterdir()):
            d.rmdir()

    states = Counter(b["state"] for b in blobs)
    manifest = {
        "what": "The files the sporePrint pages name. `held`: the file is in this repository at `path`, with its "
                "SHA-256. `held back`: found, not copied, for the reason given. `undecided`: several files fit the "
                "name. `not found`: no tree the gathering was given holds it. Made by scripts/commons.py.",
        "made": time.strftime("%Y-%m-%dT%H:%M:%S%z"),
        "pages_read": n_pages,
        "counts": {"named": len(blobs), **{s: states.get(s, 0) for s in ("held", "held back", "undecided", "not found")},
                   "bytes_held": sum(b.get("bytes", 0) for b in blobs if b["state"] == "held")},
        "blobs": blobs,
    }
    COMMONS.mkdir(parents=True, exist_ok=True)
    home = posix(Path.home())
    roots = [posix(Path(t).resolve()) + "/" for t in args.tree]
    for u in undecided:                          # where the trees sit on the gathering machine says nothing to anyone else
        for cand in u["candidates"]:
            w = cand["where"]
            if w.startswith(posix(REPO) + "/"):
                w = "sporePrint/" + w[len(posix(REPO)) + 1:]
            for root in roots:
                if w.startswith(root):
                    w = w[len(root):]
            cand["where"] = w.replace(home, "~")
    for text in (json.dumps(manifest, default=str), json.dumps(undecided, default=str)):
        if HOLD[0][1].search(text.encode()):
            sys.exit("STOP: a house network address would be written into the commons; nothing written")
    MANIFEST.write_text(json.dumps(manifest, ensure_ascii=False, indent=1) + "\n", encoding="utf-8", newline="\n")
    UNDECIDED.write_text(json.dumps(undecided, ensure_ascii=False, indent=1) + "\n", encoding="utf-8", newline="\n")

    c = manifest["counts"]
    print("pages read: %d; files named: %d" % (n_pages, c["named"]))
    print("  held: %d (%.1f MB) | held back: %d | undecided: %d | not found: %d" % (
        c["held"], c["bytes_held"] / 1e6, c["held back"], c["undecided"], c["not found"]))
    print("  held, by where from: " + ", ".join("%s %d" % kv for kv in Counter(
        b["from"]["repo"] or "no repository" for b in blobs if b["state"] == "held").most_common(14)))
    print("  held, by how it was told apart: " + ", ".join("%s %d" % kv for kv in Counter(
        re.sub(r"^\d+ copies", "several copies", b["how"]) for b in blobs if b["state"] == "held").most_common()))
    print("  held back, by reason: " + ", ".join("%s %d" % kv for kv in Counter(
        re.sub(r" \(\d+ bytes\)", "", b["why"]) for b in blobs if b["state"] == "held back").most_common()))
    print("  not committed where they came from: %d" % sum(
        1 for b in blobs if b["state"] == "held" and b["from"].get("committed", "yes") != "yes"))
    print("  held, has the source published these bytes: " + ", ".join("%s %d" % kv for kv in Counter(
        b["from"].get("published_there", "was already here") for b in blobs if b["state"] == "held").most_common()))
    print("  the committed version taken instead of a changed working copy: %d" % sum(
        1 for b in blobs if b["state"] == "held" and "working copy" in b["from"].get("note", "")))


# ── check ────────────────────────────────────────────────────────────────────

def check(args):
    manifest = json.loads(MANIFEST.read_text(encoding="utf-8"))
    held = [b for b in manifest["blobs"] if b["state"] == "held"]
    holder = args.holder or str(REPO)
    remote = holder.startswith("http://") or holder.startswith("https://")

    def one(b):
        try:
            if remote:
                # a site serves static/ at its root; a raw view of the repository serves the path as it is
                tail = b["path"][len("static/"):] if (b["path"].startswith("static/") and not args.raw) else b["path"]
                req = urllib.request.Request(holder.rstrip("/") + "/" + tail, headers={"User-Agent": "sporePrint-commons"})
                with urllib.request.urlopen(req, timeout=30) as r:
                    raw = r.read()
            else:
                raw = (Path(holder) / b["path"]).read_bytes()
        except (OSError, urllib.error.URLError, ValueError):
            return b["name"], "missing"
        if sha256(raw) == b["sha256"]:
            return b["name"], "same"
        # a checkout on Windows may have rewritten the line ends of a text file outside static/commons
        if not remote and sha256(raw.replace(b"\r\n", b"\n")) == b["sha256"]:
            return b["name"], "same"
        return b["name"], "differs"

    servable = [b for b in held if not remote or args.raw or b["path"].startswith("static/")]
    with ThreadPoolExecutor(8 if remote else 1) as pool:
        results = list(pool.map(one, servable))
    tally = Counter(state for _, state in results)
    out = {"holder": holder, "checked": time.strftime("%Y-%m-%dT%H:%M:%S%z"), "manifest_made": manifest["made"],
           "of": len(servable), "same": tally["same"], "differs": tally["differs"], "missing": tally["missing"],
           "not_same": sorted(n for n, s in results if s != "same")[:40]}
    if args.json:
        print(json.dumps(out, ensure_ascii=False))
    else:
        print("%s: %d of %d the same as the manifest; %d differ; %d missing" % (
            holder, out["same"], out["of"], out["differs"], out["missing"]))
        for n in out["not_same"][:12]:
            print("  not the same: %s" % n)
    return 0 if out["same"] == out["of"] else 1


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = ap.add_subparsers(dest="cmd", required=True)
    g = sub.add_parser("gather")
    g.add_argument("--tree", action="append", default=[])
    g.add_argument("--brought")
    g.add_argument("--picks")
    g.add_argument("--only-published", action="append", default=[], metavar="REPO",
                   help="files of this repository go in only if the repository has already published the same bytes")
    c = sub.add_parser("check")
    c.add_argument("holder", nargs="?")
    c.add_argument("--raw", action="store_true", help="the address serves the repository as it is (paths start with static/)")
    c.add_argument("--json", action="store_true")
    args = ap.parse_args()
    sys.exit(gather(args) if args.cmd == "gather" else check(args))


if __name__ == "__main__":
    main()
