# PowerShell profile

# --- Tool initialization ---

# Starship prompt
Invoke-Expression (&starship init powershell)

# Zoxide uses zz for navigation, leaving z for Zed.
if (Get-Command zoxide -ErrorAction SilentlyContinue) {
    zoxide init powershell --cmd zz | Out-String | Invoke-Expression
}

# Mado Terminal Navigator
if (Get-Command mado-term-navigator -ErrorAction SilentlyContinue) {
    mado-term-navigator init powershell --cmd mtn | Out-String | Invoke-Expression
}

# Jujutsu completions
if (Get-Command jj -ErrorAction SilentlyContinue) {
    Invoke-Expression (& { (jj util completion power-shell | Out-String) })
}

# --- Paths and navigation ---

$sandbox = "C:/SANDBOX/"

function desktop  { Set-Location $env:USERPROFILE/Desktop }
function doc      { Set-Location $env:USERPROFILE/Documents/DOUTORADO }
function sand     { Set-Location $sandbox }
function home     { Set-Location $env:USERPROFILE }
function local    { Set-Location $env:LOCALAPPDATA }
function roam     { Set-Location $env:APPDATA }
function roaming  { Set-Location $env:APPDATA }
function appdata  { Set-Location $env:APPDATA }
function localapp { Set-Location $env:LOCALAPPDATA }
function nvimpath { Set-Location $env:LOCALAPPDATA/nvim }
function cd_nvim  { Set-Location "~/AppData/Local/nvim" }

# Create a directory, then navigate with Zoxide.
function cdd {
    param ([string]$path)
    if (!(Test-Path $path)) { New-Item -ItemType Directory -Path $path -Force | Out-Null }
    # Set-Location $path
    zz $path
}

# --- File browsing and search ---

function pilot { C:\Users\daniel\AppData\Local\Voidstar\FilePilot\FPilot.exe $args }
function e     { pilot $args }
function ee    { explorer $args }
function ll    { eza -a $args }
function j     { mtn jump @args }

# FD wrapper with path separator fix
function fd  { fdfind --path-separator / $args }
function fdd { fdfind $args }

# Remove recursively and forcefully.
function rmf {
    param ([string]$arg)
    Remove-Item -Path $arg -Recurse -Force
}

function rm_nul { node -e "require('fs').unlinkSync('nul')" }

# --- Command lookup ---

# Open the folder containing a command.
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

# Find the source of a command.
function w {
    param ([string]$arg)
    (Get-Command $arg).Source
}

# --- Editors and project commands ---

function agr       { antigravity -r . }
function cr        { code -r . }
function zr        { zed -r . }
function dtodo     { code C:/Users/daniel/Documents/DOUTORADO/TODO }
function web_douto { . $env:USERPROFILE/Downloads/web_server_daniel-windows-amd64.exe }
function a         { . .venv/Scripts/activate.ps1 }
function lserver   { llama-server $args -ngl 99 --port 8033 }

function gmail {
    Set-Location C:/SANDBOX/REPOS/mado_mail
    task run
    # . "C:/SANDBOX/REPOS/mado_mail/dist/web 1.0.0.exe"
}

# Build system helpers
function setvs17 { cmake -G "Visual Studio 15 2017" -A x64 }
function setvs22 { cmake -G "Visual Studio 17 2022" -A x64 }

# SSH setup
function ssh_setup {
    Start-Service -Name sshd
    netsh advfirewall set allprofiles state off
}

# --- Git and Pi helpers ---

function gstatus { git status }
function gfetch  { git fetch }
function gd      { git diff $args }

function commit { pi --model gpt-6-luna --thinking xhigh -p "generate a commit message for the staged items with a meaninful message" }
function pii {
    pi --model gpt-6-luna --thinking xhigh @args
}

# --- Project search with FZF ---

function ExpandFolders {
    param ([string]$path, [int]$depth = 2, [string]$fd_params)
    $search = (fd $fd_params -t d -d $depth -a --base-directory $path) + $path
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
        (ExpandFolders "$env:USERPROFILE" -depth 4 -fd_params "-u"),
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
        if (Get-Module PSReadLine) {
            [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
        }
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

# --- PSReadLine keybindings ---

# Ctrl+R: FZF history search
Set-PSReadlineKeyHandler -Chord 'Ctrl+r' -ScriptBlock {
    $historyPath = [System.IO.Path]::Combine($env:APPDATA, 'Microsoft', 'Windows', 'PowerShell', 'PSReadLine', 'ConsoleHost_history.txt')
    $search = Get-Content -Path $historyPath | fzf --tac --no-sort --ansi
    if ($search) {
        [Microsoft.PowerShell.PSConsoleReadLine]::SetCursorPosition(0)
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($search)
    }
}

# Previous FZF project-jumper bindings
# Set-PSReadlineKeyHandler -Chord 'Ctrl+p' -ScriptBlock { FZF_go_to_projects }
# Set-PSReadlineKeyHandler -Chord 'Alt+p'  -ScriptBlock { FZF_go_to_projects }

# Ctrl+P: Mado Terminal Navigator goto
Set-PSReadLineKeyHandler -Chord 'Ctrl+p' -ScriptBlock {
    mtn goto
    [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
}

Set-PSReadLineKeyHandler -Chord 'Ctrl+d' -ScriptBlock {
    mtn sub
    [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
}

# Ctrl+E / Alt+E: FZF explorer with FilePilot
Set-PSReadLineKeyHandler -Key "Ctrl+e" -ScriptBlock { FZF_explorer }
Set-PSReadLineKeyHandler -Key "Alt+e"  -ScriptBlock { FZF_explorer }

# --- Aliases ---
Set-Alias b     bun
Set-Alias c     code
Set-Alias cdvim cd_nvim
Set-Alias d     docker
Set-Alias dc    docker-compose
Set-Alias g     git
Set-Alias gwt   git-wt
Set-Alias gfe   gfetch
Set-Alias gs    gstatus
Set-Alias m     mingw32-make
Set-Alias o     ollama
Set-Alias ob    obsidian
Set-Alias p     podman
Set-Alias t     task
Set-Alias v     nvim
Set-Alias z     zed
Set-Alias mex   mado-excalidraw

# --- Git worktree shell initialization ---
# Keep after aliases so the gwt alias is available.

if (Get-Command gwt -ErrorAction SilentlyContinue) { Invoke-Expression (& gwt config shell init powershell | Out-String) }
if (Get-Command git-wt -ErrorAction SilentlyContinue) { Invoke-Expression (& git-wt config shell init powershell | Out-String) }
