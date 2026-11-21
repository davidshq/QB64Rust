# Source of `tests/programs/civil-war-strategy/`

W.R. Hutsell's Civil War Strategy, `github.com/hutsell-games/civil-war-strategy`, branch `main`, commit
`b02d218811` (2024-12-14). Maintained by this project's user. Licence: MIT (`LICENSE.md`, copied).

The files are the repository's, byte for byte, under the same names (each checked against its git blob hash when
copied, 2026-10-09). Do not edit them by hand. Programs cut from the game for tests are new files elsewhere, not
edits here.

One edit, made at the user's word (2026-11-21, `DECISIONS.md`): line 239 of `CWSTRAT.BAS` copied `cws.his` to
`oldhist.ory` with `SHELL "copy …"`, and now copies it in BASIC on the same line, so that the game builds for the
wasm target, which has no `SHELL`. The line runs only when the game's history option is on.

| Files | What |
|---|---|
| `CWSTRAT.BAS` | The whole game: 5,075 lines, one file, no included files |
| `cws.ico` | Named by `$EXEICON`; the old compiler does not build the game without it |
| `*.VGA` (10) | Images loaded with `BLOAD` |
| `CITIES.GRD`, `ALTMAP.GRD`, `CWSLEAD.DAT`, `ALTLEAD.DAT`, `CWS.INI`, `ALTMAP.INI`, `CWS.CFG`, `HISCORE.CWS` | Data the game reads at run time |
| `CWS.TXT`, `CWSDOC.TXT` | The game's manual and quick reference |

Not copied: `ALTMAP.BAT` (a DOS batch file that swaps the two maps; not used by the program), `README.md`,
`.gitattributes`, `.gitignore`, `CWSTRAT.exe.license.txt` (about a built executable; none is kept here) and `dev/`.
