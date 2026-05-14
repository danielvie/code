# reset network

```cmd
netsh winsock reset
netsh int ip reset
ipconfig /release
ipconfig /renew
ipconfig /flushdns
```


# Start SSH:

*check ssh service*
```powershell
Get-Service | rg -i ssh
```

*start ssh service*
```powershell
Start-Service -Name sshd
```

*stop ssh service*
```powershell
Stop-Service -Name sshd
```

# Firewall

## turn on / off firewall

```powershell
netsh advfirewall set allprofiles state on
```

```powershell
netsh advfirewall set allprofiles state off
```
