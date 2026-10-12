$CONSOLE:ONLY
' Verification (m2-builtin-statements, D1 "Files"): OPEN in every mode, access and lock word, LEN =, with and without
' #, the old form with each mode letter and a bad one; file numbers that are floats, 0, negative, 256, already open,
' beyond LONG; CLOSE with several numbers, a float, a number that is not open. Files are named v22_b_* and removed.
ON ERROR GOTO h
DIM f AS STRING, n AS LONG, d AS DOUBLE, q AS _INTEGER64
f = "v22_b_o.tmp"
PRINT "OUTPUT": OPEN f FOR OUTPUT AS #1: PRINT #1, "one": CLOSE #1
PRINT "APPEND": OPEN f FOR APPEND AS #1: PRINT #1, "two": CLOSE #1
PRINT "INPUT": OPEN f FOR INPUT AS #1: LINE INPUT #1, a$: PRINT "["; a$; "]": CLOSE #1
PRINT "BINARY": OPEN f FOR BINARY AS #1: PRINT LOF(1): CLOSE #1
PRINT "RANDOM": OPEN f FOR RANDOM AS #1: PRINT LOF(1): CLOSE #1
PRINT "no mode (RANDOM)": OPEN f AS #1: PRINT LOF(1): CLOSE #1
PRINT "RANDOM LEN = 4": OPEN f FOR RANDOM AS #1 LEN = 4: PRINT LOF(1): CLOSE #1
PRINT "INPUT LEN = 100": OPEN f FOR INPUT AS #1 LEN = 100: CLOSE #1
PRINT "ACCESS READ": OPEN f FOR BINARY ACCESS READ AS #1: CLOSE #1
PRINT "ACCESS WRITE": OPEN f FOR BINARY ACCESS WRITE AS #1: CLOSE #1
PRINT "ACCESS READ WRITE": OPEN f FOR BINARY ACCESS READ WRITE AS #1: CLOSE #1
PRINT "SHARED": OPEN f FOR INPUT SHARED AS #1: CLOSE #1
PRINT "LOCK READ": OPEN f FOR INPUT LOCK READ AS #1: CLOSE #1
PRINT "LOCK WRITE": OPEN f FOR INPUT LOCK WRITE AS #1: CLOSE #1
PRINT "LOCK READ WRITE": OPEN f FOR INPUT LOCK READ WRITE AS #1: CLOSE #1
PRINT "ACCESS READ LOCK WRITE": OPEN f FOR INPUT ACCESS READ LOCK WRITE AS #1: CLOSE #1
PRINT "INPUT ACCESS WRITE": OPEN f FOR INPUT ACCESS WRITE AS #1: CLOSE #1
PRINT "OUTPUT ACCESS READ": OPEN f FOR OUTPUT ACCESS READ AS #1: CLOSE #1
PRINT "without #": OPEN f FOR INPUT AS 1: CLOSE 1
PRINT "number from a variable": n = 3: OPEN f FOR INPUT AS n: PRINT EOF(3): CLOSE n
PRINT "number 1.5 (2)": OPEN f FOR INPUT AS #1.5: PRINT EOF(2): CLOSE #2
PRINT "number 2.5 (2)": d = 2.5: OPEN f FOR INPUT AS d: PRINT EOF(2): CLOSE d
PRINT "number 0": OPEN f FOR INPUT AS #0
PRINT "number -1": OPEN f FOR INPUT AS #-1
PRINT "number 255": OPEN f FOR INPUT AS #255: CLOSE #255
PRINT "number 256": OPEN f FOR INPUT AS #256: CLOSE #256
PRINT "number 70000": OPEN f FOR INPUT AS #70000: CLOSE #70000
PRINT "number 4294967297 (1 in 32 bits)": q = 4294967297: OPEN f FOR INPUT AS q: PRINT EOF(1): CLOSE
PRINT "already open": OPEN f FOR INPUT AS #1: OPEN f FOR INPUT AS #1: CLOSE
PRINT "same file twice for input": OPEN f FOR INPUT AS #1: OPEN f FOR INPUT AS #2: CLOSE
PRINT "same file for output while open": OPEN f FOR INPUT AS #1: OPEN f FOR OUTPUT AS #2: CLOSE
PRINT "missing for INPUT": OPEN "v22_b_none.tmp" FOR INPUT AS #1
PRINT "missing for BINARY (created)": OPEN "v22_b_new.tmp" FOR BINARY AS #1: CLOSE: KILL "v22_b_new.tmp"
PRINT "missing for APPEND (created)": OPEN "v22_b_new.tmp" FOR APPEND AS #1: CLOSE: KILL "v22_b_new.tmp"
PRINT "empty name": OPEN "" FOR INPUT AS #1
PRINT "raising name": OPEN CHR$(-1) FOR OUTPUT AS #1
PRINT "raising number": OPEN f FOR INPUT AS ASC("")
PRINT "LEN 0": OPEN f FOR RANDOM AS #1 LEN = 0: CLOSE
PRINT "LEN -1": OPEN f FOR RANDOM AS #1 LEN = -1: CLOSE
PRINT "LEN 2.5": OPEN f FOR RANDOM AS #1 LEN = 2.5: CLOSE
PRINT "old form O": OPEN "O", #1, f: PRINT #1, "old": CLOSE
PRINT "old form o": OPEN "o", 1, f: PRINT #1, "old": CLOSE
PRINT "old form A": OPEN "A", #1, f: PRINT #1, "more": CLOSE
PRINT "old form I": OPEN "I", #1, f: LINE INPUT #1, a$: PRINT "["; a$; "]": CLOSE
PRINT "old form B": OPEN "B", #1, f: PRINT LOF(1): CLOSE
PRINT "old form R": OPEN "R", #1, f: PRINT LOF(1): CLOSE
PRINT "old form R with length": OPEN "R", #1, f, 4: PRINT LOF(1): CLOSE
PRINT "old form X": OPEN "X", #1, f: CLOSE
PRINT "old form empty": OPEN "", #1, f: CLOSE
PRINT "old form INPUT (first letter)": OPEN "INPUT", #1, f: CLOSE
PRINT "old form variable": m$ = "I": OPEN m$, #1, f: CLOSE
PRINT "CLOSE several": OPEN f FOR INPUT AS #1: OPEN f FOR INPUT AS #2: OPEN f FOR INPUT AS #3
CLOSE #1, 2, #3
OPEN f FOR INPUT AS #1: PRINT "all three were closed": CLOSE
PRINT "CLOSE 1.5 closes 2": OPEN f FOR INPUT AS #1: OPEN f FOR INPUT AS #2: CLOSE 1.5
OPEN f FOR INPUT AS #2: PRINT "2 was closed": OPEN f FOR INPUT AS #1: CLOSE
PRINT "CLOSE not open": CLOSE #5: CLOSE 0: CLOSE -1: CLOSE 300: PRINT "no error"
PRINT "CLOSE with a raising number": CLOSE ASC("")
PRINT "CLOSE bare with nothing open": CLOSE: PRINT "no error"
KILL f
SYSTEM
h:
PRINT "  error"; ERR
RESUME NEXT
