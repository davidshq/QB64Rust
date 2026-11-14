# Sound

QuickBASIC's `SOUND`, `PLAY` and `BEEP` play tones and music in up to four voices. QB64pe's sound handles open, play
and build sounds from files, memory and raw samples.

The pages of this area: [SOUND](../entries/sound.md), [PLAY](../entries/play.md), [BEEP](../entries/beep.md),
[_WAVE](../entries/_wave.md), [_SNDOPEN](../entries/_sndopen.md), [_SNDPLAY](../entries/_sndplay.md),
[_SNDLOOP](../entries/_sndloop.md), [_SNDPAUSE](../entries/_sndpause.md), [_SNDSTOP](../entries/_sndstop.md),
[_SNDPLAYING](../entries/_sndplaying.md), [_SNDGETPOS](../entries/_sndgetpos.md), [_SNDLEN](../entries/_sndlen.md),
[_SNDVOL](../entries/_sndvol.md), [_SNDBAL](../entries/_sndbal.md), [_SNDLIMIT](../entries/_sndlimit.md),
[_SNDCOPY](../entries/_sndcopy.md), [_SNDPLAYCOPY](../entries/_sndplaycopy.md),
[_SNDPLAYFILE](../entries/_sndplayfile.md), [_SNDCLOSE](../entries/_sndclose.md), [_SNDNEW](../entries/_sndnew.md),
[_MEMSOUND](../entries/_memsound.md), [_SNDOPENRAW](../entries/_sndopenraw.md), [_SNDRAW](../entries/_sndraw.md),
[_SNDRAWBATCH](../entries/_sndrawbatch.md), [_SNDRAWLEN](../entries/_sndrawlen.md),
[_SNDRAWDONE](../entries/_sndrawdone.md), [_SNDRATE](../entries/_sndrate.md),
[_MIDISOUNDBANK](../entries/_midisoundbank.md), [$MIDISOUNDFONT](../entries/meta-midisoundfont.md).

## Foreground and background

In the foreground, the default, `SOUND` and `PLAY` hold the program while their sound plays. `PLAY "MB"` puts a voice
in the background, where each statement only adds its length to what `PLAY(voice)` reports, and `PLAY "MF"` brings it
back.
<!-- src: language/sound "Sound in the foreground holds the program"; language/sound "What a statement queues"; tests\corpus\slice\s75_sound_wait -->

`SOUND WAIT` holds back what later statements queue until `SOUND RESUME`; a `PLAY` with more than one string never
holds the program and ends a `SOUND WAIT`.
<!-- src: language/sound "Sound in the foreground holds the program"; tests\corpus\slice\s75_sound_wait -->

An argument outside its range, or a `PLAY` command that does not exist, raises error 5. The volume, pan and waveform
are checked only once libqb has the audio device.
<!-- src: language/sound "Run-time errors of sound"; language/sound "Run-time errors of a PLAY string"; tests\corpus\slice\s76_sound_errors -->

## Sound handles

`_SNDOPEN` gives a handle above 0 for a sound file, or for a file's bytes with `"memory"`, and 0 for anything it
cannot decode. `_SNDNEW` makes a silent sound whose samples `_MEMSOUND` gives, and `_SNDOPENRAW` a stream that
`_SNDRAW` and `_SNDRAWBATCH` fill.
<!-- src: language/sound "Opening a sound"; language/sound "New sounds and their memory"; language/sound "Raw sound" -->

`_SNDPLAY`, `_SNDLOOP`, `_SNDPAUSE` and `_SNDSTOP` change what `_SNDPLAYING`, `_SNDPAUSED` and `_SNDGETPOS` give, at
any volume, so a program can play at volume 0 and still follow the state.
<!-- src: language/sound "Playing and its state"; tests\corpus\slice\s127_sound_handles; tests\reference\examples\_sndplaying_1 -->

A handle that is not open is never an error: a statement does nothing and a function gives 0.
<!-- src: language/sound "A handle that is not open"; tests\reference\examples\_sndlen_1 -->

## Testing sound

The tests play only silence or sound at volume 0, and print only what is exact on any audio device: lengths and
positions of whole fractions of a second, states, and comparisons.
<!-- src: tests\corpus\slice\s127_sound_handles; tests\corpus\slice\s128_sound_new_memory -->

A program that plays sound and ends soon after could crash in QB64pe, since its audio thread changed a buffer the
program was writing; qb64rust takes a lock there.
<!-- src: language/sound "Queued samples and the audio thread"; D-062 -->
