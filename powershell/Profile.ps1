Invoke-Expression (&starship init powershell)
$env:STARSHIP_ASYNC = $fals

$sandbox  = "C:/SANDBOX/"
$sandcode = "C:/SANDBOX/CODE/"

function desktop
{
    Set-Location $env:USERPROFILE/Desktop
}

function doc
{
    Set-Location $env:USERPROFILE/Documents/DOUTORADO
}

function sand
{
    Set-Location $sandcode
}

function home
{
    Set-Location $env:USERPROFILE
}

function local
{
    Set-Location $env:LOCALAPPDATA
}

function roaming
{
    Set-Location $env:APPDATA
}

function localapp
{
    Set-Location $env:LOCALAPPDATA
}

function gmail
{
    Set-Location $sandbox/code.git/sandbox/go/go_gmail
    task
}

function nvimpath
{
    Set-Location $env:LOCALAPPDATA/nvim
}

function w
{
    param (
        [string]$arg
    )
    (Get-Command $arg).Source
}

function pilot
{
    C:\Users\daniel\AppData\Local\Voidstar\FilePilot\FPilot.exe $args
}

function ee
{
    explorer $args
}

function e
{
    pilot $args
}

function we
{
    param (
        [Parameter(Mandatory=$true)]
        [string]$CommandName
    )

    try
    {
        # Get the command object. The 'Get-Command' cmdlet resolves aliases and functions
        # to their underlying executable paths.
        $commandInfo = Get-Command -Name $CommandName -ErrorAction Stop

        # Get the full path of the command's source file.
        $commandPath = $commandInfo.Source

        # Split the path to get only the parent directory.
        # This is the folder containing the executable.
        $directoryPath = Split-Path -Path $commandPath -Parent

        # Use 'Start-Process' to open File Explorer with the directory path as the argument.
        Write-Host "Opening folder for '$CommandName': $directoryPath" -ForegroundColor Green
        Start-Process -FilePath "explorer.exe" -ArgumentList $directoryPath
    } catch
    {
        # Catch any errors, such as if the command does not exist.
        Write-Error "Could not find the command '$CommandName' or an error occurred."
        Write-Error $_.Exception.Message
    }
}



function rmf
{
    param (
        [string]$arg
    )

    Remove-Item -Path $arg -Recurse -Force
}

function setvs17
{
    cmake -G "Visual Studio 15 2017" -A x64
}

function setvs22
{
    cmake -G "Visual Studio 17 2022" -A x64
}

# go projects
function ExpandFolders
{
    param (
        [string]$path,
        [int]$depth = 2
    )

    $search = (fd -t d -d $depth -a --base-directory $path) + $path
    return $search
}

function GetProjects
{
    $sandbox = ExpandFolders $sandbox -depth 8

    $doutorado = "C:\Users\daniel\Documents\DOUTORADO"
    $doutorado = ExpandFolders $doutorado -depth 10

    $documents = "C:\Users\daniel\Documents"
    $documents = ExpandFolders $documents -depth 7

    $downloads = "$env:USERPROFILE\Downloads"
    $nvim = "$env:LOCALAPPDATA\nvim"
    
    $localapp = "$env:LOCALAPPDATA"
    $localapp = ExpandFolders $localapp -depth 4

    $user = "$env:USERPROFILE"
    $user = ExpandFolders $user -depth 4
    
    $program_files_x86 = ExpandFolders ${env:ProgramFiles(x86)} -depth 3
    $program_files_ = ExpandFolders $env:ProgramFiles -depth 3
    $program_files  = $program_files_x86 + $program_files_

    $projects = @(
        $alias,
        $sandbox,
        $doutorado,
        $nvim,
        $documents,
        $downloads,
        $localapp,
        $user,
        $program_files
    ) | ForEach-Object {
        # Handle both directory objects and string paths
        if ($_ -is [System.IO.DirectoryInfo])
        {
            $_.FullName
        } else
        {
            $_
        }
    } | Sort-Object -Unique
    return $projects

}

function FZF_go_to_projects
{
    $search = GetProjects | fzf --tac

    if ($search)
    {
        # Set-Location $search
        zz $search

        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        # [Microsoft.PowerShell.PSConsoleReadLine]::Insert($command)
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}

function FZF_open_project_with_vscode
{
    $search = $(GetProjects | fzf --tac)

    if ($search)
    {
        code $search
    }
}

function FZF_open_project_with_zed
{
    $search = $(GetProjects | fzf --tac)

    if ($search)
    {
        zed $search
    }
}

function FZF_explorer
{
    $search = GetProjects | fzf --tac

    if ($search)
    {
        # Invoke-Item -Path $search
        pilot $search
    }
}

# .. SET BEHAVIOR

# Import-Module PSFzf
#
# Set-PsFzfOption -PSReadlineChordProvider 'Ctrl+t'
# Set-PsFzfOption -PSReadlineChordReverseHistory 'Ctrl+r'
# Set-PSReadLineKeyHandler -Key Tab -ScriptBlock { Invoke-FzfTabCompletion }


Set-PSReadlineKeyHandler -Chord 'Ctrl+r' -ScriptBlock {
    $historyPath = [System.IO.Path]::Combine($env:APPDATA, 'Microsoft', 'Windows', 'PowerShell', 'PSReadLine', 'ConsoleHost_history.txt')
    $search = Get-Content -Path $historyPath | fzf --tac --no-sort --ansi

    if ($search)
    {
        [Microsoft.PowerShell.PSConsoleReadLine]::SetCursorPosition(0)
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($search)
    }
}

Set-PSReadlineKeyHandler -Chord 'Ctrl+p' -ScriptBlock {
    FZF_go_to_projects
}

Set-PSReadlineKeyHandler -Chord 'Alt+p' -ScriptBlock {
    FZF_go_to_projects
}

Set-PSReadlineKeyHandler -Chord 'Ctrl+o' -ScriptBlock {
    FZF_open_project_with_zed
}

Set-PSReadlineKeyHandler -Chord 'Ctrl+f' -ScriptBlock {
    $search = fd -t f -d 5 | fzf --tac --ansi --preview 'bat --color=always --style=numbers --line-range=:500 {}'
    if ($search)
    {
        $command = "v $search"
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($command)
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}

Set-PSReadLineKeyHandler -Chord 'Ctrl+d' -ScriptBlock {
    $search = fd -t d -d 8 | fzf --tac
    # $search = fzf --tac

    if ($search)
    {
        Set-Location $search
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}

Set-PSReadLineKeyHandler -Chord 'Ctrl+u' -ScriptBlock {
    $search = fd -t d -d 4 -u | fzf --tac

    if ($search)
    {
        Set-Location $search
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}

# Set-PSReadlineKeyHandler -Chord 'Ctrl+o' -ScriptBlock {
#     $search = fd -t d -d 5 | fzf --tac
#     if ($search) {
#         # $command = "cd ""$search"""
#         Set-Location $search
#         [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
#         # [Microsoft.PowerShell.PSConsoleReadLine]::Insert($command)
#         [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
#     }
# }

# open project with explorer
Set-PSReadLineKeyHandler -Key "Ctrl+e" -ScriptBlock {
    FZF_explorer
}

Set-PSReadLineKeyHandler -Key "Alt+e" -ScriptBlock {
    FZF_explorer
}

function gstatus
{
    git status
}

function gfetch
{
    git fetch
}

function cr_fun
{
    code -r .
}

function zr_fun
{
    zed -r .
}

function cd_nvim
{
    Set-Location "~/AppData/Local/nvim"
}

function fd
{ fdfind --path-separator / $args
}
function fdd
{ fdfind $args
}

function cdd
{
    param (
        [string]$path
    )
    mkdir -p $path
    Set-Location $path
}

function ssh_setup
{
    Start-Service -Name sshd # start ssh service
    netsh advfirewall set allprofiles state off # set firewall off
}

function gd
{
    git diff $args
}

# .. ALIASES

function dtodo
{
    code C:/Users/daniel/Documents/DOUTORADO/TODO
}

function web_douto
{
    . $env:USERPROFILE/Downloads/web_server_daniel-windows-amd64.exe
}

function a
{
    . .venv/Scripts/activate.ps1
}

function lserver
{
    llama-server $args -ngl 99 --port 8033
}

Set-Alias ag antigravity
Set-Alias b bun
Set-Alias c code
Set-Alias cdvim cd_nvim
Set-Alias cr cr_fun
Set-Alias d docker
Set-Alias dc docker-compose
Set-Alias g git
Set-Alias gfe gfetch
Set-Alias gs gstatus
Set-Alias j just
Set-Alias ll eza
Set-Alias m mingw32-make
Set-Alias o ollama
Set-Alias ob obsidian
Set-Alias p podman
Set-Alias t task
Set-Alias v nvim
Set-Alias z zed
Set-Alias zr zr_fun

Set-Alias goto FZF_go_to_projects

# set zoxide
& "$env:USERPROFILE\Documents\WindowsPowerShell\zoxide.ps1"

# set completion for jj
Invoke-Expression (& { (jj util completion power-shell | Out-String) })

