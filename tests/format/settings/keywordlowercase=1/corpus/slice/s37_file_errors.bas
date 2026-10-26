$console:only
' Runtime errors of the file statements under a handler (spec language/file-io, language/builtin-statements "Runtime
' errors"): 52 for a number that is not open, 53 for a missing file, 54 for the wrong mode, 55 for a number that is
' open, 62 past the end, 6 for a value outside the target's range, 63 for a bad position, 75 and 76 from the file
' system, 5 for a raising argument. A statement with several items stops at the first that raises; what was written
' or read before it stays. Files are s37_*.tmp and removed again.
on error goto handler
dim f as string, a as string, c as string, n as long, b as _byte, ub as _unsigned _byte
dim arr(3) as long, sa(3) as string, retry as long
f = "s37_errors.tmp"
print "not open"
print #3, "x"
write #3, "x"
input #3, a
line input #3, a
seek 3, 1
print eof(3); lof(3); loc(3); seek(3)
print "file numbers 0 and -1"
open f for output as #0
open f for output as #-1
print #0, "x"
print eof(0)
print "CLOSE of numbers that are not open is no error"
close #5: close 0, -1, 300
close
print "a missing file"
open "s37_none.tmp" for input as #1
open "" for input as #1
print "a number that is open"
open f for output as #1
open f for output as #1
print "the wrong mode"
input #1, a
line input #1, a
print #1, "one"
print #1, "200,7"
print #1, "-1,8"
print #1, "31,32,33"
print #1, "s1,s2,s3"
print #1, "ra"; chr$(-1); "rb"
print #1, "after the raising item"
write #1, "wa", chr$(-1), "wb"
write #1, "after the raising item"
close #1
open f for input as #1
print #1, "x"
write #1, "x"
print "the old form with a bad mode letter"
open "X", #2, f
open "", #2, f
print "raising arguments"
open chr$(-1) for input as #2
open f for input as asc("")
close asc("")
print #asc(""), "x"
input #asc(""), a
seek 1, asc("")
print eof(asc(""))
print "a bad position"
seek 1, 0
seek 1, -5
print "reading"
line input #1, a: print "["; a; "]"
print "200 into a _BYTE: 0 is stored and the next target is not read"
b = 1: n = 1
input #1, b, n
print b; n
input #1, a: print "the next field is ["; a; "]"
print "-1 into an unsigned type"
ub = 1
input #1, ub, n
print ub; n
line input #1, a: print "the rest of the line is ["; a; "]"
print "a target with a bad index reads nothing and ends the statement"
n = 0: b = 0
input #1, n, arr(9), b
print n; b
line input #1, a: print "the rest of the line is ["; a; "]"
a = "": c = ""
input #1, a, sa(9), c
print "["; a; "]["; c; "]["; sa(0); "]"
line input #1, a: print "the rest of the line is ["; a; "]"
print "what the raising items left in the file"
line input #1, a: print "["; a; "]"
line input #1, a: print "["; a; "]"
print "past the end"; eof(1)
a = "kept": n = 5
line input #1, a: print "["; a; "]"
input #1, a, c
input #1, n: print n
close
print "RESUME runs the OPEN again, after the handler made the file"
retry = 1
open "s37_made.tmp" for input as #1
print "opened; the handler ran"; retry - 1; "time"
retry = 0
close
kill "s37_made.tmp"
print "the file system"
open f for input as #1
kill f
name f as "s37_other.tmp"
close
mkdir "s37_dir"
open "s37_dir/a.tmp" for output as #1: close
rmdir "s37_dir"
kill "s37_dir"
kill "s37_dir/*.tmp"
kill "s37_dir/*.tmp"
rmdir "s37_dir"
kill f
print "in a SUB"
insub
system
handler:
print "  error"; err
if err = 53 and retry = 1 then
    retry = 2
    open "s37_made.tmp" for output as #9: close #9
    resume
end if
resume next

sub insub
    dim t as string
    print #7, "x"
    print "  after PRINT # in the SUB"
    line input #7, t
    close #7
    print "  end of the SUB"
end sub
