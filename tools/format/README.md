# The formatter's keyword table

`layout_words.py` writes the keyword spellings the formatter uses (OpenSpec change `m5b-formatter`, design D3)
from the reference clone's sources:

- `crates\format\src\words.rs`: every word of an `SCase$("...")` or `SCase2$("...")` call in `source\` (`*.bas`,
  `*.bi`, `*.bm`) whose argument is made of string literals and the separators `sp` and `sp2`, in upper case with
  its spelling (`END` `End`, `GOSUB` `GoSub`, `$INCLUDE` `$Include`, `_ARRAYCOPY` `_ArrayCopy`), in order of the
  upper-case word.

```
python tools\format\layout_words.py            # write the table
python tools\format\layout_words.py --check    # exit 1 when the file is not what the clone gives
python -m unittest discover -s tools/format -p "test_*.py"
```

The rules: a literal is split into words at blanks and at `sp`/`sp2` (`"End" + sp + "If"` gives `End` and `If`);
a metacommand keeps its colon and quote (`$ExeIcon:'`, `$Resize:Stretch`); a call in a comment, or with any other
argument (`SCase$(RTRIM$(id.cn))`, `SCase2$(t$)`), gives nothing. Those spell names at run time: the built-in
table's name (`crates\builtins`, from `tools\builtins\builtins.json`) and `SCase2$`'s CamelCase rule, which the
formatter applies itself. A word spelled two ways stops the tool. QB64pe is MIT; these are short words.

A new reference commit that changes a spelling makes `--check` (and the tool's test against the clone) fail; run
the tool again.
