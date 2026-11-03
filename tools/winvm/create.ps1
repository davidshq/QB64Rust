# Makes the test VM that the desktop family runs in (tools\winvm\README.md; OpenSpec change windows-vm, design
# D1-D3). Two ways:
#
#   tools\winvm\create.ps1 -Iso <path to the ISO>    from an elevated PowerShell: a new disk from the ISO
#   tools\winvm\create.ps1 -FromBase                 from a normal PowerShell: the disk kept by the last -Iso
#
# -Iso writes the image of the Windows 11 Enterprise evaluation ISO straight to a new 64 GB disk in Hyper-V's
# default disk folder, with tools\winvm\unattend.xml as the answer file, so Windows setup is never shown. It also
# puts Mesa's software OpenGL in the disk's C:\mesa (below). Before the disk ever boots, it keeps a copy of it, the
# base, in the folder <name>-base beside the VM's own. -FromBase copies the base in place of writing the image,
# which needs no administrator rights, so the VM can be made again after its 90 days without elevation.
#
# Then both make a Generation 2 VM with no network adapter and start it. They wait until the first logon of the
# test account "tester" has written C:\ready. They activate Windows online through Hyper-V's "Default Switch" and
# remove the network adapter again. They save the checkpoint "ready" with the account logged on and idle, and turn
# the VM off.
#
# Do not click in this console window while the script runs: a click starts a text selection, which stops the
# script at its next line of output until a key is pressed.
#
# The VM is named qb64rust-desktop, or $env:QB64RUST_VM. The account's password is random, made by -Iso. On the
# host it is kept only in %LOCALAPPDATA%\qb64rust\winvm\credential.xml, which Windows encrypts for the user who
# wrote it. The base holds the same password, so -FromBase needs that file.
#
# It refuses when a VM of that name, or its disk, already exists. It never removes a VM.

[CmdletBinding(DefaultParameterSetName = 'Iso')]
param(
    [Parameter(ParameterSetName = 'Iso', Mandatory = $true)][string]$Iso,
    [Parameter(ParameterSetName = 'Base', Mandatory = $true)][switch]$FromBase
)

$ErrorActionPreference = 'Stop'
$name = if ($env:QB64RUST_VM) { $env:QB64RUST_VM } else { 'qb64rust-desktop' }
$hostFolder = Join-Path $env:LOCALAPPDATA 'qb64rust\winvm'
$credentialFile = Join-Path $hostFolder 'credential.xml'
$readyLimitMinutes = 60

# Mesa's software OpenGL (llvmpipe), from the Windows builds of pal1000/mesa-dist-win, MIT license. The guest's only
# display is Hyper-V's synthetic adapter, whose OpenGL 1.1 cannot make a QB64 program's window (design.md,
# "Context", task 3.1). inside.ps1 puts these two DLLs beside each program.
$mesaVersion = '26.2.5'
$mesaUrl = "https://github.com/pal1000/mesa-dist-win/releases/download/$mesaVersion/mesa3d-$mesaVersion-release-msvc.7z"
$mesaSha256 = '997123ef4ce8896204ab912d5b54f19372c487ff9835bcefafa2fca23b8953c2'
$mesaFiles = 'opengl32.dll', 'libgallium_wgl.dll'

function Fail([string]$message) {
    Write-Host "create.ps1: $message"
    exit 2
}

# The two DLLs, downloaded once into %LOCALAPPDATA%\qb64rust\winvm\mesa\<version>\ and checked against the hash.
function Get-Mesa {
    $folder = Join-Path $hostFolder "mesa\$mesaVersion"
    $archive = Join-Path $folder "mesa3d-$mesaVersion-release-msvc.7z"
    New-Item -ItemType Directory -Force $folder | Out-Null
    if (-not (Test-Path -LiteralPath $archive)) {
        Write-Host "Downloading Mesa $mesaVersion (about 70 MB)..."
        Invoke-WebRequest -Uri $mesaUrl -OutFile $archive -UseBasicParsing
    }
    $hash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLower()
    if ($hash -ne $mesaSha256) {
        Remove-Item -LiteralPath $archive
        throw "the Mesa download has SHA-256 $hash, not the pinned $mesaSha256; it was deleted"
    }
    & "$env:SystemRoot\System32\tar.exe" -xf $archive -C $folder ($mesaFiles | ForEach-Object { "x64/$_" })
    if ($LASTEXITCODE -ne 0) { throw "tar could not unpack $archive (exit $LASTEXITCODE)" }
    Join-Path $folder 'x64'
}

$fromIso = $PSCmdlet.ParameterSetName -eq 'Iso'
if ($fromIso) {
    $principal = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    if (-not $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)) {
        Fail ('run this from an elevated PowerShell: writing the image to a new disk needs administrator rights. ' +
            'To make the VM again from the disk kept by an earlier run, use -FromBase, which does not.')
    }
    if (-not (Test-Path -LiteralPath $Iso -PathType Leaf)) {
        Fail "no file at the ISO path given: $Iso"
    }
}
Import-Module Hyper-V
if (-not (Get-VMSwitch -Name 'Default Switch' -ErrorAction SilentlyContinue)) {
    Fail "Hyper-V's 'Default Switch' is missing: the VM needs it for a minute, to activate Windows."
}

$diskFolder = Join-Path (Get-VMHost).VirtualHardDiskPath $name
$vhdx = Join-Path $diskFolder "$name.vhdx"
$base = Join-Path (Get-VMHost).VirtualHardDiskPath "$name-base\$name-base.vhdx"
if (Get-VM -Name $name -ErrorAction SilentlyContinue) {
    Fail ("a VM named $name already exists. Nothing was changed. To make it again, remove it first:`n" +
        "  Remove-VM -Name $name -Force`n" +
        "  Remove-Item -Recurse -Force `"$diskFolder`"")
}
if (Test-Path -LiteralPath $diskFolder) {
    Fail ("the disk folder of $name already exists, without its VM. Nothing was changed. Remove it first:`n" +
        "  Remove-Item -Recurse -Force `"$diskFolder`"")
}
if (-not $fromIso) {
    if (-not (Test-Path -LiteralPath $base)) {
        Fail "there is no base disk ($base): make the VM with -Iso, from an elevated PowerShell."
    }
    if (-not (Test-Path -LiteralPath $credentialFile)) {
        Fail ("the credential that goes with the base disk ($credentialFile) is missing: make the VM with -Iso, " +
            "from an elevated PowerShell.")
    }
}
# The VM's disk takes about 18 GB and the checkpoint's memory up to 4 GB; -Iso writes the base, about 12 GB, too
# (tools\winvm\README.md asks for 37 GB). The limits leave room for the disk to grow during setup.
$needGB = if ($fromIso) { 40 } else { 30 }
$freeGB = (Get-PSDrive ($diskFolder.Substring(0, 1))).Free / 1GB
if ($freeGB -lt $needGB) {
    Fail ("{0:N0} GB is free on the drive of Hyper-V's disk folder; making the VM needs about {1} GB." -f $freeGB, $needGB)
}

$started = Get-Date
$isoMounted = $false
$vhdMounted = $false
$vmMade = $false
try {
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

        # The disk: GPT with EFI, MSR and Windows partitions.
        New-Item -ItemType Directory -Force $diskFolder | Out-Null
        New-VHD -Path $vhdx -SizeBytes 64GB -Dynamic | Out-Null
        $disk = Mount-VHD -Path $vhdx -Passthru | Get-Disk
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

        Write-Host 'Writing the image to the disk (several minutes)...'
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
        Dismount-VHD -Path $vhdx
        $vhdMounted = $false
        Dismount-DiskImage -ImagePath (Resolve-Path -LiteralPath $Iso).Path | Out-Null
        $isoMounted = $false

        # The base: the disk before it ever boots, with the password of the credential written next.
        Write-Host 'Keeping a copy of the disk before its first boot, for -FromBase...'
        New-Item -ItemType Directory -Force (Split-Path $base) | Out-Null
        Copy-Item -LiteralPath $vhdx -Destination $base -Force
        New-Item -ItemType Directory -Force $hostFolder | Out-Null
        $credential | Export-Clixml -Path $credentialFile
    } else {
        $credential = Import-Clixml -Path $credentialFile
        Write-Host 'Copying the base disk...'
        New-Item -ItemType Directory -Force $diskFolder | Out-Null
        Copy-Item -LiteralPath $base -Destination $vhdx
    }

    # The VM (design D1, D3, D5).
    New-VM -Name $name -Generation 2 -MemoryStartupBytes 4GB -VHDPath $vhdx | Out-Null
    $vmMade = $true
    Get-VMNetworkAdapter -VMName $name | Remove-VMNetworkAdapter
    # 4 processors, not 2: with Mesa drawing on the CPU, 2 let mousemove_readback read 0, 0 in about half the runs
    # (design.md, "Context", task 3.2).
    Set-VM -Name $name -ProcessorCount 4 -StaticMemory -CheckpointType Standard -AutomaticCheckpointsEnabled $false `
        -AutomaticStartAction Nothing -AutomaticStopAction TurnOff
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

    # The evaluation is activated online, which starts its 90 days; unactivated, it reports no grace time left
    # from its first boot. The VM is online only for this, through Hyper-V's NAT "Default Switch", with Windows
    # Update stopped, and the adapter is removed before the checkpoint.
    Write-Host 'The first logon has finished. Activating Windows (the VM is online for about a minute)...'
    $session = New-PSSession -VMName $name -Credential $credential
    try {
        Invoke-Command -Session $session { Stop-Service wuauserv -Force; Set-Service wuauserv -StartupType Disabled }
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
    } finally {
        Remove-PSSession $session
    }
    if (-not $activated) {
        throw "Windows was not activated within 4 minutes: is the host online, and does Hyper-V's 'Default Switch' exist?"
    }

    Write-Host 'Activated. Letting the guest settle for a minute...'
    Start-Sleep -Seconds 60
    Checkpoint-VM -Name $name -SnapshotName ready
    Stop-VM -Name $name -TurnOff -Force
    Write-Host ("Made $name with its checkpoint 'ready' in {0:N0} minutes." -f ((Get-Date) - $started).TotalMinutes)
} catch {
    Write-Host "create.ps1: $($_.Exception.Message)"
    if ($vhdMounted) { Dismount-VHD -Path $vhdx -ErrorAction SilentlyContinue }
    if ($isoMounted) { Dismount-DiskImage -ImagePath (Resolve-Path -LiteralPath $Iso).Path -ErrorAction SilentlyContinue | Out-Null }
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
