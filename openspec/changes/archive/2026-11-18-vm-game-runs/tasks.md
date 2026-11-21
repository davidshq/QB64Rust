# Tasks

## 1. The pin and the run

- [x] 1.1 `tools\winvm\pins.psd1`: Node.js 24.22.3, `node-v24.22.3-win-x64.zip`, its SHA-256 from nodejs.org's
      `SHASUMS256.txt` (checked 2026-11-17)
- [x] 1.2 `tools\winvm\game.ps1`: start, Node (A), copies of the game and `-With` folders, the job, `-Stop`
- [x] 1.3 `step.ps1`: the optional fields `compiler`, `path` and `scratch`; `outputs.py` from beside itself
- [x] 1.4 `outputs.py`: a folder output; a test in `test_outputs.py`

## 2. Installed Node (B)

- [x] 2.1 `create.ps1 -Shape Suites`: unpack the pinned zip into `%ProgramFiles%\nodejs`, add it to the machine's
      PATH, check `node --version` before the checkpoint
- [x] 2.2 At the next making of the suites VM: see that `create.ps1` reports Node.js v24.22.3, and that a game's
      run says nothing about putting Node on the cache disk (2026-11-17, in a second VM, `qb64rust-suites-2`,
      made in 12 minutes while `qb64rust-suites` was in use: `create.ps1` reported Node.js v24.22.3, and
      `game.ps1 -Game ..\world-war-2 -Command node,--version` printed `v24.22.3` with no line about the cache disk)

## 3. Checks

- [x] 3.1 A game's run in the VM: Civil War Strategy's `play_qbjs.js` on two scripts with `--native`
      (2026-11-17: both passed; Node put on the cache disk in 14 s, the game copied in 80 s. The native runs never
      reached the title menu: the game's `play.py` did not put Mesa's DLLs beside the program, as
      `run_legacy_tests.py` does under `QB64RUST_OPENGL_DIR`. Fixed in the game's `play.py`; `navy` then gave its
      native screens in the VM)
- [x] 3.2 `check`'s VM steps still run (2026-11-17: `just check callsite --where vm` in `qb64rust-suites-2`
      passed in 936 s)
- [x] 3.3 All 41 of Civil War Strategy's scripts through `game.ps1` (2026-11-17: the run went through, with
      native screens; 23 passed and 18 failed, for two QBJS bugs, in the game's `docs\qbjs-steps.md`; 1,190
      output files came back). Again after the two fixes: 41 of 41; and autoplay seeds 1 to 100 through
      `game.ps1`, all matching the native results

## 4. Records

- [x] 4.1 `tools\winvm\README.md`: "A game port's run", and Node in "The suites VM"
- [x] 4.2 `docs\LAYOUT.md` rows for `game.ps1` and `pins.psd1`; a `DECISIONS.md` row
- [x] 4.3 The merged spec `testing/suites-vm` at archive time
