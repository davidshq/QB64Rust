# Runs a command of a game port's repo in the suites VM (tools\winvm\README.md, "A game port's run"; OpenSpec change
# vm-game-runs). For a run too large for the host, such as Civil War Strategy's tests\web\play_qbjs.js over all its
# scripts:
#
#   tools\winvm\game.ps1 -Game <path> -Command <word> ... [-With <folder> ...] [-Include <path> ...]
#                        [-Outputs <path> ...] [-Cwd <path>] [-Limit <seconds>]
#
#   -Game      the game's repo, beside this repo (the cache disk is mounted at their parent folder)
#   -Command   the command, one word per item, run in the game's folder (or -Cwd, relative to it)
#   -With      other folders beside the game that the command reads, copied in too: their files git tracks or would
#              track (for example qbjs)
#   -Include   folders or files of the game that git ignores but the command needs, copied in whole (for
#              example tests/web/node_modules)
#   -Outputs   files or folders of the game copied back to the host's game folder at the end, also after a failure
#   -Limit     the longest the command may take (default 3600)
#
# It starts the VM as check does (suites.ps1 -Start, with this repo as the tree, so that QB64Rust's tools and the
# QB64pe clone are there at the host's paths). Then it puts Node.js in (pins.psd1): the installed one when the VM has
# it (a VM made by create.ps1 -Shape Suites since 2026-11-17), else the pinned zip unpacked onto the cache disk once.
# It copies the game and each -With folder in, replacing the last copy. The command runs through step.ps1, with
# Node on PATH, and with this repo's target\release\qb64rust.exe copied in when the host has it, so that the
# command can build with QB64Rust. At the end suites.ps1 -Stop turns the VM off.
#
# The VM has Microsoft Edge, which comes with Windows 11, and no Chrome.
#
# Exit: the command's exit code; 2 when the VM refused (its line says why), 1 when the run could not be done.

param(
    [Parameter(Mandatory = $true)][string]$Game,
    [Parameter(Mandatory = $true)][string[]]$Command,
    [string[]]$With = @(),
    [string[]]$Include = @(),
    [string[]]$Outputs = @(),
    [string]$Cwd = '',
    [int]$Limit = 3600
)

$ErrorActionPreference = 'Stop'
$name = if ($env:QB64RUST_SUITES_VM) { $env:QB64RUST_SUITES_VM } else { 'qb64rust-suites' }
$credentialFile = Join-Path $env:LOCALAPPDATA "qb64rust\winvm\$name\credential.xml"
$hostFolder = Join-Path $env:LOCALAPPDATA 'qb64rust\winvm'
$own = 'qb64rust-vm'
$repo = (Resolve-Path -LiteralPath (Join-Path $PSScriptRoot '..\..')).Path.TrimEnd('\')
$parent = Split-Path $repo
$node = (Import-PowerShellDataFile (Join-Path $PSScriptRoot 'pins.psd1')).Node

function Say([string]$text) { Write-Host "game.ps1: $text" }
function Seconds([datetime]$since) { ((Get-Date) - $since).TotalSeconds }

$Game = (Resolve-Path -LiteralPath $Game).Path.TrimEnd('\')
if ((Split-Path $Game) -ne $parent) {
    Say "refused: the game ($Game) is not beside this repo ($repo); the VM mounts their parent folder"
    exit 2
}
$folders = @($Game)
foreach ($w in $With) {
    $folders += (Resolve-Path -LiteralPath (Join-Path $parent $w)).Path.TrimEnd('\')
}
# Each folder's copy in the guest is removed and made again, so none may be one the suites VM keeps itself.
foreach ($p in $folders) {
    if ((Split-Path $p) -ne $parent -or (Split-Path $p -Leaf) -in @((Split-Path $repo -Leaf), 'QB64pe', 'community', $own)) {
        Say "refused: $p must be a folder beside this repo, other than this repo, QB64pe, community and $own"
        exit 2
    }
}
# Scratch files on the host, in this repo's target\, which git ignores.
$scratch = Join-Path $repo "target\check\vm\game-$(Split-Path $Game -Leaf)"
New-Item -ItemType Directory -Force $scratch | Out-Null

# A file downloaded once into %LOCALAPPDATA%\qb64rust\winvm\installers\ and checked against its hash, as create.ps1
# does.
function Get-Pinned($pin) {
    $folder = Join-Path $hostFolder 'installers'
    $path = Join-Path $folder $pin.File
    New-Item -ItemType Directory -Force $folder | Out-Null
    if (-not (Test-Path -LiteralPath $path)) {
        Say "downloading $($pin.Url)"
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $pin.Url -OutFile $path -UseBasicParsing
    }
    $hash = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLower()
    if ($hash -ne $pin.Sha256) {
        Remove-Item -LiteralPath $path
        throw "the download of $($pin.Url) has SHA-256 $hash, not the pinned $($pin.Sha256); it was deleted"
    }
    $path
}

# The folder of node.exe in the guest: the installed one, or the pinned zip unpacked on the cache disk.
function Get-GuestNode($session) {
    $installed = Invoke-Command -Session $session {
        $p = Join-Path $env:ProgramFiles 'nodejs'
        if (Test-Path -LiteralPath (Join-Path $p 'node.exe')) { $p } else { '' }
    }
    if ($installed) { return $installed }
    $folder = "$parent\$own\$([IO.Path]::GetFileNameWithoutExtension($node.File))"
    $there = Invoke-Command -Session $session -ArgumentList "$folder\node.exe" { param($p) Test-Path -LiteralPath $p }
    if (-not $there) {
        $t = Get-Date
        $zip = Get-Pinned $node
        Copy-Item -ToSession $session -LiteralPath $zip -Destination "$parent\$own\$($node.File)"
        $code = Invoke-Command -Session $session -ArgumentList $parent, $own, $node.File {
            param($parent, $own, $file)
            & "$env:SystemRoot\System32\tar.exe" -xf "$parent\$own\$file" -C "$parent\$own"
            $code = $LASTEXITCODE
            Remove-Item -LiteralPath "$parent\$own\$file"
            $code
        }
        if ($code -ne 0) { throw "tar could not unpack $($node.File) in the guest (exit $code)" }
        Say ("Node.js $($node.Version) put on the cache disk ({0:N0} s)" -f (Seconds $t))
    }
    $folder
}

# A folder's files that git tracks or would track, and those under -Include for the game, copied in place of the
# guest's last copy.
function Copy-Folder($session, [string]$path, [string[]]$extra) {
    $t = Get-Date
    $leaf = Split-Path $path -Leaf
    $files = @(& git -C $path ls-files -co --exclude-standard) | Where-Object { [IO.File]::Exists((Join-Path $path $_)) }
    foreach ($rel in $extra) {
        $dir = Join-Path $path ($rel.Replace('/', '\'))
        if (-not (Test-Path -LiteralPath $dir)) { throw "-Include ${rel}: there is no $dir" }
        if (Test-Path -LiteralPath $dir -PathType Leaf) {
            $files += $rel.Replace('\', '/')
            continue
        }
        $prefix = $path.Length + 1
        $files += @([IO.Directory]::EnumerateFiles($dir, '*', 'AllDirectories') | ForEach-Object {
            $_.Substring($prefix).Replace('\', '/')
        })
    }
    $list = Join-Path $scratch "$leaf.list"
    [IO.File]::WriteAllText($list, (($files -join "`n") + "`n"), (New-Object Text.UTF8Encoding($false)))
    $archive = Join-Path $scratch "$leaf.tar"
    & "$env:SystemRoot\System32\tar.exe" -cf $archive -C $path -T $list
    if ($LASTEXITCODE -ne 0) { throw "tar could not archive $path (exit $LASTEXITCODE)" }
    Copy-Item -ToSession $session -LiteralPath $archive -Destination "$parent\$own\$leaf.tar"
    Remove-Item -LiteralPath $archive
    $code = Invoke-Command -Session $session -ArgumentList $parent, $own, $leaf {
        param($parent, $own, $leaf)
        $copy = "$parent\$leaf"
        if (Test-Path -LiteralPath $copy) { Remove-Item -Recurse -Force -LiteralPath $copy }
        New-Item -ItemType Directory -Force $copy | Out-Null
        & "$env:SystemRoot\System32\tar.exe" -xf "$parent\$own\$leaf.tar" -C $copy
        $code = $LASTEXITCODE
        Remove-Item -LiteralPath "$parent\$own\$leaf.tar"
        $code
    }
    if ($code -ne 0) { throw "tar could not unpack $leaf in the guest (exit $code)" }
    Say ("$leaf copied in: {0} files ({1:N0} s)" -f $files.Count, (Seconds $t))
}

& (Join-Path $PSScriptRoot 'suites.ps1') -Start -Tree $repo
$started = $LASTEXITCODE
if ($started -eq 2) { exit 2 }
$code = 1
$session = $null
try {
    if ($started -ne 0) { throw 'suites.ps1 -Start failed' }
    $session = New-PSSession -VMName $name -Credential (Import-Clixml -Path $credentialFile)
    $nodeFolder = Get-GuestNode $session
    foreach ($f in $folders) { Copy-Folder $session $f ($(if ($f -eq $Game) { $Include } else { @() })) }
    Remove-PSSession $session
    $session = $null

    # The job step.ps1 runs. Paths in it are relative to the game's folder.
    $compiler = Join-Path $repo 'target\release\qb64rust.exe'
    $relCompiler = '../' + (Split-Path $repo -Leaf) + '/target/release/qb64rust.exe'
    $job = [ordered]@{
        step     = 'game-' + (Split-Path $Game -Leaf)
        command  = $Command
        first    = $null
        cwd      = $Cwd
        env      = @{}
        copy     = @($(if (Test-Path -LiteralPath $compiler) { $relCompiler }))
        compiler = $(if (Test-Path -LiteralPath $compiler) { $relCompiler } else { '' })
        path     = @($nodeFolder)
        outputs  = $Outputs
        clears   = @()
        limit    = $Limit
        scratch  = $scratch
    }
    $jobFile = Join-Path $scratch 'job.json'
    [IO.File]::WriteAllText($jobFile, ($job | ConvertTo-Json -Depth 4), (New-Object Text.UTF8Encoding($false)))
    & (Join-Path $PSScriptRoot 'step.ps1') -Tree $Game -Job $jobFile
    $code = $LASTEXITCODE
} catch {
    Say "the run could not be done: $($_.Exception.Message)"
    $code = 1
} finally {
    if ($session) { Remove-PSSession $session -ErrorAction SilentlyContinue }
    & (Join-Path $PSScriptRoot 'suites.ps1') -Stop
}
exit $code
