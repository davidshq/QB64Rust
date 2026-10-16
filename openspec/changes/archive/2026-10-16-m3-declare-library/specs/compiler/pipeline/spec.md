# Spec Delta

## MODIFIED Requirements

### Requirement: The build follows the program
The emitter and the driver SHALL give a program the start-up settings and the make flags the old compiler gives
it: a console, a console without a window, a window hidden at the start, each optional part of the runtime
one of its built-ins needs, the icon `$EXEICON` names as a resource of the executable, and the library files its
`DECLARE LIBRARY` blocks name on the link command. Two builds that differ in any of them, or in the bytes of the
icon, of a library file or of a header a block names, SHALL NOT share a cached executable.

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

#### Scenario: A library on the link command
- **WHEN** a program's block names `"./v37c"` and `v37c.a` lies beside it
- **THEN** the executable is linked with that file, as the old compiler's is

#### Scenario: A changed header
- **WHEN** the bytes of a header a block names change and the program is built again with the build cache
- **THEN** the executable is built again, not copied from the cache
