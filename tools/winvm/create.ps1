# Makes a test VM (tools\winvm\README.md): the desktop VM that the desktop family runs in (OpenSpec change
# windows-vm, design D1-D3), or the suites VM that check's program steps run in (OpenSpec change vm-suites, design
# D1-D4, D11). Two ways:
#
#   tools\winvm\create.ps1 -Iso <path to the ISO> [-Name <vm>] [-Shape Desktop|Suites]   elevated: a new base disk
#   tools\winvm\create.ps1 -FromBase [-Name <vm>] [-Shape Desktop|Suites]                 not elevated
#
# -Iso writes the image of the Windows 11 Enterprise evaluation ISO straight to a new 64 GB disk, with
# tools\winvm\unattend.xml as the answer file, so Windows setup is never shown. It also puts Mesa's software OpenGL
# in the disk's C:\mesa (below). That disk, never booted, is the base: qb64rust-base\qb64rust-base.vhdx in Hyper-V's
# disk folder, marked read-only. Then the VM is made from it as -FromBase does, which needs no administrator rights,
# so a VM can be made again after its 90 days without elevation.
#
# The VM's own disk is a copy of the base for -Shape Desktop, and a differencing disk on it for -Shape Suites, which
# stores only what changes after the first boot. A differencing disk breaks when its parent changes, so -Iso refuses
# to write a new base while any VM has a disk that is its child, and names that VM.
#
# Then both make a Generation 2 VM with no network adapter and start it. They wait until the first logon of the
# test account "tester" has written C:\ready. They activate Windows online through Hyper-V's "Default Switch" and
# remove the network adapter again. They save the checkpoint "ready" with the account logged on and idle, and turn
# the VM off.
#
# -Shape Suites also puts in, before the checkpoint: Microsoft's Visual C++ runtime and Python, both pinned by
# SHA-256; the C++ toolchain, whose zip the host downloads and the host's target\release\qb64rust.exe installs in
# the guest as tester ("toolchain install --from-zip"); and a cache disk, cache.vhdx in the VM's disk folder,
# formatted and holding a copy of Mesa, which is detached again before the checkpoint (tools\winvm\suites.ps1 attaches it at each run).
#
# Do not click in this console window while the script runs: a click starts a text selection, which stops the
# script at its next line of output until a key is pressed.
#
# The VM is named by -Name, or else qb64rust-desktop ($env:QB64RUST_VM) for Desktop and qb64rust-suites
# ($env:QB64RUST_SUITES_VM) for Suites. The account's password is random, made by -Iso. On the host it is kept only
# in %LOCALAPPDATA%\qb64rust\winvm\: the base's in qb64rust-base\credential.xml, and a copy for each VM in
# <name>\credential.xml. Windows encrypts these files for the user who wrote them. The base holds the same
# password, so -FromBase needs the base's file.
#
# It refuses (exit 2) when a VM of that name, or its disk folder, already exists. It never removes a VM.

[CmdletBinding(DefaultParameterSetName = 'Iso')]
param(
    [Parameter(ParameterSetName = 'Iso', Mandatory = $true)][string]$Iso,
    [Parameter(ParameterSetName = 'Base', Mandatory = $true)][switch]$FromBase,
    [string]$Name,
    [ValidateSet('Desktop', 'Suites')][string]$Shape = 'Desktop'
)

$ErrorActionPreference = 'Stop'
$suites = $Shape -eq 'Suites'
$name = if ($Name) { $Name }
    elseif ($suites) { if ($env:QB64RUST_SUITES_VM) { $env:QB64RUST_SUITES_VM } else { 'qb64rust-suites' } }
    elseif ($env:QB64RUST_VM) { $env:QB64RUST_VM } else { 'qb64rust-desktop' }
$hostFolder = Join-Path $env:LOCALAPPDATA 'qb64rust\winvm'
$baseCredentialFile = Join-Path $hostFolder 'qb64rust-base\credential.xml'
$credentialFile = Join-Path $hostFolder "$name\credential.xml"
$readyLimitMinutes = 60

# Mesa's software OpenGL (llvmpipe), from the Windows builds of pal1000/mesa-dist-win, MIT license. The guest's only
# display is Hyper-V's synthetic adapter, whose OpenGL 1.1 cannot make a QB64 program's window (windows-vm's
# design.md, "Context", task 3.1). inside.ps1 puts these two DLLs beside each program; in the suites VM the tools
# link them from the cache disk (vm-suites, design D7).
$mesaVersion = '26.2.5'
$mesaUrl = "https://github.com/pal1000/mesa-dist-win/releases/download/$mesaVersion/mesa3d-$mesaVersion-release-msvc.7z"
$mesaSha256 = '997123ef4ce8896204ab912d5b54f19372c487ff9835bcefafa2fca23b8953c2'
$mesaFiles = 'opengl32.dll', 'libgallium_wgl.dll'

# The suites VM's two installers (vm-suites, tasks 1.1 and 2.3). qb64rust.exe and the CLI test binary are built for
# MSVC and load VCRUNTIME140.dll, which a clean Windows does not have. Python: the last 3.12 release with a Windows
# installer, the minor version the host's tools are run with; the installer, not the embeddable zip, whose ._pth
# file keeps a script's own folder off sys.path. Both are signed (Microsoft Corporation; Python Software Foundation).
$installers = @(
    @{ File = 'VC_redist.x64.exe'; What = "Microsoft's Visual C++ runtime 14.44.35211 (about 25 MB)"
       Url = 'https://download.visualstudio.microsoft.com/download/pr/990c720a-46ce-4dc7-87b9-c2eb615d34b9/CC0FF0EB1DC3F5188AE6300FAEF32BF5BEEBA4BDD6E8E445A9184072096B713B/VC_redist.x64.exe'
       Sha256 = 'cc0ff0eb1dc3f5188ae6300faef32bf5beeba4bdd6e8e445a9184072096b713b' },
    @{ File = 'python-3.12.10-amd64.exe'; What = 'Python 3.12.10 (about 26 MB)'
       Url = 'https://www.python.org/ftp/python/3.12.10/python-3.12.10-amd64.exe'
       Sha256 = '67b5635e80ea51072b87941312d00ec8927c4db9ba18938f7ad2d27b328b95fb' }
)
# The suites VM's C++ toolchain: the release qb64rust pins, read from the pin itself so that the two cannot differ.
# The host downloads it and the guest installs it with "qb64rust toolchain install --from-zip". The guest does not
# download it: on 2026-11-05 every TLS download in the VM through the Default Switch broke partway through
# (SEC_E_DECRYPT_FAILURE), with curl and with Invoke-WebRequest, while the host's own download worked.
$toolchainPin = Get-Content -Raw -LiteralPath (Join-Path $PSScriptRoot '..\..\crates\driver\src\toolchain.rs')
$toolchainUrl = [regex]::Match($toolchainPin, 'pub const URL: &str =\s*"([^"]+)"').Groups[1].Value
$toolchainSha256 = [regex]::Match($toolchainPin, 'pub const SHA256: &str = "([0-9a-f]{64})"').Groups[1].Value
# The folder of the suites VM's own things on its cache disk, beside the trees' copies (vm-suites, design D4, D5).
$cacheOwn = 'qb64rust-vm'

function Fail([string]$message) {
    Write-Host "create.ps1: $message"
    exit 2
}

# A file downloaded once into %LOCALAPPDATA%\qb64rust\winvm\<folder>\ and checked against its hash.
function Get-Pinned([string]$folder, [string]$file, [string]$url, [string]$sha256, [string]$what) {
    $folder = Join-Path $hostFolder $folder
    $path = Join-Path $folder $file
    New-Item -ItemType Directory -Force $folder | Out-Null
    if (-not (Test-Path -LiteralPath $path)) {
        Write-Host "Downloading $what..."
        # Without the progress bar: drawing it slows a large download down many times over.
        $ProgressPreference = 'SilentlyContinue'
        Invoke-WebRequest -Uri $url -OutFile $path -UseBasicParsing
    }
    $hash = (Get-FileHash -LiteralPath $path -Algorithm SHA256).Hash.ToLower()
    if ($hash -ne $sha256) {
        Remove-Item -LiteralPath $path
        throw "the download of $url has SHA-256 $hash, not the pinned $sha256; it was deleted"
    }
    $path
}

# Mesa's two DLLs, unpacked beside the archive.
function Get-Mesa {
    $archive = Get-Pinned "mesa\$mesaVersion" "mesa3d-$mesaVersion-release-msvc.7z" $mesaUrl $mesaSha256 `
        "Mesa $mesaVersion (about 70 MB)"
    $folder = Split-Path $archive
    & "$env:SystemRoot\System32\tar.exe" -xf $archive -C $folder ($mesaFiles | ForEach-Object { "x64/$_" })
    if ($LASTEXITCODE -ne 0) { throw "tar could not unpack $archive (exit $LASTEXITCODE)" }
    Join-Path $folder 'x64'
}

# The VMs with a disk whose chain of parents holds $path, from their current disks and those of every checkpoint:
# one object per VM, with the folder of its disk that is the direct child.
function Get-ChildVM([string]$path) {
    $full = [IO.Path]::GetFullPath($path)
    foreach ($vm in Get-VM) {
        $drives = @(Get-VMHardDiskDrive -VMName $vm.Name)
        foreach ($snapshot in @(Get-VMSnapshot -VMName $vm.Name)) { $drives += @(Get-VMHardDiskDrive -VMSnapshot $snapshot) }
        $childDisk = $null
        foreach ($drive in $drives) {
            $disk = $drive.Path
            while ($disk -and -not $childDisk) {
                $parent = (Get-VHD -Path $disk -ErrorAction SilentlyContinue).ParentPath
                if ($parent -and [IO.Path]::GetFullPath($parent) -eq $full) { $childDisk = $disk }
                $disk = $parent
            }
        }
        if ($childDisk) { [pscustomobject]@{ Name = $vm.Name; Folder = (Split-Path $childDisk) } }
    }
}

# Runs a script block in the guest and fails on a non-zero $LASTEXITCODE it returns as its last value.
function Invoke-Guest($session, [string]$what, [scriptblock]$block, [object[]]$arguments = @()) {
    $code = Invoke-Command -Session $session -ScriptBlock $block -ArgumentList $arguments | Select-Object -Last 1
    if ($code -ne 0) { throw "$what failed in the guest (exit $code)" }
}

$fromIso = $PSCmdlet.ParameterSetName -eq 'Iso'
try {
    Import-Module Hyper-V -ErrorAction Stop
    $vmHost = Get-VMHost -ErrorAction Stop
} catch {
    Fail ("Hyper-V cannot be used: $($_.Exception.Message) Turn it on and join the group 'Hyper-V " +
        "Administrators' (tools\winvm\README.md, 'What the host needs').")
}
if (-not (Get-VMSwitch -Name 'Default Switch' -ErrorAction SilentlyContinue)) {
    Fail "Hyper-V's 'Default Switch' is missing: the VM needs it for a minute, to activate Windows."
}

$diskFolder = Join-Path $vmHost.VirtualHardDiskPath $name
$vhdx = Join-Path $diskFolder "$name.vhdx"
$cacheDisk = Join-Path $diskFolder 'cache.vhdx'
$base = Join-Path $vmHost.VirtualHardDiskPath 'qb64rust-base\qb64rust-base.vhdx'
# The base of the windows-vm change, named after its VM, with the one credential file of that time. Read by
# -FromBase when there is no qb64rust-base.
$oldBase = Join-Path $vmHost.VirtualHardDiskPath "$name-base\$name-base.vhdx"
$oldCredentialFile = Join-Path $hostFolder 'credential.xml'

if (Get-VM -Name $name -ErrorAction SilentlyContinue) {
    Fail ("a VM named $name already exists. Nothing was changed. To make it again, remove it first:`n" +
        "  Remove-VM -Name $name -Force`n" +
        "  Remove-Item -Recurse -Force `"$diskFolder`"")
}
if (Test-Path -LiteralPath $diskFolder) {
    Fail ("the disk folder of $name already exists, without its VM. Nothing was changed. Remove it first:`n" +
        "  Remove-Item -Recurse -Force `"$diskFolder`"")
}
if ($fromIso) {
    # Before the elevation check, so that a refusal never needs an elevated shell to be seen.
    if (Test-Path -LiteralPath $base) {
        $children = @(Get-ChildVM $base)
        if ($children) {
            $remove = ($children | ForEach-Object {
                "  Stop-VM -Name $($_.Name) -TurnOff -Force`n  Remove-VM -Name $($_.Name) -Force`n" +
                "  Remove-Item -Recurse -Force `"$($_.Folder)`""
            }) -join "`n"
            Fail ("the base disk ($base) is the parent of a disk of the VM $(($children.Name) -join ', '), which a new " +
                "base would break. Nothing was changed. Remove that VM first, or make this VM with -FromBase:`n" +
                $remove)
        }
    }
    $principal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        Fail ('run this from an elevated PowerShell: writing the image to a new disk needs administrator rights. ' +
            'To make the VM again from the disk kept by an earlier run, use -FromBase, which does not.')
    }
    if (-not (Test-Path -LiteralPath $Iso -PathType Leaf)) {
        Fail "no file at the ISO path given: $Iso"
    }
} else {
    if (-not (Test-Path -LiteralPath $base) -and (Test-Path -LiteralPath $oldBase)) {
        $base = $oldBase
        $baseCredentialFile = $oldCredentialFile
    }
    if (-not (Test-Path -LiteralPath $base)) {
        Fail "there is no base disk ($base): make the VM with -Iso, from an elevated PowerShell."
    }
    if (-not (Test-Path -LiteralPath $baseCredentialFile)) {
        Fail ("the credential that goes with the base disk ($baseCredentialFile) is missing: make the VM with -Iso, " +
            "from an elevated PowerShell.")
    }
}
$qb64rust = Join-Path $PSScriptRoot '..\..\target\release\qb64rust.exe'
if ($suites -and -not (Test-Path -LiteralPath $qb64rust)) {
    Fail ("the suites VM gets its C++ toolchain from target\release\qb64rust.exe, which is not built: run " +
        "cargo build --release -p qb64rust first.")
}
# The desktop VM's disk takes about 18 GB and its checkpoint's memory up to 4 GB. The suites VM's differencing disk
# and cache disk take about 10 to 17 GB, its checkpoint the memory in use. -Iso writes the base, about 12 GB, too.
# The limits leave room for the disks to grow during setup.
$needGB = $(if ($suites) { 25 } else { 30 }) + $(if ($fromIso) { 15 } else { 0 })
$freeGB = (Get-PSDrive ($diskFolder.Substring(0, 1))).Free / 1GB
if ($freeGB -lt $needGB) {
    Fail ("{0:N0} GB is free on the drive of Hyper-V's disk folder; making the VM needs about {1} GB." -f $freeGB, $needGB)
}

$started = Get-Date
$isoMounted = $false
$vhdMounted = $false
$vmMade = $false
# New-VHD takes only a .vhd or .vhdx name.
$newBase = [IO.Path]::ChangeExtension($base, '.new.vhdx')
try {
    if ($suites) {
        $setup = @($installers | ForEach-Object { Get-Pinned 'installers' $_.File $_.Url $_.Sha256 $_.What })
        if (-not $toolchainUrl -or -not $toolchainSha256) {
            throw 'could not read the toolchain pin (URL and SHA256) from crates\driver\src\toolchain.rs'
        }
        $toolchainZip = Get-Pinned 'installers' ($toolchainUrl -split '/')[-1] $toolchainUrl $toolchainSha256 `
            'the C++ toolchain, llvm-mingw (about 190 MB)'
        $setup += $toolchainZip
    }
    if ($fromIso) {
        $mesa = Get-Mesa

        # Letters and digits only: the password goes into XML and a reg command line unescaped.
        $alphabet = 'ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz23456789'
        $bytes = New-Object byte[] 24
        [Security.Cryptography.RandomNumberGenerator]::Create().GetBytes($bytes)
        $password = -join ($bytes | ForEach-Object { $alphabet[$_ % $alphabet.Length] })
        $credential = New-Object Management.Automation.PSCredential('tester',
            (ConvertTo-SecureString $password -AsPlainText -Force))

        # The image.
        $isoVolume = Mount-DiskImage -ImagePath (Resolve-Path -LiteralPath $Iso).Path -PassThru | Get-Volume
        $isoMounted = $true
        $sources = "$($isoVolume.DriveLetter):\sources"
        $wim = @("$sources\install.wim", "$sources\install.esd") | Where-Object { Test-Path $_ } | Select-Object -First 1
        if (-not $wim) { throw "the ISO has no sources\install.wim or install.esd: is it a Windows ISO?" }
        $images = @(Get-WindowsImage -ImagePath $wim)
        $image = $images | Where-Object { $_.ImageName -match 'Enterprise' } | Select-Object -First 1
        if (-not $image) { $image = $images[0] }
        $info = Get-WindowsImage -ImagePath $wim -Index $image.ImageIndex
        $language = $info.Languages[$info.DefaultLanguageIndex]
        Write-Host "Image: $($image.ImageName) (index $($image.ImageIndex), $language)"

        # The new base, written beside the old one and put in its place only when whole: GPT with EFI, MSR and
        # Windows partitions.
        New-Item -ItemType Directory -Force (Split-Path $base) | Out-Null
        if (Test-Path -LiteralPath $newBase) { Remove-Item -LiteralPath $newBase -Force }
        New-VHD -Path $newBase -SizeBytes 64GB -Dynamic | Out-Null
        $disk = Mount-VHD -Path $newBase -Passthru | Get-Disk
        $vhdMounted = $true
        Initialize-Disk -Number $disk.Number -PartitionStyle GPT
        # The EFI partition is made as a data partition so that it can take a letter and be formatted, and gets
        # the EFI type at the end.
        $efi = New-Partition -DiskNumber $disk.Number -Size 260MB -AssignDriveLetter
        Format-Volume -Partition $efi -FileSystem FAT32 -NewFileSystemLabel System -Confirm:$false | Out-Null
        New-Partition -DiskNumber $disk.Number -Size 16MB -GptType '{e3c9e316-0b5c-4db8-817d-f92df00215ae}' | Out-Null
        $windows = New-Partition -DiskNumber $disk.Number -UseMaximumSize -AssignDriveLetter
        Format-Volume -Partition $windows -FileSystem NTFS -NewFileSystemLabel Windows -Confirm:$false | Out-Null
        $efiRoot = "$((Get-Partition -DiskNumber $disk.Number -PartitionNumber $efi.PartitionNumber).DriveLetter):\"
        $windowsRoot = "$((Get-Partition -DiskNumber $disk.Number -PartitionNumber $windows.PartitionNumber).DriveLetter):\"

        Write-Host 'Writing the image to the base disk (several minutes)...'
        Expand-WindowsImage -ImagePath $wim -Index $image.ImageIndex -ApplyPath $windowsRoot | Out-Null
        & bcdboot (Join-Path $windowsRoot 'Windows') /s $efiRoot.TrimEnd('\') /f UEFI | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "bcdboot failed with exit code $LASTEXITCODE" }

        $unattend = Get-Content -Raw (Join-Path $PSScriptRoot 'unattend.xml')
        $unattend = $unattend.Replace('@@PASSWORD@@', $password).Replace('@@LANGUAGE@@', $language)
        $panther = Join-Path $windowsRoot 'Windows\Panther'
        New-Item -ItemType Directory -Force $panther | Out-Null
        [IO.File]::WriteAllText((Join-Path $panther 'unattend.xml'), $unattend, (New-Object Text.UTF8Encoding($false)))

        $guestMesa = Join-Path $windowsRoot 'mesa'
        New-Item -ItemType Directory -Force $guestMesa | Out-Null
        foreach ($f in $mesaFiles) { Copy-Item -LiteralPath (Join-Path $mesa $f) -Destination $guestMesa }

        Set-Partition -DiskNumber $disk.Number -PartitionNumber $efi.PartitionNumber `
            -GptType '{c12a7328-f81f-11d2-ba4b-00a0c93ec93b}'
        Remove-PartitionAccessPath -DiskNumber $disk.Number -PartitionNumber $efi.PartitionNumber -AccessPath $efiRoot
        Dismount-VHD -Path $newBase
        $vhdMounted = $false
        Dismount-DiskImage -ImagePath (Resolve-Path -LiteralPath $Iso).Path | Out-Null
        $isoMounted = $false

        # The base, never booted, with the password of the credential written beside it. Read-only, so that
        # nothing changes it under a differencing disk.
        if (Test-Path -LiteralPath $base) { Set-ItemProperty -LiteralPath $base -Name IsReadOnly -Value $false }
        Move-Item -LiteralPath $newBase -Destination $base -Force
        Set-ItemProperty -LiteralPath $base -Name IsReadOnly -Value $true
        New-Item -ItemType Directory -Force (Split-Path $baseCredentialFile) | Out-Null
        $credential | Export-Clixml -Path $baseCredentialFile
        Write-Host "Kept the base disk: $base"
    } else {
        $credential = Import-Clixml -Path $baseCredentialFile
    }

    # The VM's own disk.
    New-Item -ItemType Directory -Force $diskFolder | Out-Null
    if ($suites) {
        Set-ItemProperty -LiteralPath $base -Name IsReadOnly -Value $true
        New-VHD -Path $vhdx -ParentPath $base -Differencing | Out-Null
    } else {
        Write-Host 'Copying the base disk...'
        Copy-Item -LiteralPath $base -Destination $vhdx
        Set-ItemProperty -LiteralPath $vhdx -Name IsReadOnly -Value $false
    }
    New-Item -ItemType Directory -Force (Split-Path $credentialFile) | Out-Null
    $credential | Export-Clixml -Path $credentialFile

    # The VM (windows-vm design D1, D3, D5; vm-suites design D1).
    New-VM -Name $name -Generation 2 -MemoryStartupBytes 4GB -VHDPath $vhdx | Out-Null
    $vmMade = $true
    Get-VMNetworkAdapter -VMName $name | Remove-VMNetworkAdapter
    if ($suites) {
        # 8 of the host's 16 processors. Dynamic memory, so that the checkpoint saves only the memory in use.
        Set-VM -Name $name -ProcessorCount 8 -DynamicMemory -MemoryMinimumBytes 2GB -MemoryStartupBytes 4GB `
            -MemoryMaximumBytes 24GB
    } else {
        # 4 processors, not 2: with Mesa drawing on the CPU, 2 let mousemove_readback read 0, 0 in about half the
        # runs (windows-vm's design.md, "Context", task 3.2).
        Set-VM -Name $name -ProcessorCount 4 -StaticMemory
    }
    Set-VM -Name $name -CheckpointType Standard -AutomaticCheckpointsEnabled $false -AutomaticStartAction Nothing `
        -AutomaticStopAction TurnOff
    Set-VMFirmware -VMName $name -EnableSecureBoot On -SecureBootTemplate MicrosoftWindows
    Set-VMKeyProtector -VMName $name -NewLocalKeyProtector
    Enable-VMTPM -VMName $name
    Set-VMVideo -VMName $name -HorizontalResolution 1920 -VerticalResolution 1080 -ResolutionType Single

    Start-VM -Name $name
    Write-Host "Started $name. Waiting for its first logon to finish (about 5 minutes, at most $readyLimitMinutes)..."
    $deadline = (Get-Date).AddMinutes($readyLimitMinutes)
    $ready = $false
    while (-not $ready -and (Get-Date) -lt $deadline) {
        Start-Sleep -Seconds 15
        try {
            $session = New-PSSession -VMName $name -Credential $credential -ErrorAction Stop
            try { $ready = Invoke-Command -Session $session { Test-Path 'C:\ready' } }
            finally { Remove-PSSession $session }
        } catch {
            # The guest is still being set up: PowerShell Direct has no account to log on with yet.
        }
    }
    if (-not $ready) {
        throw ("C:\ready did not appear within $readyLimitMinutes minutes. The VM is left running: open its " +
            "console in Hyper-V Manager (basic session) to see where setup stopped.")
    }

    Write-Host 'The first logon has finished.'
    $session = New-PSSession -VMName $name -Credential $credential
    try {
        Invoke-Command -Session $session { Stop-Service wuauserv -Force; Set-Service wuauserv -StartupType Disabled }

        if ($suites) {
            # The runtime first: qb64rust.exe needs it to start.
            Write-Host 'Installing the Visual C++ runtime and Python in the guest...'
            Invoke-Command -Session $session { New-Item -ItemType Directory -Force C:\setup | Out-Null }
            foreach ($file in @($setup) + (Resolve-Path -LiteralPath $qb64rust).Path) {
                Copy-Item -ToSession $session -LiteralPath $file -Destination 'C:\setup'
            }
            Invoke-Guest $session 'the Visual C++ runtime installer' {
                $p = Start-Process "$env:SystemDrive\setup\VC_redist.x64.exe" '/install /quiet /norestart' -Wait -PassThru
                # 3010: installed, a restart wanted; the runtime's DLLs are in place already.
                if ($p.ExitCode -eq 3010) { 0 } else { $p.ExitCode }
            }
            Invoke-Guest $session 'the Python installer' {
                $p = Start-Process "$env:SystemDrive\setup\python-3.12.10-amd64.exe" -Wait -PassThru -ArgumentList (
                    '/quiet InstallAllUsers=1 PrependPath=1 Include_test=0 Include_launcher=0 Include_doc=0 ' +
                    'Include_tcltk=0 Shortcuts=0')
                $p.ExitCode
            }
            # The toolchain as a user installs it (vm-suites design D11), but from the zip the host downloaded: the
            # SHA-256 check and the unpack, as tester, into tester's %LOCALAPPDATA%\qb64rust\toolchain.
            Write-Host 'Installing the C++ toolchain in the guest (qb64rust toolchain install --from-zip)...'
            Invoke-Guest $session 'qb64rust toolchain install --from-zip' -arguments (Split-Path -Leaf $toolchainZip) {
                param($zip)
                & "$env:SystemDrive\setup\qb64rust.exe" toolchain install --from-zip "$env:SystemDrive\setup\$zip" | Out-Host
                $LASTEXITCODE
            }
        }

        # The evaluation is activated online, which starts its 90 days; unactivated, it reports no grace time left
        # from its first boot. The VM is online only for this, through Hyper-V's NAT "Default Switch", with Windows
        # Update stopped, and the adapter is removed before the checkpoint.
        Write-Host 'Activating Windows (the VM is online for a minute or two)...'
        Add-VMNetworkAdapter -VMName $name -SwitchName 'Default Switch'
        try {
            $activated = Invoke-Command -Session $session {
                $deadline = (Get-Date).AddMinutes(4)
                do {
                    Start-Sleep -Seconds 10
                    cscript //nologo "$env:SystemRoot\System32\slmgr.vbs" /ato | Out-Null
                    $license = Get-CimInstance SoftwareLicensingProduct -Filter ("PartialProductKey IS NOT NULL AND " +
                        "ApplicationID='55c92734-d682-4d71-983e-d6ec3f16059f'")
                } until ($license.LicenseStatus -eq 1 -or (Get-Date) -gt $deadline)
                $license.LicenseStatus -eq 1
            }
        } finally {
            Get-VMNetworkAdapter -VMName $name | Remove-VMNetworkAdapter
        }
        if (-not $activated) {
            throw "Windows was not activated within 4 minutes: is the host online, and does Hyper-V's 'Default Switch' exist?"
        }

        if ($suites) {
            $found = Invoke-Command -Session $session {
                $python = & "$env:ProgramFiles\Python312\python.exe" --version 2>&1
                $toolchain = @(& "$env:SystemDrive\setup\qb64rust.exe" toolchain path 2>&1)
                Remove-Item -Recurse -Force C:\setup
                "$python", "$($toolchain[1])"
            }
            if ($found[0] -ne 'Python 3.12.10') { throw "the guest's Python reports '$($found[0])', not Python 3.12.10" }
            if ($found[1] -notmatch '^installed:') {
                throw "qb64rust toolchain path in the guest says '$($found[1])', not that the toolchain is installed"
            }
            Write-Host "The guest has $($found[0]) and the installed toolchain."

            # The cache disk (vm-suites design D4): made, formatted in the guest with Mesa copied to it, and
            # detached before the checkpoint. A hot-added disk is online at once on this Windows (task 1.2).
            Write-Host 'Making the cache disk...'
            New-VHD -Path $cacheDisk -SizeBytes 64GB -Dynamic | Out-Null
            Add-VMHardDiskDrive -VMName $name -ControllerType SCSI -Path $cacheDisk
            Invoke-Guest $session 'formatting the cache disk' -arguments $cacheOwn {
                param($own)
                $deadline = (Get-Date).AddSeconds(60)
                $disk = $null
                while (-not $disk -and (Get-Date) -lt $deadline) {
                    Update-HostStorageCache
                    $disk = Get-Disk | Where-Object { $_.PartitionStyle -eq 'RAW' } | Select-Object -First 1
                    if (-not $disk) { Start-Sleep -Milliseconds 500 }
                }
                if (-not $disk) { return 1 }
                Initialize-Disk -Number $disk.Number -PartitionStyle GPT
                $partition = New-Partition -DiskNumber $disk.Number -UseMaximumSize
                Format-Volume -Partition $partition -FileSystem NTFS -NewFileSystemLabel qb64rust-cache -Confirm:$false |
                    Out-Null
                $mount = 'C:\cache-setup'
                New-Item -ItemType Directory -Force $mount | Out-Null
                Add-PartitionAccessPath -DiskNumber $disk.Number -PartitionNumber $partition.PartitionNumber -AccessPath "$mount\"
                New-Item -ItemType Directory -Force "$mount\$own\mesa" | Out-Null
                Copy-Item "$env:SystemDrive\mesa\*" "$mount\$own\mesa\"
                Remove-PartitionAccessPath -DiskNumber $disk.Number -PartitionNumber $partition.PartitionNumber -AccessPath "$mount\"
                Remove-Item $mount
                Set-Disk -Number $disk.Number -IsOffline $true
                0
            }
            Get-VMHardDiskDrive -VMName $name | Where-Object { $_.Path -eq $cacheDisk } | Remove-VMHardDiskDrive
        }
    } finally {
        Remove-PSSession $session
    }

    Write-Host 'Letting the guest settle for a minute...'
    Start-Sleep -Seconds 60
    Checkpoint-VM -Name $name -SnapshotName ready
    Stop-VM -Name $name -TurnOff -Force
    Write-Host ("Made $name with its checkpoint 'ready' in {0:N0} minutes." -f ((Get-Date) - $started).TotalMinutes)
} catch {
    Write-Host "create.ps1: $($_.Exception.Message)"
    if ($vhdMounted) { Dismount-VHD -Path $newBase -ErrorAction SilentlyContinue }
    if ($isoMounted) { Dismount-DiskImage -ImagePath (Resolve-Path -LiteralPath $Iso).Path -ErrorAction SilentlyContinue | Out-Null }
    if (Test-Path -LiteralPath $newBase) { Remove-Item -LiteralPath $newBase -Force -ErrorAction SilentlyContinue }
    if ($vmMade) {
        Write-Host ("The VM was made but is not ready. To start again, remove it:`n" +
            "  Stop-VM -Name $name -TurnOff -Force`n" +
            "  Remove-VM -Name $name -Force`n" +
            "  Remove-Item -Recurse -Force `"$diskFolder`"")
    } elseif (Test-Path -LiteralPath $diskFolder) {
        # Only the disk this run made is there; no VM uses it.
        Remove-Item -Recurse -Force -LiteralPath $diskFolder
    }
    exit 1
}
