' TEST: warnings
' `unused-procedure` (spec `compiler/lints`, "Unused procedure"): only `Spare` and `Twice` are reported.
$CONSOLE:ONLY
DECLARE SUB Spare ()
DECLARE FUNCTION Used% (n AS INTEGER)
DECLARE LIBRARY
    FUNCTION labs& (BYVAL v AS LONG)
END DECLARE
ON TIMER(1) Tick
PRINT Used%(3)
CALL Called
SUB Spare
END SUB
FUNCTION Used% (n AS INTEGER)
    Used% = n
END FUNCTION
FUNCTION Fact& (n AS LONG)
    IF n > 1 THEN Fact& = n * Fact&(n - 1) ELSE Fact& = 1
END FUNCTION
FUNCTION Twice% (n AS INTEGER)
    Twice% = 2 * n
END FUNCTION
SUB Tick
END SUB
SUB Called
END SUB
SUB _GL
END SUB
