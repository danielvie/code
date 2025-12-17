https://github.com/junegunn/fzf

(SRES... ui/native/osstools/fzf/0.64.0/)

```powershell
function expandFolders {
    param (
        [string]$path,
        [int]$depth = 2
    )
    
    $search = (fd -t d -d $depth -a --base-directory $path) + $path
    return $search
}

function getProjects {
    $sand = expandFolders "C:\SANDBOX" -depth 6
    $documents = expandFolders "$env:USERPROFILE\Documents" -depth 6
    $obsidian = expandFolders "C:\SANDBOX\NOTES\obsidian" -depth 6
    $user = expandFoldersU $env:USERPROFILE -depth 2

    $projects = @(
        $sand,
        $documents,
        $user,
        "C:\Program Files",
        "C:\Program Files (x86)",
        $genericAirplane,
        "$env:LOCALAPPDATA",
        "$env:LOCALAPPDATA\nvim"
    ) | ForEach-Object { 
        # Handle both directory objects and string paths
        if ($_ -is [System.IO.DirectoryInfo]) {
            $_.FullName
        } else {
            $_
        }
    } | Sort-Object -Unique
    
    return $projects
}


function fzf_projects {
    $search = getProjects | fzf --tac 
    
    if ($search) {
        # Set-Location $search
        z $search
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}

function fzf_explorer {
    $search = getProjects | fzf --tac 
    
    if ($search) {
        Invoke-Item -Path $search
    }
}

function fzf_child {
    $search = fd -t d -d 4 -u | fzf --tac

    if ($search) {
        Set-Location $search
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}


Set-PSReadLineKeyHandler -Key "Ctrl+d" -ScriptBlock {
    fzf_child
}

Set-PSReadLineKeyHandler -Key "Ctrl+e" -ScriptBlock {
    fzf_explorer
}

Set-PSReadLineKeyHandler -Key "Ctrl+p" -ScriptBlock {
    fzf_projects
}
```
