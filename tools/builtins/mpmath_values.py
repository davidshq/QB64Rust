"""Writes crates/sema/src/mpmath_values.rs: true values of the functions the CONST evaluator certifies, computed
with mpmath at 60 significant digits, for the test of the high-precision reference (design D3 of
m3-constant-evaluation, guard 2). mpmath is needed only to write the file again; the file is not edited by hand.

    python tools/builtins/mpmath_values.py
"""

import os

import mpmath

mpmath.mp.dps = 80

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.normpath(os.path.join(HERE, "..", "..", "crates", "sema", "src", "mpmath_values.rs"))

# Arguments that are exact binary fractions, so the reference and mpmath take the same value.
ARGS = ["0.25", "0.5", "1", "1.5", "2", "3.75", "-0.75", "10.125", "100.5", "0.0078125"]
UNIT = ["0.25", "0.5", "-0.75", "0.0078125", "-0.5"]

FUNCTIONS = [
    ("sin", mpmath.sin, ARGS),
    ("cos", mpmath.cos, ARGS),
    ("tan", mpmath.tan, ARGS),
    ("atan", mpmath.atan, ARGS),
    ("exp", mpmath.exp, ARGS),
    ("ln", mpmath.log, [a for a in ARGS if not a.startswith("-")]),
    ("asin", mpmath.asin, UNIT),
    ("acos", mpmath.acos, UNIT),
    ("sqrt", mpmath.sqrt, [a for a in ARGS if not a.startswith("-")]),
]
POWERS = [("2", "0.5"), ("10", "0.3125"), ("20", "0.25"), ("1.5", "-2.5"), ("3.75", "1.125")]


def text(v):
    return mpmath.nstr(v, 60, strip_zeros=False, min_fixed=-10**9, max_fixed=10**9).replace(" ", "")


def main():
    rows = []
    for name, f, args in FUNCTIONS:
        for a in args:
            rows.append((name, a, "", text(f(mpmath.mpf(a)))))
    for x, y in POWERS:
        yv = mpmath.mpf(y)
        rows.append(("pow", x, y, text(mpmath.power(mpmath.mpf(x), yv))))
    out = [
        "//! True values of the functions `certify.rs` takes a reference of, from Python's `mpmath` at 60 digits,",
        "//! written by `tools/builtins/mpmath_values.py` (design D3 of `m3-constant-evaluation`). Not edited by hand.",
        "",
        "/// (function, argument, second argument of `pow`, value).",
        "pub(crate) const MPMATH: &[(&str, &str, &str, &str)] = &[",
    ]
    for name, a, b, v in rows:
        out.append('    ("%s", "%s", "%s", "%s"),' % (name, a, b, v))
    out.append("];")
    out.append("")
    data = "\n".join(out).encode("utf-8")
    try:
        with open(OUT, "rb") as f:
            if f.read() == data:
                print("mpmath_values.rs: unchanged")
                return
    except FileNotFoundError:
        pass
    with open(OUT, "wb") as f:
        f.write(data)
    print("wrote", OUT, len(rows), "values")


if __name__ == "__main__":
    main()
