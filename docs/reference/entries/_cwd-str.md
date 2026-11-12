# _CWD$

`_CWD$` gives the current folder, where relative file names start. `_STARTDIR$` gives the folder the program started
in.

## Syntax

<!-- begin generated: syntax -->
```text
_CWD$
_STARTDIR$
```

From the built-in table (`tools\builtins\builtins.json`).
<!-- end generated -->

## Result

A STRING, the folder's full path.

## Behavior

`_CWD$` is the current folder; `CHDIR` changes it. At the start it is `_STARTDIR$`.
<!-- src: language/file-io "File functions"; tests\reference\examples\_cwd-str_1 -->

`_CWD$` and `_STARTDIR$` are written without parentheses.
<!-- src: language/builtin-functions "Built-ins without parentheses" -->

## Availability

<!-- begin generated: availability -->
Supported. All 3 programs that use it compile (call-site programs, examples and probes).
<!-- end generated -->

## Example

<!-- begin generated: example:_cwd-str_1 -->
```qb64
PRINT "_CWD$ is a folder:"; _DIREXISTS(_CWD$)
PRINT "At the start, it is the start folder:"; _CWD$ = _STARTDIR$
END
```

Output:

```text
_CWD$ is a folder:-1 
At the start, it is the start folder:-1 

Press any key to continue
```
<!-- end generated -->

## See also

`CHDIR`, `_DIR$`, `_FULLPATH$`.
