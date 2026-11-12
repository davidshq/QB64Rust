# The mouse

A program reads the mouse as a queue of events: `_MOUSEINPUT` takes the next one, and the other functions describe it.
Statements show, hide, shape and move the cursor.

The pages of this area: [_MOUSEINPUT](../entries/_mouseinput.md), [_MOUSEX](../entries/_mousex.md),
[_MOUSEBUTTON](../entries/_mousebutton.md), [_MOUSEWHEEL](../entries/_mousewheel.md),
[_MOUSEMOVEMENTX](../entries/_mousemovementx.md), [_MOUSEHIDDEN](../entries/_mousehidden.md),
[_MOUSEDISABLED](../entries/_mousedisabled.md), [_MOUSESHOW](../entries/_mouseshow.md),
[_MOUSECURSOR](../entries/_mousecursor.md), [_MOUSEMOVE](../entries/_mousemove.md).

## The queue

Each move, press, release and turn of the wheel is an event. `_MOUSEINPUT` gives -1 while it takes one and 0 when the
queue is empty, so `DO WHILE _MOUSEINPUT` reads them all and `WHILE _MOUSEINPUT: WEND` keeps only the latest.
<!-- src: language/mouse "The mouse queue"; tests\corpus\screen\u01_mouse_queue -->

`_MOUSEX`, `_MOUSEY`, `_MOUSEBUTTON`, `_MOUSEWHEEL` and the movement functions describe the event last taken: a click
is two events, one with the button down and one with it up.
<!-- src: language/mouse "The mouse queue"; tests\corpus\screen\u01_mouse_queue -->

## Where the mouse is

In a graphics mode `_MOUSEX` and `_MOUSEY` give a pixel; in a text mode a column and a row, kept inside the screen.
<!-- src: language/mouse "Position in text and graphics modes"; tests\corpus\screen\u02_mouse_text -->

In a `$CONSOLE:ONLY` program they read the console's mouse, which gives 0 where nothing moves it.
<!-- src: tests\reference\examples\_mousex_1 -->

## The cursor

`_MOUSEHIDE`, `_MOUSESHOW` and `_MOUSECURSOR` change the cursor and `_MOUSEMOVE` moves it. Their effect is outside the
screen's state, so the tests run them but compare nothing of it.
<!-- src: language/mouse "Mouse built-ins that no test runs" -->
