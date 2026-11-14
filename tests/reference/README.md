# The language reference's examples

Each page of the language reference (`docs\reference\entries\`) shows its examples, and each example is a
program here, recorded with `qb64pe.exe` and compared with `qb64rust` as a corpus program is
(`tests\corpus\README.md` gives the conventions: `.output`, `.err`, `.norun`, `.stdin`, `.files`, `.external`).
The tool that embeds them in the pages is `tools\reference\reference.py` (`tools\reference\README.md`).

| Path | Content |
|---|---|
| `examples\<page>_<n>.bas` | Example `n` of the page `<page>.md`, with `$CONSOLE:ONLY` on its first line, and its recording `<page>_<n>.output` |
| `examples\<page>_<n>.files\` | The files an example reads or includes, copied beside it before it is built |
| `pass.list` | The examples the new compiler passes. Only grows |
| `availability.json` | Written by `reference.py render --compiler`: for each page, how many of the programs that use its words compile |

```
python tools\legacy_tests\run_legacy_tests.py --suite corpus --corpus-root tests\reference --record ^
    --glob "left-str_*.bas" --results target\legacy-tests-reference
python tools\legacy_tests\run_legacy_tests.py --suite corpus --corpus-root tests\reference ^
    --list tests\reference\pass.list --qb64 target\release\qb64rust.exe --results target\legacy-tests-reference
```

A run of more than about 20 examples goes to the suites VM: `just check reference-examples --where vm` (`CLAUDE.md`
working rule 6). The editor reports an example's `$INCLUDE` as not found, because the included file is in the
example's `.files` folder and is copied beside it only for the run.
