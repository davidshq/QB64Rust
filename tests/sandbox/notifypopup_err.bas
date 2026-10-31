' _NOTIFYPOPUP shows a notification nothing can read back: ERR after it, and that the program goes on
' (tests/sandbox/README.md).
OPEN "out.txt" FOR OUTPUT AS #1
ON ERROR GOTO failed

_NOTIFYPOPUP "QB64Rust", "A test of the sandbox family", "info"
PRINT #1, "ERR after _NOTIFYPOPUP:"; e
_NOTIFYPOPUP "QB64Rust", "Another, as a warning", "warning"
PRINT #1, "ERR after the second:"; e
PRINT #1, "went on"
CLOSE #1
SYSTEM

failed:
e = ERR
RESUME NEXT
