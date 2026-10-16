# Spec Delta

## MODIFIED Requirements

### Requirement: Program settings in the IR
The IR SHALL say of a program, in terms that name no libqb symbol and no make flag: whether it has a console,
whether it has a window, whether the window is hidden at the start, which optional parts of the runtime its
built-ins need, and whether its executable carries an icon. The IR dump SHALL show them, and SHALL NOT show
the folder the icon's file is in.

#### Scenario: Console and hidden window
- **WHEN** a program with `$CONSOLE` and `$SCREENHIDE` is lowered
- **THEN** the IR says it has a console and a window that starts hidden

#### Scenario: Console only
- **WHEN** a program with `$CONSOLE:ONLY` is lowered
- **THEN** the IR says it has a console and no window

#### Scenario: Sound
- **WHEN** a program with `SOUND 440, 1` is lowered
- **THEN** the IR says it needs the audio part of the runtime, and a program with only `BEEP` does not

#### Scenario: An icon
- **WHEN** a program with `$EXEICON:'./game.ico'` is lowered
- **THEN** the IR says its executable carries an icon and that it needs the icon part of the runtime

### Requirement: The build follows the program
The emitter and the driver SHALL give a program the start-up settings and the make flags the old compiler gives
it: a console, a console without a window, a window hidden at the start, each optional part of the runtime
one of its built-ins needs, and the icon `$EXEICON` names as a resource of the executable. Two builds that
differ in any of them, or in the bytes of the icon, SHALL NOT share a cached executable.

#### Scenario: A program with a console and a window
- **WHEN** a program with `$CONSOLE`, `$SCREENHIDE` and `_DEST _CONSOLE` is built and run
- **THEN** it links against the runtime with the window, shows no window, and its output equals the old
  compiler's

#### Scenario: A built-in that needs a part of the runtime
- **WHEN** a program names `_ICON`
- **THEN** it is built with the make flag the old compiler passes for the icon dependency

#### Scenario: Reference clone unchanged
- **WHEN** a program with `$CONSOLE` is built for the first time
- **THEN** `git status --porcelain` in the reference clone prints the same as before the build

#### Scenario: A program with sound
- **WHEN** a program names `SOUND` or `PLAY`
- **THEN** it is built with the make flag the old compiler passes for the audio dependency, and a program
  without them is built without it

#### Scenario: An icon in the executable
- **WHEN** a program with `$EXEICON:'./game.ico'` is built
- **THEN** the executable holds the images of `game.ico` as its icon resource, as the old compiler's does

#### Scenario: A changed icon
- **WHEN** the icon file's bytes change and the program is built again with the build cache
- **THEN** the executable is built again, not copied from the cache

#### Scenario: A file that is not an icon
- **WHEN** `$EXEICON` names a text file
- **THEN** the build fails, as the old compiler's does
