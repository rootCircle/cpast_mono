#!/usr/bin/env python3
"""Minimal cleaner for cpast_api test databases.

Edits are controlled via the constants below if needed.
Pattern: cpast_api_tests_<uuid_simple_hex>
"""

import os
import asyncio
import re
import subprocess
import sys

# --- Configuration constants ---
HOST = "localhost"
PORT = "5432"
USER = "postgres"
DBNAME = "postgres"  # maintenance DB
PASSWORD = "password"  # used via PGPASSWORD env for psql auth
PATTERN = r"^cpast_api_tests_[0-9a-f-]{32}$"

# Toggle behavior
DRY_RUN = False
ASSUME_YES = False
MAX_PARALLEL_DROPS = 64  # adjust if needed


def run_psql(sql: str, check: bool = True, quiet: bool = False) -> subprocess.CompletedProcess:
    env = {**os.environ, "PGPASSWORD": PASSWORD}
    base = [
        "psql",
        "--no-align",
        "--tuples-only",
        "--quiet",
        "-h",
        HOST,
        "-p",
        str(PORT),
        "-U",
        USER,
        "-d",
        DBNAME,
        "-v",
        "ON_ERROR_STOP=1",
        "-c",
        sql,
    ]
    if quiet:
        return subprocess.run(base, check=check, env=env, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    else:
        return subprocess.run(base, check=check, env=env, capture_output=True, text=True)


async def run_psql_async(sql: str, check: bool = True, quiet: bool = False):
    env = {**os.environ, "PGPASSWORD": PASSWORD}
    base = [
        "psql",
        "--no-align",
        "--tuples-only",
        "--quiet",
        "-h",
        HOST,
        "-p",
        str(PORT),
        "-U",
        USER,
        "-d",
        DBNAME,
        "-v",
        "ON_ERROR_STOP=1",
        "-c",
        sql,
    ]
    stdout = asyncio.subprocess.DEVNULL if quiet else asyncio.subprocess.PIPE
    stderr = asyncio.subprocess.DEVNULL if quiet else asyncio.subprocess.PIPE
    proc = await asyncio.create_subprocess_exec(*base, env=env, stdout=stdout, stderr=stderr)
    out_b, err_b = await proc.communicate()
    if check and proc.returncode != 0:
        raise subprocess.CalledProcessError(
            proc.returncode,
            base,
            output=(out_b.decode() if out_b else None),
            stderr=(err_b.decode() if err_b else None),
        )
    return proc


def list_databases():
    pattern_sql = PATTERN.replace("'", "''")
    try:
        proc = run_psql(
            f"SELECT datname FROM pg_database WHERE datname ~ '{pattern_sql}' ORDER BY datname;",
            check=True,
        )
    except FileNotFoundError:
        print("Error: psql not found in PATH.", file=sys.stderr)
        sys.exit(2)
    if proc.returncode != 0:
        print(getattr(proc, "stderr", None) or "Failed to query databases", file=sys.stderr)
        sys.exit(proc.returncode)
    lines = [ln.strip() for ln in (proc.stdout or "").splitlines() if ln.strip()]
    # Extra safeguard: strict full match filter
    rx = re.compile(PATTERN)
    return [d for d in lines if rx.fullmatch(d)]


def confirm(prompt: str) -> bool:
    if ASSUME_YES:
        return True
    try:
        reply = input(f"{prompt} [y/N]: ").strip().lower()
    except EOFError:
        return False
    return reply in ("y", "yes")


def terminate_and_drop(db: str) -> None:
    term_sql = (
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity "
        f"WHERE datname = '{db}' AND pid <> pg_backend_pid();"
    )
    drop_sql = f'DROP DATABASE IF EXISTS "{db}";'
    # Ignore failures on terminate step
    try:
        run_psql(term_sql, check=False, quiet=True)
    except Exception:
        pass
    run_psql(drop_sql, check=True)


async def amain() -> None:
    print(f"Connecting to Postgres: host={HOST} port={PORT} user={USER} db={DBNAME}")
    print(f"Searching for databases matching regex: {PATTERN}")

    dbs = list_databases()
    if not dbs:
        print("No databases found that match the pattern. Nothing to do.")
        return

    print("Found databases:")
    for d in dbs:
        print(f"  - {d}")

    if DRY_RUN:
        print("Dry-run: not dropping any databases.")
        return

    if not confirm("Proceed to drop ALL of the above databases?"):
        print("Aborted.")
        return

    total = len(dbs)
    workers = min(MAX_PARALLEL_DROPS, total) or 1
    print(f"Dropping {total} databases with concurrency={workers}...")

    sem = asyncio.Semaphore(workers)
    successes = 0
    failures = 0

    async def drop_one(d: str):
        nonlocal successes, failures
        async with sem:
            term_sql = (
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity "
                f"WHERE datname = '{d}' AND pid <> pg_backend_pid();"
            )
            drop_sql = f'DROP DATABASE IF EXISTS "{d}";'
            try:
                # Terminate quietly; ignore errors
                try:
                    await run_psql_async(term_sql, check=False, quiet=True)
                except Exception:
                    pass
                await run_psql_async(drop_sql, check=True, quiet=False)
                print(f"Dropped {d}")
                successes += 1
            except subprocess.CalledProcessError as e:
                print(f"Failed to drop {d}: {e}", file=sys.stderr)
                failures += 1
            except Exception as e:
                print(f"Error while dropping {d}: {e}", file=sys.stderr)
                failures += 1

    await asyncio.gather(*(drop_one(d) for d in dbs))

    print(f"Done. Success: {successes}, Failed: {failures}")


if __name__ == "__main__":
    asyncio.run(amain())
