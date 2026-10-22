# The old compiler's wording of errors

`old_wording.py` writes `crates\base\src\old_wording.tsv` (and its word list `old_wording_words.txt`): for each
message of this compiler that a recorded rejection pairs exactly with a message of `qb64pe.exe`, the old wording,
which `qb64rust` prints as a note after the error and the language server adds to the message
(`m3-old-wording-notes`, its design for the rules).

```
cargo build --release
python tools\diagnostics\old_wording.py            # the table, from verification\*.compile.txt and the .err files
cargo build --release
python tools\diagnostics\old_wording.py --check    # every note on a recorded rejection is its recorded message
```

Run both when a change alters messages (a changed message loses its note until then; none is ever wrong) or adds
recorded rejections. Never edit the table by hand.

`test_old_wording.py` tests the rules that need no compiler (which old messages name a file: a masked folder, a
backslash or a BASIC file's extension, not a `/` between words), run by CI:

```
python -m unittest discover -s tools\diagnostics -p "test_*.py"
```
