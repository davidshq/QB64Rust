# Run-time errors

<!-- Written by tools/reference/reference.py render; do not edit. -->

The run-time errors and their messages, as libqb gives them (`ERR` and `_ERRORMESSAGE$`), from
`libqb/internal/c/libqb/src/error_handle.cpp`. Any other number prints "Unprintable error". Errors 300 to 315 end the
program even under `ON ERROR` ([Memory blocks](chapters/mem-blocks.md),
[Assertions and $DEBUG](chapters/asserts-and-debug.md)).

| Number | Message |
|---|---|
| 1 | NEXT without FOR |
| 2 | Syntax error |
| 3 | RETURN without GOSUB |
| 4 | Out of DATA |
| 5 | Illegal function call |
| 6 | Overflow |
| 7 | Out of memory |
| 8 | Label not defined |
| 9 | Subscript out of range |
| 10 | Duplicate definition |
| 11 | Division by zero |
| 12 | Illegal in direct mode |
| 13 | Type mismatch |
| 14 | Out of string space |
| 16 | String formula too complex |
| 17 | Cannot continue |
| 18 | Function not defined |
| 19 | No RESUME |
| 20 | RESUME without error |
| 24 | Device timeout |
| 25 | Device fault |
| 26 | FOR without NEXT |
| 27 | Out of paper |
| 29 | WHILE without WEND |
| 30 | WEND without WHILE |
| 33 | Duplicate label |
| 35 | Subprogram not defined |
| 37 | Argument-count mismatch |
| 38 | Array not defined |
| 40 | Variable required |
| 50 | FIELD overflow |
| 51 | Internal error |
| 52 | Bad file name or number |
| 53 | File not found |
| 54 | Bad file mode |
| 55 | File already open |
| 56 | FIELD statement active |
| 57 | Device I/O error |
| 58 | File already exists |
| 59 | Bad record length |
| 61 | Disk full |
| 62 | Input past end of file |
| 63 | Bad record number |
| 64 | Bad file name |
| 67 | Too many files |
| 68 | Device unavailable |
| 69 | Communication-buffer overflow |
| 70 | Permission denied |
| 71 | Disk not ready |
| 72 | Disk-media error |
| 73 | Feature unavailable |
| 74 | Rename across disks |
| 75 | Path/File access error |
| 76 | Path not found |
| 258 | Invalid handle |
| 300 | Memory region out of range |
| 301 | Invalid size |
| 302 | Source memory region out of range |
| 303 | Destination memory region out of range |
| 304 | Source and destination memory regions out of range |
| 305 | Source memory has been freed |
| 306 | Destination memory has been freed |
| 307 | Memory already freed |
| 308 | Memory has been freed |
| 309 | Memory not initialized |
| 310 | Source memory not initialized |
| 311 | Destination memory not initialized |
| 312 | Source and destination memory not initialized |
| 313 | Source and destination memory have been freed |
| 314 | _ASSERT failed |
| 315 | _ASSERT failed (check console for description) |
