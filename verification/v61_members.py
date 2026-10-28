"""Writes the programs v61_m_<name>.bas (qbasic-testcases, task 1.1, design D4): a TYPE member named like a
keyword or a built-in, one program per name.

The old compiler refuses a member name ("Name already in use") when its reserved table holds it with
HASHFLAG_TYPE, HASHFLAG_CUSTOMSYNTAX, HASHFLAG_OPERATOR or HASHFLAG_XELEMENTNAME (`qb64pe.bas` 2313-2320; the
table 1250-1395), and takes every other name `validname` passes. The names here are every name of that table, and
names of built-in statements and functions (those the qbasic_testcases programs use as members, `Time` among them,
and others of QB 4.5). Each program declares the member beside a plain one, uses it on both sides of an assignment
and prints it. The recordings are the verification suite's (`verification/run.sh v61_m_`).

Usage (from the repo root): python verification/v61_members.py
"""

from pathlib import Path

HERE = Path(__file__).resolve().parent

# The reserved table of qb64pe.bas 1250-1395, in its order.
RESERVED = (
    # HASHFLAG_TYPE
    "_UNSIGNED _BIT _BYTE INTEGER LONG _INTEGER64 _OFFSET SINGLE DOUBLE _FLOAT STRING ANY "
    # HASHFLAG_OPERATOR
    "NOT IMP EQV AND OR XOR MOD _NEGATE _ANDALSO _ORELSE "
    # HASHFLAG_CUSTOMSYNTAX
    "LIST BASE _EXPLICIT _EXPLICITARRAY AS IS OFF ON STOP TO USING _DYNAMIC _STATIC PRESET PSET FOR OUTPUT RANDOM "
    "BINARY APPEND SHARED ACCESS LOCK READ WRITE STEP LPRINT PRINT "
    # HASHFLAG_XELEMENTNAME, or not (the entries written `f - HASHFLAG_XELEMENTNAME`)
    "COMMON CALL CASE COM CONST DATA DECLARE DEF DEFDBL DEFINT DEFLNG DEFSNG DEFSTR DIM DO ERROR ELSE ELSEIF ENDIF "
    "EXIT FIELD FUNCTION GOSUB GOTO INPUT IF KEY LET LOOP LEN NEXT PLAY PEN REDIM REM RESTORE RESUME RETURN RUN "
    "STATIC STRIG SEG SELECT SUB SCREEN THEN TIMER TYPE UNTIL UEVENT WEND WHILE"
).split()

BUILTINS = (
    "TIME DATE LEFT RIGHT MID COLOR NAME POINT CLS LINE CIRCLE BEEP SOUND LOCATE OPEN CLOSE GET PUT END SEEK SHELL "
    "WIDTH EOF ERASE OPTION SWAP WAIT VIEW WINDOW PALETTE PAINT DRAW INSTR RND INT FIX SGN ABS SQR LOG EXP SIN COS "
    "TAN ATN ASC CHR STR VAL HEX OCT CSRLIN POS FREEFILE LOF LOC INKEY SPACE ERR ERL STICK PEEK POKE OUT INP VARPTR "
    "VARSEG SADD ABSOLUTE INTERRUPT RESET CHDIR MKDIR FILES KILL RANDOMIZE SLEEP SYSTEM CLEAR LPOS FRE ENVIRON "
    "COMMAND UCASE LCASE LTRIM RTRIM CVI MKI X Y "
    # A single leading underscore, of a built-in and of no name (validname, qb64pe.bas 28273)
    "_WIDTH _MINE"
).split()


def program(name: str) -> str:
    low = name.lower()
    return (
        "$CONSOLE:ONLY\n"
        f"' Verification (qbasic-testcases, task 1.1; written by v61_members.py): a TYPE member named {name}.\n"
        "TYPE t\n"
        f"    {low} AS INTEGER\n"
        "    other AS INTEGER\n"
        "END TYPE\n"
        "DIM o AS t\n"
        f"o.{low} = 4: o.other = 2\n"
        f"o.{low} = o.{low} + o.other\n"
        f"PRINT o.{low}; o.other\n"
        "SYSTEM\n"
    )


def main():
    names = list(dict.fromkeys(RESERVED + BUILTINS))
    for name in names:
        (HERE / f"v61_m_{name.lower()}.bas").write_text(program(name), newline="\n")
    print(f"{len(names)} programs")


if __name__ == "__main__":
    main()
