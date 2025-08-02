Invoke-Expression (&starship init powershell)

$sandbox  = "C:\SANDBOX\"
$sandcode = "C:\SANDBOX\CODE\"

function desktop {
    Set-Location $env:USERPROFILE/Desktop
}

function doc {
    Set-Location $env:USERPROFILE/Documents/DOUTORADO
}

function sand {
    Set-Location $sandcode
}

function home {
    Set-Location $env:USERPROFILE
}

function localapp {
    Set-Location $env:LOCALAPPDATA
}

function gmail {
    Set-Location $sandbox/code.git/go/go_gmail
    task
}

function nvimpath {
    Set-Location $env:LOCALAPPDATA/nvim
}

function w {
    param (
        [string]$arg
    )
    Get-Command $arg
}

function e {
    param (
        [string]$arg
    )
    explorer $arg
}

function we {
    param (
        [Parameter(Mandatory=$true)]
        [string]$CommandName
    )

    try {
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
    }
    catch {
        # Catch any errors, such as if the command does not exist.
        Write-Error "Could not find the command '$CommandName' or an error occurred."
        Write-Error $_.Exception.Message
    }
}

function rmf {
    param (
        [string]$arg
    )
    
    Remove-Item -Path $arg -Recurse -Force
}

function setvs17 {
    cmake -G "Visual Studio 15 2017" -A x64
}

function setvs22 {
    cmake -G "Visual Studio 17 2022" -A x64
}

# go projects 
function ExpandFolders {
    param (
        [string]$path,
        [int]$depth = 2
    )
    
    $search = (fd -t d -d $depth -a --base-directory $path) + $path
    return $search
}

function GetProjects {
    $sandbox = ExpandFolders $sandbox -depth 4

    $doutorado = "C:\Users\daniel\Documents\DOUTORADO"
    $doutorado = ExpandFolders $doutorado -depth 1
    
    $documents = "C:\Users\daniel\Documents"
    $documents = ExpandFolders $documents -depth 1

    $arduino = "$documents\Arduino"

    $downloads = "$env:USERPROFILE\Downloads"
    
    $nvim = "$env:LOCALAPPDATA\nvim"

    $alias = "C:\SANDBOX\ALIAS\"

    $projects = @(
        $alias,
        $sandbox,
        $doutorado,
        $nvim,
        $documents,
        $downloads,
        $arduino,
        ${env:ProgramFiles(x86)},
        $env:ProgramFiles
    ) | Sort-Object
    
    return $projects
}

function GoToProjects {
    
    $projects = GetProjects
    $search = $projects | fzf --tac

    if ($search) {
        # Set-Location $search
        z $search
    
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        # [Microsoft.PowerShell.PSConsoleReadLine]::Insert($command)
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}

Set-PSReadlineKeyHandler -Chord 'Ctrl+o' -ScriptBlock {
    
    $search = fd -t d -d 5 | fzf --tac
    if ($search) {
        # $command = "cd ""$search"""
        Set-Location $search
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        # [Microsoft.PowerShell.PSConsoleReadLine]::Insert($command)
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}

Set-PSReadLineKeyHandler -Chord 'Ctrl+d' -ScriptBlock {
    $search = fd -t d -d 4 | fzf --tac
    
    if ($search) {
        Set-Location $search
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}

Set-PSReadlineKeyHandler -Chord 'Ctrl+f' -ScriptBlock {
    $search = fd -t f -d 5 | fzf --tac --ansi --preview 'bat --color=always --style=numbers --line-range=:500 {}'
    if ($search) {
        $command = "v $search"
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($command)
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}

Set-PSReadlineKeyHandler -Chord 'Ctrl+p' -ScriptBlock {
    GoToProjects
}

Set-PSReadlineKeyHandler -Chord 'Ctrl+r' -ScriptBlock {
    $historyPath = [System.IO.Path]::Combine($env:APPDATA, 'Microsoft', 'Windows', 'PowerShell', 'PSReadLine', 'ConsoleHost_history.txt')
    $search = Get-Content -Path $historyPath | Select-Object -Unique | fzf --tac 

    if ($search) {
        [Microsoft.PowerShell.PSConsoleReadLine]::SetCursorPosition(0)
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($search)
        # [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}



# open project with explorer 
Set-PSReadLineKeyHandler -Key "Ctrl+e" -ScriptBlock {
    $search = GetProjects | fzf --tac
    
    if ($search) {
        Invoke-Item -Path $search
    }
}

function gstatus {
    git status
}

function gfetch {
    git fetch
}

function cr_fun {
    code -r .
}

function mkdir_cd {
    param (
        [string]$arg
    )
    
    mkdir -p $arg
    Set-Location $arg
}

function cd_nvim {
    Set-Location "C:/Users/daniel/AppData/Local/nvim"
}

function fd { fdfind --path-separator / $args }

Set-Alias cdd mkdir_cd
Set-Alias ll eza
Set-Alias t task
Set-Alias m mingw32-make
Set-Alias j just
Set-Alias c code
Set-Alias cr cr_fun
Set-Alias g git
Set-Alias v nvim
Set-Alias d docker
Set-Alias gs gstatus
Set-Alias gfe gfetch

Set-Alias cdvim cd_nvim

& "$env:USERPROFILE\Documents\WindowsPowerShell\zoxide.ps1"

Invoke-Expression (& { (jj util completion power-shell | Out-String) })
