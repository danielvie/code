# ==============================================================================
# POWERSHELL PROFILE
# ==============================================================================

$__profileTimingEnabled = $false
$__profileTimer = $null

if ($__profileTimingEnabled) {
    $__profileTimer = [System.Diagnostics.Stopwatch]::StartNew()
}

function Write-ProfileTime {
    param ([string]$Label)

    if (-not $__profileTimingEnabled) {
        return
    }

    Write-Host ("[profile] {0,6} ms  {1}" -f $__profileTimer.ElapsedMilliseconds, $Label) -ForegroundColor DarkGray
}

Write-ProfileTime "profile start"

# ------------------------------------------------------------------------------
# 1. TOOL INITIALIZATIONS
# ------------------------------------------------------------------------------
Write-ProfileTime "start tool initializations"

# Starship Prompt
Write-ProfileTime "start starship init"
if (Get-Command starship -ErrorAction SilentlyContinue) {
    Invoke-Expression (& starship init powershell)
}
Write-ProfileTime "end starship init"

# Zoxide (using zz for navigation, leaving z for zed)
Write-ProfileTime "start zoxide init"
if (Get-Command zoxide -ErrorAction SilentlyContinue) {
    zoxide init powershell --cmd zz | Out-String | Invoke-Expression
}
Write-ProfileTime "end zoxide init"

# jj (Jujutsu) Completions
Write-ProfileTime "start jj completion init"
if (Get-Command jj -ErrorAction SilentlyContinue) {
    Invoke-Expression (& { (jj util completion power-shell | Out-String) })
}
Write-ProfileTime "end jj completion init"

Write-ProfileTime "end tool initializations"

# ------------------------------------------------------------------------------
# 2. CORE PROFILE
# ------------------------------------------------------------------------------
Write-ProfileTime "start core"

$sandbox = "C:/SANDBOX/"

# Navigation shortcuts
function desktop  { Set-Location $env:USERPROFILE/Desktop }
function doc      { Set-Location $env:USERPROFILE/Documents/DOUTORADO }
function sand     { Set-Location $sandbox }
function home     { Set-Location $env:USERPROFILE }
function local    { Set-Location $env:LOCALAPPDATA }
function roaming  { Set-Location $env:APPDATA }
function localapp { Set-Location $env:LOCALAPPDATA }
function nvimpath { Set-Location $env:LOCALAPPDATA/nvim }

function gmail {
    Set-Location $sandbox/code.git/sandbox/go/go_gmail
    task
}

# Utility functions
function pilot { & $env:LOCALAPPDATA/Voidstar/FilePilot/FPilot.exe $args }
function e     { pilot $args }
function ee    { explorer $args }

function we {
    param ([Parameter(Mandatory = $true)][string]$CommandName)
    try {
        $commandPath = (Get-Command $CommandName -ErrorAction Stop).Source
        $directoryPath = Split-Path -Path $commandPath -Parent
        Write-Host "Opening folder for '$CommandName': $directoryPath" -ForegroundColor Green
        Start-Process -FilePath "explorer.exe" -ArgumentList $directoryPath
    } catch {
        Write-Error "Could not find the command '$CommandName' or an error occurred."
    }
}

function w {
    param ([string]$arg)
    (Get-Command $arg).Source
}

function rmf {
    param ([string]$arg)
    Remove-Item -Path $arg -Recurse -Force
}

function cdd {
    param ([string]$path)
    if (!(Test-Path $path)) { New-Item -ItemType Directory -Path $path -Force | Out-Null }
    Set-Location $path
}

function fd  { fdfind --path-separator / $args }
function fdd { fdfind $args }

function setvs17 { cmake -G "Visual Studio 15 2017" -A x64 }
function setvs22 { cmake -G "Visual Studio 17 2022" -A x64 }

function ssh_setup {
    Start-Service -Name sshd
    netsh advfirewall set allprofiles state off
}

function get_skills {
    $sourcePath = "C:\Users\ae904f\.agents\skills\*"
    $destinationPath = ".agent\skills"

    New-Item `
        -ItemType Directory `
        -Force `
        -Path $destinationPath | Out-Null

    Copy-Item -Path $sourcePath -Destination $destinationPath -Recurse -Force
}

# Git helpers
function gstatus { git status }
function gfetch  { git fetch }
function gd      { git diff $args }

# General aliases
Set-Alias b     bun
Set-Alias c     code
Set-Alias d     docker
Set-Alias dc    docker-compose
Set-Alias g     git
Set-Alias gfe   gfetch
Set-Alias gs    gstatus
Set-Alias ll    eza
Set-Alias m     mingw32-make
Set-Alias o     ollama
Set-Alias ob    obsidian
Set-Alias p     podman
Set-Alias t     task
Set-Alias v     nvim
Set-Alias z     zed

# Misc shortcuts
function web_douto { . $env:USERPROFILE/Downloads/web_server_daniel-windows-amd64.exe }
function a         { . .venv/Scripts/activate.ps1 }
function lserver   { llama-server $args -ngl 99 --port 8033 }
function cr        { code -r . }
function zr        { zed -r . }

Write-ProfileTime "end core"

# ------------------------------------------------------------------------------
# 3. PROJECT MANAGEMENT & FZF HELPERS
# ------------------------------------------------------------------------------
Write-ProfileTime "start fzf"

function ExpandFolders {
    param ([string]$path, [int]$depth = 2, [string]$fd_param)
    $search = (fd -t d $fd_param -d $depth -a --base-directory $path) + $path
    return $search
}

function GetProjects {
    $projects = @(
        (ExpandFolders $sandbox -depth 8),
        (ExpandFolders "$env:USERPROFILE\Documents\DOUTORADO" -depth 10),
        (ExpandFolders "$env:USERPROFILE\Documents" -depth 7),
        "$env:USERPROFILE\Downloads",
        "$env:LOCALAPPDATA\nvim",
        (ExpandFolders "$env:LOCALAPPDATA" -depth 4),
        (ExpandFolders "$env:USERPROFILE" -depth 4 -fd_param "-u"),
        (ExpandFolders ${env:ProgramFiles(x86)} -depth 3),
        (ExpandFolders $env:ProgramFiles -depth 3)
    ) | ForEach-Object {
        if ($_ -is [System.IO.DirectoryInfo]) { $_.FullName } else { $_ }
    } | Sort-Object -Unique
    return $projects
}

function FZF_go_to_projects {
    $search = GetProjects | fzf --tac
    if ($search) {
        zz $search
        [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
    }
}

function FZF_open_project_with_vscode {
    $search = GetProjects | fzf --tac
    if ($search) { code $search }
}

function FZF_open_project_with_zed {
    $search = GetProjects | fzf --tac
    if ($search) { zed $search }
}

function FZF_explorer {
    $search = GetProjects | fzf --tac
    if ($search) { pilot $search }
}

# CTRL+R: FZF History Search
Set-PSReadLineKeyHandler -Chord 'Ctrl+r' -ScriptBlock {
    $historyPath = [System.IO.Path]::Combine($env:APPDATA, 'Microsoft', 'Windows', 'PowerShell', 'PSReadLine', 'ConsoleHost_history.txt')
    $search = Get-Content -Path $historyPath | fzf --tac --no-sort --ansi
    if ($search) {
        [Microsoft.PowerShell.PSConsoleReadLine]::SetCursorPosition(0)
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($search)
    }
}

# CTRL+P / ALT+P: FZF Project Jumper
Set-PSReadlineKeyHandler -Chord 'Ctrl+p' -ScriptBlock { FZF_go_to_projects }
Set-PSReadlineKeyHandler -Chord 'Alt+p'  -ScriptBlock { FZF_go_to_projects }

# CTRL+O: FZF Open Project in Zed
Set-PSReadlineKeyHandler -Chord 'Ctrl+o' -ScriptBlock { FZF_open_project_with_zed }

# CTRL+F: FZF File Search with Preview
Set-PSReadlineKeyHandler -Chord 'Ctrl+f' -ScriptBlock {
    $search = fd -t f -d 5 | fzf --tac --ansi --preview 'bat --color=always --style=numbers --line-range=:500 {}'
    if ($search) {
        zed $search
    }
}

# CTRL+D: FZF Directory Search
Set-PSReadLineKeyHandler -Chord 'Ctrl+d' -ScriptBlock {
    $search = fd -t d -d 8 | fzf --tac
    if ($search) {
        Set-Location $search
        [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
    }
}

# CTRL+E / ALT+E: FZF Explorer (FilePilot)
Set-PSReadLineKeyHandler -Key "Ctrl+e" -ScriptBlock { FZF_explorer }
Set-PSReadLineKeyHandler -Key "Alt+e"  -ScriptBlock { FZF_explorer }

Set-Alias goto FZF_go_to_projects

Write-ProfileTime "end fzf"

# Required tools:
# 
# starship - prompt - https://starship.rs/
# fd - file finder - https://github.com/sharkdp/fd
# rg - text search - https://github.com/BurntSushi/ripgrep
# fzf - fuzzy finder - https://github.com/junegunn/fzf
# zoxide - smarter cd - https://github.com/ajeetdsouza/zoxide
# bat - syntax-highlighted cat - https://github.com/sharkdp/bat
# eza - modern ls - https://github.com/eza-community/eza
# fpilot - file explorer launcher - https://voidstar.tech/filepilot/
# taskfile - task runner - https://taskfile.dev/
