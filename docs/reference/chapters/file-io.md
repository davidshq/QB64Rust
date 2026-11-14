# Files

A program opens a file by number and reads or writes it as text, with `PRINT #`, `WRITE #`, `INPUT #` and `LINE
INPUT #`. Other statements list, rename and remove files and folders.

The pages of this area: [OPEN](../entries/open.md), [CLOSE](../entries/close.md), [FREEFILE](../entries/freefile.md),
[EOF](../entries/eof.md), [LOF](../entries/lof.md), [LOC](../entries/loc.md), [SEEK](../entries/seek.md),
[INPUT](../entries/input.md), [LINE INPUT](../entries/line-input.md), [WRITE](../entries/write.md),
[PRINT](../entries/print.md), [KILL](../entries/kill.md), [NAME](../entries/name.md), [CHDIR](../entries/chdir.md),
[FILES](../entries/files.md), [_FILES$](../entries/_files-str.md), [_FILEEXISTS](../entries/_fileexists.md),
[_READFILE$](../entries/_readfile-str.md), [_FULLPATH$](../entries/_fullpath-str.md), [_CWD$](../entries/_cwd-str.md),
[RESET](../entries/reset.md), [_DIR$](../entries/_dir-str.md), [FILEATTR](../entries/fileattr.md),
[IOCTL](../entries/ioctl.md).

## Opening and closing

`OPEN name$ FOR mode AS #n` opens a file for `INPUT`, `OUTPUT`, `APPEND`, `BINARY` or `RANDOM`, and `CLOSE` closes it;
`FREEFILE` gives a number not in use.
<!-- src: language/file-io "OPEN and CLOSE"; tests\reference\examples\open_1 -->

## Text

`PRINT #n` writes what `PRINT` would print; `WRITE #n` writes items between commas, strings in quotes.
<!-- src: language/file-io "PRINT to a file"; language/file-io "WRITE"; tests\reference\examples\write_1 -->

`LINE INPUT #n` reads a line without its line end; `INPUT #n` reads one field per target, a field ending at a comma or
a line end.
<!-- src: language/file-io "INPUT and LINE INPUT from a file"; tests\reference\examples\line-input_1 -->

`EOF`, `LOF`, `LOC` and `SEEK` give where a file stands and how long it is.
<!-- src: language/file-io "File functions"; tests\reference\examples\eof_1 -->

## The file system

`KILL`, `NAME`, `MKDIR`, `RMDIR` and `CHDIR` act on files and folders, and `_FILES$` and `FILES` list a folder.
<!-- src: language/file-io "File-system statements"; language/file-io "Listing a folder with _FILES$" -->

`_READFILE$` and `_WRITEFILE` read and write a whole file at once.
<!-- src: language/file-io "Whole files and full paths"; tests\reference\examples\_readfile-str_1 -->
