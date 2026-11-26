' TEST: warnings
' `pds-function` (spec `compiler/lints`, "A BASIC PDS 7.1 function read as a variable"). Reported: `CURDIR$`
' twice (lines 11 and 12, the second in lower case), `DIR$` on line 13 and not `used-once`, and `STACK` in
' `Show`, a string by `DEFSTR S`, so its value is `""`. Not reported: `savepoint`, which an assignment sets;
' `DIR$(1)`, an array named once, which gets `used-once`; `curdir`, with no `$`, which is no PDS function's
' name and gets `used-once`. In `Fill`
' each name is set: `STACK` by `SWAP`, `DIR$` by a SUB's parameter, `CURDIR$` by `MID$`, `SAVEPOINT` by `GET`.
' In `Once`, `DIR$` is set and named once, so it gets `used-once`.
$CONSOLE:ONLY
DEFSTR S
here$ = CURDIR$
PRINT curdir$; here$
PRINT DIR$
savepoint = "3"
PRINT savepoint
PRINT DIR$(1)
PRINT curdir
Show
Fill
Once: Reads
SUB Show
    PRINT STACK
END SUB
SUB Fill
    SWAP STACK, other$
    PRINT STACK; other$
    SetIt DIR$
    PRINT DIR$
    MID$(CURDIR$, 1) = "x"
    PRINT CURDIR$
    GET #1, , SAVEPOINT
    PRINT SAVEPOINT
END SUB
SUB SetIt (s$)
    s$ = "set"
END SUB
SUB Once
    DIR$ = "once"
END SUB
SUB Reads
    ' Reported: `CURDIR$`, which `ShowText` only reads, and `STACK`, which `PUT` only reads.
    ShowText CURDIR$
    PUT #1, , STACK
    PRINT STACK
    ' Not reported: `SAVEPOINT`, which `Relay` passes on to `SetIt`, which sets it.
    Relay SAVEPOINT
    PRINT SAVEPOINT
END SUB
SUB ShowText (t$)
    PRINT t$
END SUB
SUB Relay (r$)
    SetIt r$
END SUB
