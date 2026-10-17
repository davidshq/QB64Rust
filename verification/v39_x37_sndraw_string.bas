$CONSOLE:ONLY
' Verification (m3-sound-handles, task 1.1): one form, on the last line, as code that is not reached.
DIM i AS INTEGER, l AS LONG, q AS _INTEGER64, s AS SINGLE, d AS DOUBLE, t AS STRING, u AS _UNSIGNED LONG
DIM h AS LONG, r AS LONG, bv AS _BYTE, sv AS SINGLE
DIM w(1 TO 64) AS _BYTE, b(1 TO 16) AS SINGLE, iw(1 TO 8) AS INTEGER, uw(1 TO 8) AS _UNSIGNED _BYTE
DIM dd(1 TO 8) AS DOUBLE, ss(1 TO 8) AS STRING, lw(1 TO 8) AS LONG, bt(1 TO 8) AS _BIT
PRINT "ok"
SYSTEM
_SNDRAW t
