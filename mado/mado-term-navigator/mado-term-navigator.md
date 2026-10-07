```powershell
if (Get-Command mado-term-navigator -ErrorAction SilentlyContinue) {
    mado-term-navigator init powershell --cmd mtn | Out-String | Invoke-Expression
}
```

```powershell
Set-PSReadLineKeyHandler -Chord 'Ctrl+p' -ScriptBlock {
    mtn goto
    [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
}

Set-PSReadLineKeyHandler -Chord 'Ctrl+d' -ScriptBlock {
    mtn sub
    [Microsoft.PowerShell.PSConsoleReadLine]::InvokePrompt()
}
```
