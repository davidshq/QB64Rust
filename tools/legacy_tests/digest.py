"""The framing of the cache keys of the test runner (run_legacy_tests.py) and the call-site check
(tools/callsite/callsite.py).

Each part goes into the hash after its length, as 8 bytes little-endian, so two different lists
of parts never feed the hash the same bytes.
"""

from __future__ import annotations


def framed(h, *parts: bytes) -> None:
    """Feeds each of parts to the hash object h, each after its length."""
    for part in parts:
        h.update(len(part).to_bytes(8, "little"))
        h.update(part)
