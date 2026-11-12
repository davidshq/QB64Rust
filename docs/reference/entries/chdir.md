# CHDIR

`CHDIR` makes a folder the current one, `MKDIR` makes a new folder and `RMDIR` removes an empty one.

## Syntax

```text
CHDIR path$
MKDIR path$
RMDIR path$
```

## Behavior

`MKDIR` makes the folder, `CHDIR` makes it the folder that relative file names start from, and `RMDIR` removes it.
`CHDIR ".."` goes up one folder.
<!-- src: language/file-io "File-system statements"; tests\reference\examples\chdir_1 -->

Each raises QB64pe's errors, as for a folder that does not exist or is not empty.
<!-- src: language/file-io "File-system statements" -->

## Availability

<!-- begin generated: availability -->
Supported. All 10 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:chdir_1 -->
```qb64
MKDIR "chdir_1_dir"
PRINT "Made:"; _DIREXISTS("chdir_1_dir")
CHDIR "chdir_1_dir"
OPEN "inside.tmp" FOR OUTPUT AS #1: CLOSE #1
KILL "inside.tmp"
CHDIR ".."
RMDIR "chdir_1_dir"
PRINT "Removed:"; _DIREXISTS("chdir_1_dir")
END
```

Output:

```text
Made:-1 
Removed: 0 

Press any key to continue
```
<!-- end generated -->

## See also

`_CWD$`, `_DIREXISTS`, `FILES`.
