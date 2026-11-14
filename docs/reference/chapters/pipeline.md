# The program's text

A program is lines of text in one file and the files it includes. Before it is compiled, the preprocessor keeps or
drops lines by `$IF`, and metacommands set how it is built.

The pages of this area: [REM](../entries/rem.md), [$INCLUDE](../entries/meta-include.md),
[$INCLUDEONCE](../entries/meta-includeonce.md), [$IF](../entries/meta-if.md), [$LET](../entries/meta-let.md),
[$ERROR](../entries/meta-error.md), [$ERRORLOCATION](../entries/meta-errorlocation.md),
[$NOPREFIX](../entries/meta-noprefix.md), [$FORMAT](../entries/meta-format.md),
[$CONSOLE](../entries/meta-console.md), [$EXEICON](../entries/meta-exeicon.md),
[$VERSIONINFO](../entries/meta-versioninfo.md), [$STATIC](../entries/meta-static.md),
[$DYNAMIC](../entries/meta-dynamic.md), [$CHECKING](../entries/meta-checking.md),
[$ASSERTS](../entries/meta-asserts.md), [$DEBUG](../entries/meta-debug.md), [$UNSTABLE](../entries/meta-unstable.md),
[$EMBED](../entries/meta-embed.md), [$USELIBRARY](../entries/meta-uselibrary.md),
[$RESIZE](../entries/meta-resize.md), [$SCREENHIDE](../entries/meta-screenhide.md),
[$MIDISOUNDFONT](../entries/meta-midisoundfont.md).

## Lines and names

A name is at most 40 characters, counted without its suffix: a longer word is a compile error wherever it stands.
<!-- src: compiler/pipeline "The 40-character limit on names" -->

A block that is not closed is one error at its first line, and a closer without its block one error at the closer; the
compiler goes on after either.
<!-- src: compiler/pipeline "Block structure" -->

Labels and line numbers belong to the body they stand in ([Control flow](control-flow.md)).
<!-- src: language/control-flow "Labels per body" -->

## Comments that are metacommands

A comment whose text starts with `$` is read for `$INCLUDE`, `$STATIC` and `$DYNAMIC`, as QB64pe reads it; any other
comment is only a comment.
<!-- src: compiler/pipeline "Comment metacommands are never ignored"; tests\reference\examples\rem_1 -->

`'$INCLUDE:'file'` includes a file after its line. A relative name is looked for in the including file's folder, then
from the compiler's folder; `$INCLUDEONCE` makes later inclusions of a file empty.
<!-- src: compiler/pipeline "Included files"; tests\reference\examples\meta-include_1 -->

A `$` line that is no metacommand QB64pe knows is a compile error.
<!-- src: compiler/pipeline "Metacommands the old compiler does not know" -->

## The preprocessor

`$IF`, `$ELSEIF`, `$ELSE` and `$END IF` keep or drop lines, in file order across included files; `$LET` sets a name
they test. `WIN`, `WINDOWS` and `LINUX` tell the platform the program is compiled on.
<!-- src: compiler/pipeline "Preprocessor"; tests\reference\examples\meta-if_1 -->

A condition may test the flags `_CONSOLE_`, `_EXPLICIT_`, `_ASSERTS_`, `_DEBUG_` and `_SOCKETS_`, which have one value
for the whole program.
<!-- src: compiler/pipeline "Precompiler flags" -->

An active `$ERROR` stops the compile with its message.
<!-- src: compiler/pipeline "Preprocessor" -->
