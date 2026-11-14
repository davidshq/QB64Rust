# The console and the window

A program has a window, a console, or both. `$CONSOLE` gives it a console beside its window, `$CONSOLE:ONLY` a console
alone, and `_DEST` and `_SOURCE` choose where text goes and comes from.

The pages of this area: [$CONSOLE](../entries/meta-console.md), [_CONSOLE](../entries/_console.md),
[_DEST](../entries/_dest.md), [_SOURCE](../entries/_source.md), [_ECHO](../entries/_echo.md),
[_CONSOLETITLE](../entries/_consoletitle.md), [_CONSOLEINPUT](../entries/_consoleinput.md),
[$SCREENHIDE](../entries/meta-screenhide.md), [_SCREENHIDE](../entries/_screenhide.md),
[_TITLE](../entries/_title.md), [_EXIT](../entries/_exit.md), [_RESIZE](../entries/_resize.md),
[$RESIZE](../entries/meta-resize.md), [_FULLSCREEN](../entries/_fullscreen.md),
[_ACCEPTFILEDROP](../entries/_acceptfiledrop.md), [_ALLOWFULLSCREEN](../entries/_allowfullscreen.md),
[_CLIPBOARD$](../entries/_clipboard-str.md), [_CLIPBOARDIMAGE](../entries/_clipboardimage.md),
[_COLORCHOOSERDIALOG](../entries/_colorchooserdialog.md), [_COMMANDCOUNT](../entries/_commandcount.md),
[_COMPILEDATE$](../entries/_compiledate-str.md), [_COMPILERVERSION$](../entries/_compilerversion-str.md),
[_DESKTOPREFRESHRATE](../entries/_desktoprefreshrate.md), [_DESKTOPWIDTH](../entries/_desktopwidth.md),
[_ENVIRONCOUNT](../entries/_environcount.md), [_ICON](../entries/_icon.md), [_INPUTBOX$](../entries/_inputbox-str.md),
[_MESSAGEBOX](../entries/_messagebox.md), [_NOTIFYPOPUP](../entries/_notifypopup.md),
[_OPENFILEDIALOG$](../entries/_openfiledialog-str.md), [_OS$](../entries/_os-str.md),
[_SCALEDWIDTH](../entries/_scaledwidth.md), [_SCREENCLICK](../entries/_screenclick.md),
[_SCREENEXISTS](../entries/_screenexists.md), [_SCREENICON](../entries/_screenicon.md),
[_SCREENIMAGE](../entries/_screenimage.md), [_SCREENMOVE](../entries/_screenmove.md),
[_SCREENPRINT](../entries/_screenprint.md), [_SCREENX](../entries/_screenx.md),
[_SHELLHIDE](../entries/_shellhide.md), [_UPTIME](../entries/_uptime.md),
[_WINDOWHANDLE](../entries/_windowhandle.md), [_WINDOWHASFOCUS](../entries/_windowhasfocus.md),
[_WINDOWSIZELIMIT](../entries/_windowsizelimit.md), [COMMAND$](../entries/command-str.md).

## Console and window

A program has a console when `$CONSOLE` or `$CONSOLE:ONLY` stands anywhere in its compiled code. `$CONSOLE:ONLY` makes
the console the destination and the source, and the program one without a window.
<!-- src: language/console-and-window "A program with a console"; language/console-and-window "$CONSOLE:ONLY" -->

`_DEST` and `_SOURCE` choose the image or console that text goes to and comes from; `_ECHO` writes to the console
whatever the destination.
<!-- src: language/console-and-window "_DEST and _SOURCE"; language/console-and-window "_ECHO" -->

When a console program's standard input has ended, `INPUT`, `LINE INPUT` and `INPUT$` raise error 62 instead of
waiting.
<!-- src: language/console-and-window "Console input at the end of the input"; D-048 -->

## The window

The window built-ins compile to QB64pe's calls. What they do to a window is outside what the tests compare, and no
test opens a dialog or moves the window.
<!-- src: language/console-and-window "Window built-ins are compiled"; language/console-and-window "Dialogs and the window's place are compiled" -->

`END` in a program with a console prints "Press any key to continue" and waits for a key only when its input is a
console.
<!-- src: language/console-and-window "The end of a program and errors with a hidden window" -->
