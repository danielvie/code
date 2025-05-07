# Install the OpenSSH Server:
- Go to Settings > Apps > Optional Features.
- Scroll down and look for OpenSSH Server. If it’s not installed:
- Click Add a feature.
- Search for OpenSSH Server.
- Select it and click Install.

# Start the OpenSSH Server Service:
- Open the Services application (type services.msc in the Start menu).
- Find OpenSSH SSH Server.
- Right-click and select Properties.
- Set the Startup type to Automatic.
- Click Start to start the service.

# Configure the Firewall:
- Open the Windows Defender Firewall settings.
- Allow inbound traffic on port 22 (default SSH port):
  - Go to Advanced settings > Inbound Rules.
  - Add a new rule to allow TCP connections on port 22.

# Check the Server:
- Open a terminal on another computer.
- Use the command: ssh <username>@<windows_ip_address>.

# Useful commands:

```powershell
Get-Service | rg -i ssh
```

```powershell
Start-Service -Name sshd
```

```powershell
Stop-Service -Name sshd
```

