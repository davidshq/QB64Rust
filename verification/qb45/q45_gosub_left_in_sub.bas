' QB 4.5 and PDS 7.1 (DIVERGENCES-QB45.md Q-018): a SUB leaves its own GOSUB by GOTO and END SUB,
' without RETURN, while the main module's GOSUB is pending. Then the main module's RETURN.
' QB64pe and QB64Rust keep one GOSUB stack for the whole program, so that RETURN takes the SUB's
' entry and stops with "RETURN without GOSUB". The World War 2 game's editor (EDIT.BAS) relied on
' the old compilers' result: its menu SUB leaves GOSUB selex by GOTO.
DECLARE SUB leaver ()
OPEN "OUT.TXT" FOR OUTPUT AS #1
ON ERROR GOTO trap
PRINT #1, "start"
GOSUB ask
PRINT #1, "back in main"
CLOSE #1
SYSTEM
ask:
CALL leaver
PRINT #1, "after the CALL"
RETURN
trap:
PRINT #1, "error"; ERR
CLOSE #1
SYSTEM

SUB leaver
GOSUB inner
PRINT #1, "not reached"
inner:
GOTO done
done:
PRINT #1, "leaving the SUB"
END SUB
