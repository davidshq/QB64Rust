# KILL

`KILL name$` deletes a file.

## Syntax

```text
KILL name$
```

## Behavior

`KILL` deletes the file. A file that does not exist raises error 53.
<!-- src: language/file-io "File-system statements"; tests\reference\examples\kill_1 -->

## Errors

- A missing file raises run-time error 53.
  <!-- src: language/file-io "File-system statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 8 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:kill_1 -->
```qb64
OPEN "kill_1.tmp" FOR OUTPUT AS #1: CLOSE #1
PRINT "Before KILL:"; _FILEEXISTS("kill_1.tmp")
KILL "kill_1.tmp"
PRINT "After KILL:"; _FILEEXISTS("kill_1.tmp")
ON ERROR GOTO handler
KILL "kill_1.tmp"
END

handler:
PRINT "KILL of a missing file: error"; ERR
RESUME NEXT
```

Output:

```text
Before KILL:-1 
After KILL: 0 
KILL of a missing file: error 53 

Press any key to continue
```
<!-- end generated -->

## See also

[`NAME`](name.md), [`RMDIR`](chdir.md), [`_FILEEXISTS`](_fileexists.md).
