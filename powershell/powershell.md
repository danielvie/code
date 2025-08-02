# USEFUL commands

Allow execution of scripts. (NOTE:: options are: AllSigned, Bypass, Default, RemoteSigned, Restricted, Undefined, Unrestricted.)
```powershell
Set-ExecutionPolicy Unrestricted
```

Install recomendation
```powershell
Install-Module -Name PSReadLine -Scope CurrentUser -Force -SkipPublisherCheck
```

Install PSFzf module
```powershell
Install-Module PSFzf
```

Append Path
```powershell
$env:Path += ";C:\Your\New\Path"
```

# USEFUL bindings

open history
```powershell
Set-PSReadlineKeyHandler -Chord 'Ctrl+r' -ScriptBlock {
    $historyPath = [System.IO.Path]::Combine($env:APPDATA, 'Microsoft', 'Windows', 'PowerShell', 'PSReadLine', 'ConsoleHost_history.txt')
    $search = Get-Content -Path $historyPath | fzf --tac --no-sort --ansi

    if ($search) {
        [Microsoft.PowerShell.PSConsoleReadLine]::SetCursorPosition(0)
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($search)
    }
}
```

open directory
```powershell
Set-PSReadlineKeyHandler -Chord 'Ctrl+d' -ScriptBlock {
    $search = "cd ""$(fd -t d | fzf --tac --no-sort --ansi)"""
    if ($search) {
        $command = "cd ""$search"""
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($command)
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}
```

add addr to PATH
```powershell
$env:PATH += ";<newpath>"
```


open in nvim
```powershell
Set-PSReadlineKeyHandler -Chord 'Ctrl+f' -ScriptBlock {
    $search = fd | fzf --tac --no-sort --ansi --preview 'bat --color=always --style=numbers --line-range=:500 {}'
    if ($search) {
        $command = "v $search"
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($command)
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}
```

open project folders
```powershell
Set-PSReadLineKeyHandler -Key "Ctrl+p" -ScriptBlock {
    $sand = fd -t d -d 1 -a --base-directory "C:\SANDBOX"
    $project = @(
        $sand,
        "C:\SANDBOX",
        "$env:LOCALAPPDATA",
        "$env:LOCALAPPDATA\nvim"
    ) | fzf --tac 

    if ($project) {
        $command = "cd $project"
        [Microsoft.PowerShell.PSConsoleReadLine]::RevertLine()
        [Microsoft.PowerShell.PSConsoleReadLine]::Insert($command)
        [Microsoft.PowerShell.PSConsoleReadLine]::AcceptLine()
    }
}
```
