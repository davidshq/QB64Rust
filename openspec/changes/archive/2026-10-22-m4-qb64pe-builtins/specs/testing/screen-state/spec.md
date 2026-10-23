# Spec Delta

## MODIFIED Requirements

### Requirement: Keys for a program with a window
With `QB64_SCREEN_KEYS=<file>` the same file SHALL give the program the keys the file lists, in order, as if
typed: `text <printable ASCII>`, `key <name>` (Enter, Esc, Tab, Backspace, Space, the arrow and editing keys,
F1 to F12), `wait <milliseconds>`, `prompt`, `state`, and as the last line `repeat <name>`, which presses the key
until the program exits. `hold <name>` SHALL put a key down and leave it down until `lift <name>` puts it up; its
names are those of `key` and the modifier keys `lshift`, `rshift`, `lctrl`, `rctrl`, `lalt` and `ralt`. A key held
at the end of the file SHALL stay down until the program exits. No key SHALL be lost, whatever the number of keys and the speed of the program: a key is
given only when the program has read the ones before. A program announces that it waits for a key by adding 1
(modulo 256) to byte 0x4F0 of conventional memory (`DEF SEG = 0: POKE 1264, (PEEK(1264) + 1) AND 255`); the
k-th `prompt` SHALL wait for the k-th announcement before the steps after it are taken. If it does not come
within 30 seconds (or the milliseconds of `QB64_SCREEN_PROMPT_MS`), the program SHALL be ended with exit code 124,
after the state as it is then, and a message that names the line of the `prompt` SHALL be the last line of the
state file and be written to stderr; if a `prompt` finds that the program has already announced more waits than
that, it SHALL end the program the same way with exit code 125 and a message that says so (a check that catches a
key given without a `prompt` when it is seen, not a guarantee). `state` SHALL write the state of the
screen at that moment to the file named by `QB64_SCREEN_STATE`, after the states written before it; the state at
exit SHALL follow the last of them. A keys file without `prompt` and `state` SHALL give its keys as before. A keys
file that cannot be read, or a line that is no step, SHALL end the program with exit code 125 and a message on
stderr.

#### Scenario: More keys than the key buffer holds
- **WHEN** the keys file holds 600 characters and then Enter, and the program reads `INKEY$` until Enter
- **THEN** the program receives all 601 keys in order

#### Scenario: Special keys between characters
- **WHEN** the keys are `a`, Up, `Z`, F1, Esc, `q`, Enter
- **THEN** `INKEY$` returns `a`, `CHR$(0) + "H"`, `Z`, `CHR$(0) + ";"`, `CHR$(27)`, `q`, `CHR$(13)`

#### Scenario: END's prompt
- **WHEN** a program ends with `END` and the keys file is `repeat space`
- **THEN** the program exits

#### Scenario: A mistake in the keys file
- **WHEN** a line reads `key bogus`
- **THEN** the program exits with code 125 and stderr names the file and the line

#### Scenario: A key a pause would eat
- **WHEN** a program announces a wait, reads one key, then runs a loop that ends early when `INKEY$` returns a key,
  then announces a second wait, and the keys file is `prompt`, `text a`, `prompt`, `text b`
- **THEN** the loop runs to its end and the second wait reads `b`

#### Scenario: A state while the program runs
- **WHEN** a program draws a box, announces a wait, clears the screen after the key, and ends with `SYSTEM`, and
  the keys file is `prompt`, `state`, `text x`
- **THEN** the state file holds two states: the first with the box, the second without it

#### Scenario: A menu that announces after every key
- **WHEN** a menu announces a wait before each key it reads, and the keys file is `prompt`, `key down`,
  `prompt`, `key down`, `prompt`, `state`, `key enter`
- **THEN** the state shows the third entry chosen, whatever the speed of the program

#### Scenario: A prompt that never comes
- **WHEN** the keys file starts with `prompt` and the program never writes byte 0x4F0
- **THEN** the program is ended after 30 seconds with exit code 124, the state at that moment is written, and the
  last line of the state file names the line of the `prompt`

#### Scenario: A key held and lifted
- **WHEN** a program announces a wait, then loops until `_KEYDOWN(100304)`, prints it, loops until it is 0 and
  prints it again, and the keys file is `prompt`, `hold lshift`, `wait 200`, `lift lshift`
- **THEN** it prints `-1  0 `, and `_KEYHIT` reads 100304 and then -100304

#### Scenario: A name that is not a key
- **WHEN** a line reads `hold bogus` or `lift`
- **THEN** the program exits with code 125 and stderr names the file and the line

