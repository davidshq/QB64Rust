# First steps

This page is for someone who has never written a program. It shows a first program, then the few things most
programs are made of: variables, questions to the user, choices, loops, SUBs and drawing.

## A program

A program is a list of statements that the computer carries out one after another, from the top. This one
prints three lines:

<!-- begin generated: example:first-steps/hello -->
```qb64
' Hello: a first program. Each PRINT writes one line.
PRINT "Hello, world!"
PRINT "Two and three make"; 2 + 3
language$ = "QB64"
PRINT "This program was written in "; language$; "."
END
```

Output:

```text
Hello, world!
Two and three make 5 
This program was written in QB64.

Press any key to continue
```
<!-- end generated -->

To run it in VS Code, open the QB64 side panel and click **Run**. "Hello" is also in the Examples view, under
"First steps".

- A line that starts with `'` is a comment. The computer skips it; it is there for people.
- [`PRINT`](entries/print.md) writes text in quotes as it is, and works out a sum such as `2 + 3` before it
  writes it. A `;` between two items writes them one after the other.
- [`END`](entries/end.md) ends the program. In a program that prints to the terminal, it then waits for a key.

The first line of the program file is `$CONSOLE:ONLY`, which this page leaves out. It makes the program print in
VS Code's terminal instead of a window of its own.

## Variables

A variable is a name that holds a value. `language$ = "QB64"` above puts the text `QB64` into the variable
`language$`, and the next line prints it. A name that ends with `$` holds text. A name without it holds a number:
`score = 10`, then `score = score + 1` makes it 11.

[`DIM`](entries/dim.md) gives a variable a type of its own, such as `DIM score AS LONG` for a whole number. A
small program can do without it. The chapter [Default types](chapters/default-types.md) says what type a name
gets when `DIM` does not give one.

## Asking, choosing and repeating

This program thinks of a number and lets the user guess it:

<!-- begin generated: example:first-steps/guess -->
```qb64
' Guess the number: the program thinks of a number from 1 to 100, and you guess it.
RANDOMIZE TIMER
secret = INT(RND * 100) + 1
tries = 0
PRINT "I am thinking of a number from 1 to 100."
DO
    INPUT "Your guess"; guess
    tries = tries + 1
    IF guess < secret THEN
        PRINT "Higher!"
    ELSEIF guess > secret THEN
        PRINT "Lower!"
    END IF
LOOP UNTIL guess = secret
PRINT "Right! You found it in"; tries; "tries."
END
```

Not recorded yet.
<!-- end generated -->

- [`RND`](entries/rnd.md) gives a random number from 0 up to, but not including, 1, and [`INT`](entries/int.md)
  rounds a number down to a whole number. `INT(RND * 100) + 1` is a whole number from 1 to 100.
  [`RANDOMIZE TIMER`](entries/randomize.md) makes the numbers different each time the program runs.
- [`INPUT`](entries/input.md) prints its question, waits for the user to type a value and press Enter, and puts
  the value into the variable `guess`.
- [`IF`](entries/if.md) runs the lines after `THEN` only when its condition is true. `ELSEIF` gives a second
  condition, tried when the first one is false. `END IF` ends the choice.
- [`DO` and `LOOP`](entries/do.md) repeat the lines between them. `LOOP UNTIL guess = secret` stops repeating
  once the guess is right. [`FOR`](entries/for.md) repeats a set number of times instead:
  `FOR i = 1 TO 10` … `NEXT`.

The output is not shown here because it is different each time. Run the program to play it.

## SUBs and drawing

A program with a window can draw. This one moves a ball and bounces it off the edges:

<!-- begin generated: example:graphics/bounce -->
```qb64
' Bouncing ball: a ball moves across the window and bounces off its edges. Press Esc to stop.
SCREEN _NEWIMAGE(640, 480, 32)
_TITLE "Bouncing ball"
x = 320: y = 240
dx = 4: dy = 3
radius = 20
DO
    CLS
    x = x + dx
    y = y + dy
    IF x < radius OR x > 640 - radius THEN dx = -dx
    IF y < radius OR y > 480 - radius THEN dy = -dy
    DrawBall x, y, radius
    _DISPLAY
    _LIMIT 60
LOOP UNTIL INKEY$ = CHR$(27)
SYSTEM

' Draws a filled ball with its center at (x, y).
SUB DrawBall (x, y, r)
    CIRCLE (x, y), r, _RGB32(255, 200, 0)
    PAINT (x, y), _RGB32(255, 200, 0), _RGB32(255, 200, 0)
END SUB
```

Not recorded yet.
<!-- end generated -->

- [`SCREEN _NEWIMAGE(640, 480, 32)`](entries/_newimage.md) opens a window 640 points wide and 480 high. A point
  is named by two numbers, `x` from the left and `y` from the top.
- [`SUB DrawBall`](entries/sub.md) gives a name to a few lines. The line `DrawBall x, y, radius` runs them, with
  the SUB's `x`, `y` and `r` set to the values given. A SUB goes after the main program.
- [`CIRCLE`](entries/circle.md) draws a circle and [`PAINT`](entries/paint.md) fills it.
  [`_RGB32`](entries/_rgb32.md) makes a color from its red, green and blue parts, each from 0 to 255.
- [`CLS`](entries/cls.md) clears the window, [`_DISPLAY`](entries/_display.md) shows what was drawn at once, and
  [`_LIMIT 60`](entries/_limit.md) keeps the loop to 60 times a second, so the ball moves at the same speed on
  every computer.
- [`INKEY$`](entries/inkey-str.md) gives the key the user pressed, if any. `CHR$(27)` is the Esc key.
  [`SYSTEM`](entries/system.md) ends the program and closes its window at once.

## Where to go next

- The Examples view has more programs to run and change: graphics, sound and small games. Open one, change a
  number, and run it again to see what the change does.
- Each word has a page: put the cursor on a word in the editor and run **QB64: Help for This Word**, or look it
  up in the Help view.
- When a program has a mistake, the Program view shows the first error and the line it is on.
- The [chapters](README.md) tell how each area of the language works as a whole, such as
  [Control flow](chapters/control-flow.md) and [Procedures](chapters/procedures.md).
