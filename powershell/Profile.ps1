# ==============================================================================
# POWERSHELL PROFILE
# ==============================================================================

# ------------------------------------------------------------------------------
# 1. TOOL INITIALIZATIONS
# ------------------------------------------------------------------------------

# Starship Prompt
Invoke-Expression (&starship init powershell)

# Zoxide (using zz for navigation, leaving z for zed)
if (Get-Command zoxide -ErrorAction SilentlyContinue) {
    zoxide init powershell --cmd zz | Out-String | Invoke-Expression
}

# jj (Jujutsu) Completions
if (Get-Command jj -ErrorAction SilentlyContinue) {
    Invoke-Expression (& { (jj util completion power-shell | Out-String) })
}

# ------------------------------------------------------------------------------
# 2. ENVIRONMENT VARIABLES & PATHS
# ------------------------------------------------------------------------------

$sandbox  = "C:/SANDBOX/"

# ------------------------------------------------------------------------------
# 3. NAVIGATION FUNCTIONS (SHORTCUTS)
# ------------------------------------------------------------------------------

function desktop  { Set-Location $env:USERPROFILE/Desktop }
function doc      { Set-Location $env:USERPROFILE/Documents/DOUTORADO }
function sand     { Set-Location $sandbox }
function home     { Set-Location $env:USERPROFILE }
function local    { Set-Location $env:LOCALAPPDATA }
function roaming  { Set-Location $env:APPDATA }
function localapp { Set-Location $env:LOCALAPPDATA }
function nvimpath { Set-Location $env:LOCALAPPDATA/nvim }
function cd_nvim  { Set-Location "~/AppData/Local/nvim" }

function gmail {
    Set-Location $sandbox/code.git/sandbox/go/go_gmail
    task
}

# ------------------------------------------------------------------------------
# 4. UTILITY FUNCTIONS
# ------------------------------------------------------------------------------

# File exploration & search
function pilot { C:\Users\daniel\AppData\Local\Voidstar\FilePilot\FPilot.exe $args }
function e     { pilot $args }
function ee    { explorer $args }

# Open folder containing the command
function we {
    param ([Parameter(Mandatory=$true)][string]$CommandName)
    try {
        $commandPath = (Get-Command $CommandName -ErrorAction Stop).Source
        $directoryPath = Split-Path -Path $commandPath -Parent
        Write-Host "Opening folder for '$CommandName': $directoryPath" -ForegroundColor Green
        Start-Process -FilePath "explorer.exe" -ArgumentList $directoryPath
    } catch {
        Write-Error "Could not find the command '$CommandName' or an error occurred."
    }
}

# Find source of a command
function w {
    param ([string]$arg)
    (Get-Command $arg).Source
}

# Remove recursive & forced
function rmf {
    param ([string]$arg)
    Remove-Item -Path $arg -Recurse -Force
}

# Quick mkdir + cd
function cdd {
    param ([string]$path)
    if (!(Test-Path $path)) { New-Item -ItemType Directory -Path $path -Force | Out-Null }
    Set-Location $path
}

# FD wrapper with path separator fix
function fd  { fdfind --path-separator / $args }
function fdd { fdfind $args }

# Build system helpers
function setvs17 { cmake -G "Visual Studio 15 2017" -A x64 }
function setvs22 { cmake -G "Visual Studio 17 2022" -A x64 }

# SSH setup
function ssh_setup {
    Start-Service -Name sshd
    netsh advfirewall set allprofiles state off
}

# ------------------------------------------------------------------------------
# 5. PROJECT MANAGEMENT & FZF HELPERS
# ------------------------------------------------------------------------------

function ExpandFolders {
    param ([string]$path, [int]$depth = 2)
    $search = (fd -t d -d $depth -a --base-directory $path) + $path
    return $search
}

function GetProjects {
    $projects = @(
        (ExpandFolders $sandbox -depth 8),
        (ExpandFolders "C:\Users\daniel\Documents\DOUTORADO" -depth 10),
        (ExpandFolders "C:\Users\daniel\Documents" -depth 7),
        "$env:USERPROFILE\Downloads",
        "$env:LOCALAPPDATA\nvim",
        (ExpandFolders "$env:LOCALAPPDATA" -depth 4),
        (ExpandFolders "$env:USERPROFILE" -depth 4),
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

# ------------------------------------------------------------------------------
# 6. KEY HANDLERS (PSReadLine)
# ------------------------------------------------------------------------------

# CTRL+R: FZF History Search
Set-PSReadlineKeyHandler -Chord 'Ctrl+r' -ScriptBlock {
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
        nvim $search
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

# CTRL+U: FZF Hidden Directory Search
Set-PSReadLineKeyHandler -Chord 'Ctrl+u' -ScriptBlock {
    $search = fd -t d -d 4 -u | fzf --tac
    if ($search) {
        Set-Location $search
        [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
    }
}

# CTRL+E / ALT+E: FZF Explorer (FilePilot)
Set-PSReadLineKeyHandler -Key "Ctrl+e" -ScriptBlock { FZF_explorer }
Set-PSReadLineKeyHandler -Key "Alt+e"  -ScriptBlock { FZF_explorer }

# ------------------------------------------------------------------------------
# 7. GIT HELPERS
# ------------------------------------------------------------------------------

function gstatus { git status }
function gfetch  { git fetch }
function gd      { git diff $args }

# ------------------------------------------------------------------------------
# 8. GENERAL ALIASES
# ------------------------------------------------------------------------------

Set-Alias ag    antigravity
Set-Alias b     bun
Set-Alias c     code
Set-Alias cdvim cd_nvim
Set-Alias cr    cr_fun
Set-Alias d     docker
Set-Alias dc    docker-compose
Set-Alias g     git
Set-Alias gfe   gfetch
Set-Alias gs    gstatus
Set-Alias j     just
Set-Alias ll    eza
Set-Alias m     mingw32-make
Set-Alias o     ollama
Set-Alias ob    obsidian
Set-Alias p     podman
Set-Alias t     task
Set-Alias v     nvim
Set-Alias z     zed
Set-Alias zr    zr_fun

Set-Alias goto  FZF_go_to_projects

# Misc shortcuts
function dtodo     { code C:/Users/daniel/Documents/DOUTORADO/TODO }
function web_douto { . $env:USERPROFILE/Downloads/web_server_daniel-windows-amd64.exe }
function a         { . .venv/Scripts/activate.ps1 }
function lserver   { llama-server $args -ngl 99 --port 8033 }
function cr_fun    { code -r . }
function zr_fun    { zed -r . }
