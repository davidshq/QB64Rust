# QB64Rust language reference

<!-- Written by tools/reference/reference.py render; do not edit. -->

One page per statement, function, keyword and metacommand. Each page says what QB64Rust does, which
errors it gives, and where it differs from QB64pe 4.7.0 and QuickBASIC 4.5. Every rule on a page names
its source, and every example is a test program whose output was recorded (`tools\reference\README.md`).

The examples start with `$CONSOLE:ONLY` so that their output can be recorded; the pages leave that line
out. Run in a window, an example prints the same text there.

400 pages written; the inventory names 400.

## Chapters

How each area of the language works as a whole.

- [Arrays and user types](chapters/arrays-and-types.md)
- [Assertions and $DEBUG](chapters/asserts-and-debug.md)
- [Built-in functions](chapters/builtin-functions.md)
- [Built-in statements](chapters/builtin-statements.md)
- [The console and the window](chapters/console-and-window.md)
- [Constants](chapters/constants.md)
- [Control flow](chapters/control-flow.md)
- [Conventional memory](chapters/conventional-memory.md)
- [Checksums, compression and base64](chapters/data-processing.md)
- [DATA, READ and RESTORE](chapters/data-read.md)
- [Default types](chapters/default-types.md)
- [Game controllers and input devices](chapters/device-input.md)
- [Embedded files](chapters/embedded-files.md)
- [Handling errors](chapters/error-handling.md)
- [The order of evaluation](chapters/evaluation-order.md)
- [Events](chapters/events.md)
- [Libraries](chapters/external-libraries.md)
- [Files](chapters/file-io.md)
- [Fixed-length strings](chapters/fixed-length-strings.md)
- [Fonts](chapters/fonts.md)
- [Images](chapters/images.md)
- [The keyboard](chapters/keyboard.md)
- [Logging](chapters/logging.md)
- [Memory blocks](chapters/mem-blocks.md)
- [The mouse](chapters/mouse.md)
- [The network](chapters/network.md)
- [Numbers](chapters/numeric-semantics.md)
- [OpenGL](chapters/opengl.md)
- [The program's text](chapters/pipeline.md)
- [PRINT USING](chapters/print-using.md)
- [Procedures](chapters/procedures.md)
- [Binary and random files](chapters/record-files.md)
- [Run-time checks](chapters/run-time-checks.md)
- [The screen](chapters/screen-output.md)
- [Sound](chapters/sound.md)
- [Unstable features](chapters/unstable-features.md)

## By area

- **Lines, names, comments, labels, $INCLUDE, the preprocessor**: [$ERROR](entries/meta-error.md), [$ERRORLOCATION](entries/meta-errorlocation.md), [$IF](entries/meta-if.md), [$INCLUDE](entries/meta-include.md), [$INCLUDEONCE](entries/meta-includeonce.md), [$LET](entries/meta-let.md), [$NOPREFIX](entries/meta-noprefix.md), [REM](entries/rem.md)
- **Types, literals, operators, conversions**: [_ANDALSO](entries/_andalso.md), [_BIT](entries/_bit.md), [_BYTE](entries/_byte.md), [_CAST](entries/_cast.md), [_DEFINE](entries/_define.md), [_FLOAT](entries/_float.md), [_INTEGER64](entries/_integer64.md), [_NEGATE](entries/_negate.md), [_OFFSET](entries/_offset.md), [_ORELSE](entries/_orelse.md), [_UNSIGNED](entries/_unsigned.md), [AND](entries/and.md), [CDBL](entries/cdbl.md), [CINT](entries/cint.md), [CLNG](entries/clng.md), [CSNG](entries/csng.md), [DEFINT](entries/defint.md), [DOUBLE](entries/double.md), [EQV](entries/eqv.md), [IMP](entries/imp.md), [INTEGER](entries/integer.md), [LONG](entries/long.md), [MOD](entries/mod.md), [NOT](entries/not.md), [OR](entries/or.md), [SINGLE](entries/single.md), [STRING](entries/string.md), [XOR](entries/xor.md)
- **Variables, scope, constants**: [COMMON](entries/common.md), [CONST](entries/const.md), [DIM](entries/dim.md), [LET](entries/let.md), [$COLOR](entries/meta-color.md), [OPTION](entries/option.md), [SHARED](entries/shared.md), [STATIC](entries/static.md), [SWAP](entries/swap.md)
- **Control flow**: [_CONTINUE](entries/_continue.md), [DO](entries/do.md), [END](entries/end.md), [EXIT](entries/exit.md), [FOR](entries/for.md), [GOSUB](entries/gosub.md), [GOTO](entries/goto.md), [IF](entries/if.md), [ON](entries/on.md), [RUN](entries/run.md), [SELECT CASE](entries/select-case.md), [STOP](entries/stop.md), [SYSTEM](entries/system.md), [WHILE](entries/while.md)
- **Procedures**: [CALL](entries/call.md), [DECLARE](entries/declare.md), [FUNCTION](entries/function.md), [SUB](entries/sub.md)
- **Arrays and TYPE**: [_ARRAYCOPY](entries/_arraycopy.md), [ERASE](entries/erase.md), [LBOUND](entries/lbound.md), [$DYNAMIC](entries/meta-dynamic.md), [$STATIC](entries/meta-static.md), [$UNSTABLE](entries/meta-unstable.md), [REDIM](entries/redim.md), [TYPE](entries/type.md), [UBOUND](entries/ubound.md)
- **Strings**: [_BIN$](entries/_bin-str.md), [_CV](entries/_cv.md), [_MK$](entries/_mk-str.md), [_STRCMP](entries/_strcmp.md), [_TOSTR$](entries/_tostr-str.md), [_TRIM$](entries/_trim-str.md), [ASC](entries/asc.md), [CHR$](entries/chr-str.md), [CVI](entries/cvi.md), [CVSMBF](entries/cvsmbf.md), [HEX$](entries/hex-str.md), [LCASE$](entries/lcase-str.md), [LEFT$](entries/left-str.md), [LEN](entries/len.md), [LTRIM$](entries/ltrim-str.md), [MID$](entries/mid-str.md), [MKI$](entries/mki-str.md), [MKSMBF$](entries/mksmbf-str.md), [OCT$](entries/oct-str.md), [RIGHT$](entries/right-str.md), [RTRIM$](entries/rtrim-str.md), [SPACE$](entries/space-str.md), [STR$](entries/str-str.md), [STRING$](entries/string-str.md), [UCASE$](entries/ucase-str.md), [VAL](entries/val.md)
- **Math and number functions**: [_ARCSEC](entries/_arcsec.md), [_ASIN](entries/_asin.md), [_ASINH](entries/_asinh.md), [_ATAN2](entries/_atan2.md), [_CEIL](entries/_ceil.md), [_CLAMP](entries/_clamp.md), [_D2R](entries/_d2r.md), [_HYPOT](entries/_hypot.md), [_IIF](entries/_iif.md), [_INSTRREV](entries/_instrrev.md), [_MIN](entries/_min.md), [_PI](entries/_pi.md), [_ROL](entries/_rol.md), [_ROUND](entries/_round.md), [_SEC](entries/_sec.md), [_SECH](entries/_sech.md), [_SETBIT](entries/_setbit.md), [_SHL](entries/_shl.md), [_SINH](entries/_sinh.md), [ABS](entries/abs.md), [ATN](entries/atn.md), [COS](entries/cos.md), [EXP](entries/exp.md), [FIX](entries/fix.md), [INSTR](entries/instr.md), [INT](entries/int.md), [LOG](entries/log.md), [RANDOMIZE](entries/randomize.md), [RND](entries/rnd.md), [SGN](entries/sgn.md), [SIN](entries/sin.md), [SQR](entries/sqr.md), [TAN](entries/tan.md)
- **PRINT, PRINT USING, console and text output**: [_CONSOLE](entries/_console.md), [_CONSOLEINPUT](entries/_consoleinput.md), [_CONSOLETITLE](entries/_consoletitle.md), [_CONTROLCHR](entries/_controlchr.md), [_ECHO](entries/_echo.md), [CLS](entries/cls.md), [CSRLIN](entries/csrlin.md), [LOCATE](entries/locate.md), [LPRINT](entries/lprint.md), [$CONSOLE](entries/meta-console.md), [POS](entries/pos.md), [PRINT](entries/print.md), [PRINT USING](entries/print-using.md), [SPC](entries/spc.md), [TAB](entries/tab.md), [WIDTH](entries/width.md)
- **Keyboard and console input**: [_CAPSLOCK](entries/_capslock.md), [_KEYCLEAR](entries/_keyclear.md), [_KEYDOWN](entries/_keydown.md), [_KEYHIT](entries/_keyhit.md), [INKEY$](entries/inkey-str.md), [INPUT](entries/input.md), [INPUT$](entries/input-str.md), [LINE INPUT](entries/line-input.md)
- **Errors**: [_ASSERT](entries/_assert.md), [_ERRORLINE](entries/_errorline.md), [_ERRORMESSAGE$](entries/_errormessage-str.md), [_INCLERRORLINE](entries/_inclerrorline.md), [ERL](entries/erl.md), [ERR](entries/err.md), [ERROR](entries/error.md), [$ASSERTS](entries/meta-asserts.md), [$CHECKING](entries/meta-checking.md), [$DEBUG](entries/meta-debug.md), [ON ERROR](entries/on-error.md), [RESUME](entries/resume.md), [TRON](entries/tron.md)
- **Files**: [_CWD$](entries/_cwd-str.md), [_DIR$](entries/_dir-str.md), [_FILEEXISTS](entries/_fileexists.md), [_FILES$](entries/_files-str.md), [_FULLPATH$](entries/_fullpath-str.md), [_READFILE$](entries/_readfile-str.md), [CHDIR](entries/chdir.md), [CLOSE](entries/close.md), [DATA](entries/data.md), [EOF](entries/eof.md), [FIELD](entries/field.md), [FILEATTR](entries/fileattr.md), [FILES](entries/files.md), [FREEFILE](entries/freefile.md), [GET](entries/get.md), [IOCTL](entries/ioctl.md), [KILL](entries/kill.md), [LOC](entries/loc.md), [LOCK](entries/lock.md), [LOF](entries/lof.md), [LSET](entries/lset.md), [NAME](entries/name.md), [OPEN](entries/open.md), [PUT](entries/put.md), [READ](entries/read.md), [RESET](entries/reset.md), [RESTORE](entries/restore.md), [SEEK](entries/seek.md), [WRITE](entries/write.md)
- **Screen modes and drawing**: [_AUTODISPLAY](entries/_autodisplay.md), [_BLINK](entries/_blink.md), [_COPYPALETTE](entries/_copypalette.md), [_DEFAULTCOLOR](entries/_defaultcolor.md), [_DEST](entries/_dest.md), [_DISPLAY](entries/_display.md), [_FPS](entries/_fps.md), [_HSB32](entries/_hsb32.md), [_HUE32](entries/_hue32.md), [_PALETTECOLOR](entries/_palettecolor.md), [_RED](entries/_red.md), [_RED32](entries/_red32.md), [_RGB](entries/_rgb.md), [_RGB32](entries/_rgb32.md), [_SOURCE](entries/_source.md), [CIRCLE](entries/circle.md), [COLOR](entries/color.md), [DRAW](entries/draw.md), [LINE](entries/line.md), [PAINT](entries/paint.md), [PALETTE](entries/palette.md), [PCOPY](entries/pcopy.md), [PMAP](entries/pmap.md), [POINT](entries/point.md), [PSET](entries/pset.md), [SCREEN](entries/screen.md), [VIEW](entries/view.md), [VIEW PRINT](entries/view-print.md), [WINDOW](entries/window.md)
- **Images and fonts**: [_BLEND](entries/_blend.md), [_CLEARCOLOR](entries/_clearcolor.md), [_COPYIMAGE](entries/_copyimage.md), [_FONT](entries/_font.md), [_FONTWIDTH](entries/_fontwidth.md), [_FREEIMAGE](entries/_freeimage.md), [_LOADFONT](entries/_loadfont.md), [_LOADIMAGE](entries/_loadimage.md), [_MAPTRIANGLE](entries/_maptriangle.md), [_MAPUNICODE](entries/_mapunicode.md), [_NEWIMAGE](entries/_newimage.md), [_PIXELSIZE](entries/_pixelsize.md), [_PRINTIMAGE](entries/_printimage.md), [_PRINTMODE](entries/_printmode.md), [_PRINTSTRING](entries/_printstring.md), [_PUTIMAGE](entries/_putimage.md), [_SAVEIMAGE](entries/_saveimage.md), [_SETALPHA](entries/_setalpha.md), [_UCHARPOS](entries/_ucharpos.md), [_UFONTHEIGHT](entries/_ufontheight.md), [_UPRINTSTRING](entries/_uprintstring.md), [_WIDTH](entries/_width.md)
- **Mouse and devices**: [_AXIS](entries/_axis.md), [_BUTTON](entries/_button.md), [_DEVICE$](entries/_device-str.md), [_DEVICEINPUT](entries/_deviceinput.md), [_DEVICES](entries/_devices.md), [_LASTBUTTON](entries/_lastbutton.md), [_MOUSEBUTTON](entries/_mousebutton.md), [_MOUSECURSOR](entries/_mousecursor.md), [_MOUSEDISABLED](entries/_mousedisabled.md), [_MOUSEHIDDEN](entries/_mousehidden.md), [_MOUSEINPUT](entries/_mouseinput.md), [_MOUSEMOVE](entries/_mousemove.md), [_MOUSEMOVEMENTX](entries/_mousemovementx.md), [_MOUSESHOW](entries/_mouseshow.md), [_MOUSEWHEEL](entries/_mousewheel.md), [_MOUSEX](entries/_mousex.md), [STICK](entries/stick.md), [STRIG](entries/strig.md)
- **Sound**: [_MIDISOUNDBANK](entries/_midisoundbank.md), [_SNDBAL](entries/_sndbal.md), [_SNDCLOSE](entries/_sndclose.md), [_SNDCOPY](entries/_sndcopy.md), [_SNDGETPOS](entries/_sndgetpos.md), [_SNDLEN](entries/_sndlen.md), [_SNDLIMIT](entries/_sndlimit.md), [_SNDLOOP](entries/_sndloop.md), [_SNDNEW](entries/_sndnew.md), [_SNDOPEN](entries/_sndopen.md), [_SNDOPENRAW](entries/_sndopenraw.md), [_SNDPAUSE](entries/_sndpause.md), [_SNDPLAY](entries/_sndplay.md), [_SNDPLAYCOPY](entries/_sndplaycopy.md), [_SNDPLAYFILE](entries/_sndplayfile.md), [_SNDPLAYING](entries/_sndplaying.md), [_SNDRATE](entries/_sndrate.md), [_SNDRAW](entries/_sndraw.md), [_SNDRAWBATCH](entries/_sndrawbatch.md), [_SNDRAWDONE](entries/_sndrawdone.md), [_SNDRAWLEN](entries/_sndrawlen.md), [_SNDSTOP](entries/_sndstop.md), [_SNDVOL](entries/_sndvol.md), [_WAVE](entries/_wave.md), [BEEP](entries/beep.md), [$MIDISOUNDFONT](entries/meta-midisoundfont.md), [PLAY](entries/play.md), [SOUND](entries/sound.md)
- **Events and timers**: [_DELAY](entries/_delay.md), [_FREETIMER](entries/_freetimer.md), [_LIMIT](entries/_limit.md), [COM](entries/com.md), [KEY](entries/key.md), [PEN](entries/pen.md), [SLEEP](entries/sleep.md), [TIMER](entries/timer.md), [UEVENT](entries/uevent.md)
- **Memory**: [_MEM](entries/_mem.md), [_MEMCOPY](entries/_memcopy.md), [_MEMELEMENT](entries/_memelement.md), [_MEMEXISTS](entries/_memexists.md), [_MEMFILL](entries/_memfill.md), [_MEMFREE](entries/_memfree.md), [_MEMGET](entries/_memget.md), [_MEMIMAGE](entries/_memimage.md), [_MEMNEW](entries/_memnew.md), [_MEMPUT](entries/_memput.md), [_MEMSOUND](entries/_memsound.md), [ABSOLUTE](entries/absolute.md), [BLOAD](entries/bload.md), [BSAVE](entries/bsave.md), [DEF SEG](entries/def-seg.md), [INP](entries/inp.md), [OUT](entries/out.md), [PEEK](entries/peek.md), [POKE](entries/poke.md), [SADD](entries/sadd.md), [VARPTR](entries/varptr.md), [VARPTR$](entries/varptr-str.md), [VARSEG](entries/varseg.md), [WAIT](entries/wait.md)
- **Libraries**: [DECLARE LIBRARY](entries/declare-library.md), [$USELIBRARY](entries/meta-uselibrary.md)
- **Window, clipboard, system, date and time**: [_ACCEPTFILEDROP](entries/_acceptfiledrop.md), [_ALLOWFULLSCREEN](entries/_allowfullscreen.md), [_CLIPBOARD$](entries/_clipboard-str.md), [_CLIPBOARDIMAGE](entries/_clipboardimage.md), [_COLORCHOOSERDIALOG](entries/_colorchooserdialog.md), [_COMMANDCOUNT](entries/_commandcount.md), [_COMPILEDATE$](entries/_compiledate-str.md), [_COMPILERVERSION$](entries/_compilerversion-str.md), [_DESKTOPREFRESHRATE](entries/_desktoprefreshrate.md), [_DESKTOPWIDTH](entries/_desktopwidth.md), [_ENVIRONCOUNT](entries/_environcount.md), [_EXIT](entries/_exit.md), [_FULLSCREEN](entries/_fullscreen.md), [_ICON](entries/_icon.md), [_INPUTBOX$](entries/_inputbox-str.md), [_MESSAGEBOX](entries/_messagebox.md), [_NOTIFYPOPUP](entries/_notifypopup.md), [_OPENFILEDIALOG$](entries/_openfiledialog-str.md), [_OS$](entries/_os-str.md), [_RESIZE](entries/_resize.md), [_SCALEDWIDTH](entries/_scaledwidth.md), [_SCREENCLICK](entries/_screenclick.md), [_SCREENEXISTS](entries/_screenexists.md), [_SCREENHIDE](entries/_screenhide.md), [_SCREENICON](entries/_screenicon.md), [_SCREENIMAGE](entries/_screenimage.md), [_SCREENMOVE](entries/_screenmove.md), [_SCREENPRINT](entries/_screenprint.md), [_SCREENX](entries/_screenx.md), [_SHELLHIDE](entries/_shellhide.md), [_TITLE](entries/_title.md), [_UPTIME](entries/_uptime.md), [_WINDOWHANDLE](entries/_windowhandle.md), [_WINDOWHASFOCUS](entries/_windowhasfocus.md), [_WINDOWSIZELIMIT](entries/_windowsizelimit.md), [CHAIN](entries/chain.md), [CLEAR](entries/clear.md), [COMMAND$](entries/command-str.md), [DATE$](entries/date-str.md), [ENVIRON](entries/environ.md), [FRE](entries/fre.md), [$EXEICON](entries/meta-exeicon.md), [$FORMAT](entries/meta-format.md), [$RESIZE](entries/meta-resize.md), [$SCREENHIDE](entries/meta-screenhide.md), [$VERSIONINFO](entries/meta-versioninfo.md), [SETMEM](entries/setmem.md), [SHELL](entries/shell.md), [TIME$](entries/time-str.md)
- **Data processing, embedded files, logging**: [_ADLER32](entries/_adler32.md), [_BASE64DECODE$](entries/_base64decode-str.md), [_BASE64ENCODE$](entries/_base64encode-str.md), [_CRC32](entries/_crc32.md), [_DEFLATE$](entries/_deflate-str.md), [_EMBEDDED$](entries/_embedded-str.md), [_INFLATE$](entries/_inflate-str.md), [_LOGINFO](entries/_loginfo.md), [_LOGMINLEVEL](entries/_logminlevel.md), [_MD5$](entries/_md5-str.md), [$EMBED](entries/meta-embed.md)
- **Network**: [_CONNECTED](entries/_connected.md), [_CONNECTIONADDRESS](entries/_connectionaddress.md), [_OPENCLIENT](entries/_openclient.md), [_OPENCONNECTION](entries/_openconnection.md), [_OPENHOST](entries/_openhost.md), [_STATUSCODE](entries/_statuscode.md), [_WHATISMYIP$](entries/_whatismyip.md)
- **OpenGL**: [_DEPTHBUFFER](entries/_depthbuffer.md), [_DISPLAYORDER](entries/_displayorder.md), [SUB _GL](entries/_gl.md), [_GLRENDER](entries/_glrender.md)

## Appendices

- [Coming from QB64pe](coming-from-qb64pe.md)
- [Coming from QuickBASIC 4.5](coming-from-qb45.md)
- [The auto-included constants](appendix-constants.md)
- [Run-time errors](appendix-errors.md)
- [Reserved words](appendix-reserved.md)
- [Metacommands](appendix-metacommands.md)
- [What is not supported yet](appendix-unsupported.md)
