# Runs one step of `check` in the suites VM (tools\winvm\README.md, "The suites VM"; OpenSpec change vm-suites,
# design D6, D8, D11). tools\check\check.py calls it between suites.ps1 -Start and -Stop:
#
#   tools\winvm\step.ps1 -Tree <path> -Job <file>
#
# The job is the JSON file check.py writes (target\check\vm\<step>.json): "step", "command" (arguments), "first"
# (a command run before it, or null), "cwd" (relative to the tree), "env", "copy" (host files of the tree put in the
# guest's copy first), "outputs" (copied out at the end; tools\winvm\outputs.py), "clears" (removed first), and
# "limit" (seconds).
#
# Before the command, the guest's qb64rust.exe must say that it builds with the installed toolchain: the clone at
# ..\QB64pe beside the tree in the guest serves only qb64pe.exe, and a build with its toolchain would hide a
# dependency on the host (design D11). The command runs in the test account's logged-on session through a
# scheduled task, with Mesa's OpenGL for its programs (design D7). What the guest's log gains is printed every 10 s,
# so check's log of the step shows its progress. The outputs are copied out also after a failure or a timeout.
#
# Exit: the command's exit code; 1 when it did not end within the limit, when the toolchain is not the installed
# one, or when the step could not be run.

param(
    [Parameter(Mandatory = $true)][string]$Tree,
    [Parameter(Mandatory = $true)][string]$Job
)

$ErrorActionPreference = 'Stop'
$name = if ($env:QB64RUST_SUITES_VM) { $env:QB64RUST_SUITES_VM } else { 'qb64rust-suites' }
$credentialFile = Join-Path $env:LOCALAPPDATA "qb64rust\winvm\$name\credential.xml"
$own = 'qb64rust-vm'
$taskName = 'qb64rust-step'
$out = [Console]::OpenStandardOutput()

# A line of the step's log, as bytes, beside the guest's log bytes.
function Say([string]$text) {
    $bytes = [Text.Encoding]::UTF8.GetBytes("step.ps1: $text`n")
    $out.Write($bytes, 0, $bytes.Length)
    $out.Flush()
}

function Seconds([datetime]$since) { ((Get-Date) - $since).TotalSeconds }

# A path of the job, relative to the tree with `/`, as a full Windows path.
function InTree([string]$rel) { Join-Path $Tree ($rel.Replace('/', '\')) }

# A batch file's quoted argument: % doubled, and no quote, which cmd cannot carry inside one, nor anything but ASCII.
function Quote([string]$arg) {
    if ($arg.Contains('"')) { throw "an argument holds a double quote, which the step's batch file cannot carry: $arg" }
    if ($arg -match '[^\x20-\x7E]') { throw "an argument is not printable ASCII, which the step's batch file is: $arg" }
    '"' + $arg.Replace('%', '%%') + '"'
}

$Tree = (Resolve-Path -LiteralPath $Tree).Path.TrimEnd('\')
$parent = Split-Path $Tree
$spec = Get-Content -Raw -LiteralPath $Job | ConvertFrom-Json
$step = $spec.step
$guest = "C:\run\$step"
$session = $null
$code = 1
$offset = 0

# What the guest's log gained since the last call, printed as it is.
function Show-Log {
    $bytes = Invoke-Command -Session $session -ArgumentList "$guest\log.txt", $script:offset {
        param($path, $from)
        if (-not (Test-Path -LiteralPath $path)) { return , ([byte[]]@()) }
        $f = [IO.File]::Open($path, 'Open', 'Read', 'ReadWrite')
        try {
            $size = $f.Length - $from
            if ($size -le 0) { return , ([byte[]]@()) }
            [void]$f.Seek($from, 'Begin')
            $buffer = New-Object byte[] $size
            $n = $f.Read($buffer, 0, $size)
            if ($n -lt $size) { [Array]::Resize([ref]$buffer, $n) }
            return , $buffer
        } finally {
            $f.Close()
        }
    }
    if ($bytes -and $bytes.Length -gt 0) {
        $bytes = [byte[]]$bytes
        $out.Write($bytes, 0, $bytes.Length)
        $out.Flush()
        $script:offset += $bytes.Length
    }
}

# The step from the copy in to its end: its exit code. $session is the script's, for the cleanup after it.
function Invoke-Step {
    Import-Module Hyper-V -ErrorAction Stop
    if ((Get-VM -Name $name).State -ne 'Running') { throw "the VM $name is not running: suites.ps1 -Start comes first" }
    $credential = Import-Clixml -Path $credentialFile
    $script:session = New-PSSession -VMName $name -Credential $credential -ErrorAction Stop
    $session = $script:session

    # The job's own folder in the guest, given to tester, who runs the command.
    Invoke-Command -Session $session -ArgumentList $guest {
        param($folder)
        if (Test-Path -LiteralPath $folder) { Remove-Item -Recurse -Force -LiteralPath $folder }
        New-Item -ItemType Directory -Force $folder | Out-Null
        & icacls $folder /grant 'tester:(OI)(CI)M' /T /Q | Out-Null
    }
    foreach ($rel in @($spec.copy)) {
        if (-not $rel) { continue }
        $target = InTree $rel
        $there = Invoke-Command -Session $session -ArgumentList $target {
            param($file)
            New-Item -ItemType Directory -Force (Split-Path $file) | Out-Null
            if (Test-Path -LiteralPath $file) { Remove-Item -Force -LiteralPath $file }
        }
        Copy-Item -ToSession $session -LiteralPath $target -Destination (Split-Path $target) -Force
        $there = Invoke-Command -Session $session -ArgumentList $target { param($file) Test-Path -LiteralPath $file }
        if (-not $there) { throw "$rel was not copied into the guest" }
        Say "copied in: $rel"
    }
    Invoke-Command -Session $session -ArgumentList $Tree, @($spec.clears) {
        param($tree, $clears)
        foreach ($rel in $clears) {
            if (-not $rel) { continue }
            $p = Join-Path $tree ($rel.Replace('/', '\'))
            if (Test-Path -LiteralPath $p) { Remove-Item -Force -LiteralPath $p }
            New-Item -ItemType Directory -Force (Split-Path $p) | Out-Null
        }
    }

    # The toolchain guard (design D11).
    $compiler = Join-Path $Tree 'target\release\qb64rust.exe'
    if (-not (Test-Path -LiteralPath $compiler)) { $compiler = Join-Path $Tree 'target\debug\qb64rust.exe' }
    $found = Invoke-Command -Session $session -ArgumentList $compiler {
        param($exe)
        if (-not (Test-Path -LiteralPath $exe)) { return @('', "no $exe in the guest") }
        @(& $exe toolchain path 2>&1 | ForEach-Object { "$_" })
    }
    if (@($found).Count -lt 2 -or -not "$($found[1])".StartsWith('installed:')) {
        Say ("the VM's qb64rust would not build with the installed toolchain: '$(@($found) -join ' / ')'. Make the " +
            "VM again (tools\winvm\README.md, 'The suites VM'), so that create.ps1 installs the toolchain qb64rust " +
            "pins now. Nothing was built.")
        return 1
    }

    # The batch file the task runs: its output to the log, its exit code to code.txt when it ends.
    $cwd = if ($spec.cwd) { InTree $spec.cwd } else { $Tree }
    $lines = @(
        # No `chcp 65001`: a program's console keeps the code page it has on the host, as its recordings expect
        # (verification\v40_g7_utf8_in_name printed UTF-8 twice encoded under it, task 6.3). So the file is ASCII.
        '@echo off',
        "cd /d $(Quote $cwd)",
        "set `"GALLIUM_DRIVER=llvmpipe`"",
        "set `"QB64RUST_OPENGL_DIR=$parent\$own\mesa`"",
        'set "PYTHONUNBUFFERED=1"',
        # A test that needs what the VM lacks (a git checkout) says so and skips (crates\driver\tests\it\support).
        'set "QB64RUST_IN_SUITES_VM=1"'
    )
    foreach ($p in $spec.env.PSObject.Properties) { $lines += "set $(Quote "$($p.Name)=$($p.Value)")" }
    $lines += "call :steps > $(Quote "$guest\log.txt") 2>&1"
    $lines += ">$(Quote "$guest\code.tmp") echo %ERRORLEVEL%"
    $lines += "move /y $(Quote "$guest\code.tmp") $(Quote "$guest\code.txt") > nul"
    $lines += 'exit /b 0'
    $lines += ':steps'
    foreach ($command in @($spec.first, $spec.command)) {
        if (-not $command) { continue }
        $words = @($command)
        # A program of the tree (the CLI tests' binary) by its full path: the command may run in a folder below.
        if ($words[0] -match '[\\/]') { $words[0] = InTree $words[0] }
        $lines += (($words | ForEach-Object { Quote $_ }) -join ' ')
        # Not `if errorlevel 1`, which a crash's negative exit code does not meet.
        $lines += 'if %ERRORLEVEL% neq 0 exit /b %ERRORLEVEL%'
    }
    $lines += 'exit /b 0'
    Invoke-Command -Session $session -ArgumentList "$guest\job.cmd", (($lines -join "`r`n") + "`r`n") {
        param($path, $text) [IO.File]::WriteAllText($path, $text, [Text.Encoding]::ASCII)
    }
    Say "running in $name, at most $($spec.limit) s"

    # In tester's logged-on session, on its desktop, as run.ps1 does for the desktop family.
    Invoke-Command -Session $session -ArgumentList $guest, $taskName, ([int]$spec.limit + 600) {
        param($folder, $task, $limit)
        $action = New-ScheduledTaskAction -Execute 'cmd.exe' -Argument "/c `"$folder\job.cmd`"" -WorkingDirectory $folder
        $principal = New-ScheduledTaskPrincipal -UserId 'tester' -LogonType Interactive -RunLevel Limited
        $settings = New-ScheduledTaskSettingsSet -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries `
            -ExecutionTimeLimit (New-TimeSpan -Seconds $limit)
        Register-ScheduledTask -TaskName $task -Action $action -Principal $principal -Settings $settings -Force |
            Out-Null
        Start-ScheduledTask -TaskName $task
    }
    $begun = Get-Date
    $shown = Get-Date
    $done = $false
    while (-not $done -and (Seconds $begun) -lt $spec.limit) {
        Start-Sleep -Seconds 2
        $done = Invoke-Command -Session $session -ArgumentList "$guest\code.txt" { param($p) Test-Path -LiteralPath $p }
        if ((Seconds $shown) -ge 10) {
            Show-Log
            $shown = Get-Date
        }
    }
    Show-Log
    if (-not $done) {
        Say "the step did not end within $($spec.limit) s; it is stopped"
        return 1
    }
    [int](Invoke-Command -Session $session -ArgumentList "$guest\code.txt" {
        param($p) (Get-Content -Raw -LiteralPath $p).Trim()
    })
}

try {
    # The last value only: a cmdlet's stray output would come before it.
    $code = Invoke-Step | Select-Object -Last 1
} catch {
    Say "the step could not be run in ${name}: $($_.Exception.Message)"
    $code = 1
} finally {
    if ($session) {
        try {
            # Whatever the task left running (a timeout), so that the next step starts clean.
            Invoke-Command -Session $session -ArgumentList $taskName, $parent {
                param($task, $parent)
                Stop-ScheduledTask -TaskName $task -ErrorAction SilentlyContinue
                Get-CimInstance Win32_Process | Where-Object {
                    $_.Name -eq 'python.exe' -or ($_.ExecutablePath -and $_.ExecutablePath.StartsWith("$parent\"))
                } | ForEach-Object { Stop-Process -Id $_.ProcessId -Force -ErrorAction SilentlyContinue }
                Unregister-ScheduledTask -TaskName $task -Confirm:$false -ErrorAction SilentlyContinue
            }
            # The outputs, also after a failure or a timeout (design D8).
            $wanted = @($spec.outputs | Where-Object { $_ })
            if ($wanted) {
                $count = Invoke-Command -Session $session -ArgumentList $Tree, $guest, $wanted {
                    param($tree, $folder, $wanted)
                    $python = Join-Path $env:ProgramFiles 'Python312\python.exe'
                    $files = @(& $python (Join-Path $tree 'tools\winvm\outputs.py') $tree @wanted)
                    if (-not $files) { return 0 }
                    [IO.File]::WriteAllText("$folder\outputs.list", (($files -join "`n") + "`n"))
                    & "$env:SystemRoot\System32\tar.exe" -cf "$folder\outputs.tar" -C $tree -T "$folder\outputs.list"
                    if ($LASTEXITCODE -ne 0) { return -1 }
                    $files.Count
                }
                if ($count -gt 0) {
                    $archive = Join-Path $Tree "target\check\vm\$step.outputs.tar"
                    Copy-Item -FromSession $session -Path "$guest\outputs.tar" -Destination $archive -Force
                    & "$env:SystemRoot\System32\tar.exe" -xf $archive -C $Tree
                    if ($LASTEXITCODE -ne 0) { throw "tar could not unpack the outputs (exit $LASTEXITCODE)" }
                    Remove-Item -LiteralPath $archive
                    Say "$count output file(s) copied out"
                } elseif ($count -lt 0) {
                    Say 'the outputs could not be archived in the guest'
                    if ($code -eq 0) { $code = 1 }
                }
            }
        } catch {
            Say "the outputs could not be copied out: $($_.Exception.Message)"
            if ($code -eq 0) { $code = 1 }
        }
        Remove-PSSession $session -ErrorAction SilentlyContinue
    }
}
exit $code
